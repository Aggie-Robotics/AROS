use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Display;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::{Relaxed, SeqCst};
use core::time::Duration;

use ansi_rgb::*;
use rgb::RGB8;

use crate::*;
use crate::robot::Robot;
use crate::sync::lock::Mutex;

extern "Rust" {
    pub fn get_tests(robot: &'static Robot) -> Vec<TestItem>;
}

pub fn assert(val: bool, on_error: String) -> TestResult {
    if val {
        Ok(())
    }
    else{
        Err(on_error)
    }
}

pub type TestResult = Result<(), String>;

const HEADER_COLOR: RGB8 = orange();
const NAME_COLOR: RGB8 = cyan();
const PASSED_COLOR: RGB8 = green();
const FAILED_COLOR: RGB8 = red();

pub trait TestFunction: FnOnce() -> TestResult{}
impl<T> TestFunction for T where T: FnOnce() -> TestResult{}

pub trait ParallelTestFunction: 'static + TestFunction + Send{}
impl<T> ParallelTestFunction for T where T: 'static + TestFunction + Send{}

pub enum TestType{
    Sequential(Box<dyn TestFunction>),
    Parallel(Box<dyn ParallelTestFunction>, Duration),
}

pub struct TestItem{
    name: String,
    test_type: TestType,
}
impl TestItem{
    pub fn new(name: String, test_type: TestType) -> Self{
        Self{
            name,
            test_type,
        }
    }
}

pub fn test_runner(tests: Vec<TestItem>){
    let start = system_time();

    let header = Arc::new("[aros testing]".fg(HEADER_COLOR));

    let tests_size = tests.len();
    console_print(&format!("{} running {} tests...\n", &header, tests_size));

    let mut sequentials = Vec::with_capacity(tests.len());
    let mut parallels = Vec::with_capacity(tests.len());

    for test in tests{
        match test.test_type {
            TestType::Sequential(function) => sequentials.push((test.name, function)),
            TestType::Parallel(function, timeout) => parallels.push((test.name, function, timeout)),
        }
    }

    let passed_count = Arc::new(AtomicUsize::new(0));
    let failed_count = Arc::new(AtomicUsize::new(0));

    run_sequential_tests(&header, sequentials, &passed_count, &failed_count);
    run_parallel_tests(&header, parallels, &passed_count, &failed_count);

    let end = system_time();

    let passed_count = passed_count.load(Relaxed);
    let failed_count = failed_count.load(Relaxed);
    console_print(&format!(
        "{} Results: {}. {} passed, {} failed. Completed in {}ms\n",
        &header,
        if failed_count > 0 {
            "FAILED".fg(FAILED_COLOR)
        }
        else{
            "PASSED".fg(PASSED_COLOR)
        },
        passed_count,
        failed_count,
        (end - start).as_millis()
    ));
}
fn run_sequential_tests(header: &Arc<WithForeground<&'static str>>, sequentials: Vec<(String, Box<dyn TestFunction>)>, passed_count: &Arc<AtomicUsize>, failed_count: &Arc<AtomicUsize>){
    let sequentials_size = sequentials.len();
    console_print(&format!("{} running {} sequential tests...\n", &header, sequentials_size));
    let sequentials_start = system_time();
    for (name, function) in sequentials {
        console_print(&format!("{} \"{}\" starting sequentially...\n", &header, (&name).fg(NAME_COLOR)));

        let (result, time) = run_test(function);
        process_results(result, name, &header, &passed_count, &failed_count, time);
    }
    let sequentials_end = system_time();
    console_print(&format!("{} ran {} sequential tests in {}ms\n", &header, sequentials_size, (sequentials_end - sequentials_start).as_millis()));
}
fn run_parallel_tests(header: &Arc<WithForeground<&'static str>>, parallels: Vec<(String, Box<dyn ParallelTestFunction>, Duration)>, passed_count: &Arc<AtomicUsize>, failed_count: &Arc<AtomicUsize>){
    let parallels_size = parallels.len();
    console_print(&format!("{} running {} parallel tests...\n", &header, parallels_size));

    let parallels_start = system_time();
    let mut mutex_vec = Vec::with_capacity(parallels_size);
    for (name, function, timeout) in parallels{
        console_print(&format!("{} \"{}\" starting in parallel...\n", &header, (&name).fg(NAME_COLOR)));

        let header_clone = header.clone();
        let passed_count_clone = passed_count.clone();
        let failed_count_clone = failed_count.clone();
        //Holds true if finished
        let mutex = Arc::new(Mutex::new(false));
        let should_process = Arc::new(Mutex::new(true));
        mutex_vec.push((mutex.clone(), system_time(), timeout, should_process.clone(), name.clone()));
        Task::new(
            None,
            None,
            &format!("test_{}", &name),
            move |_|{
                let mut guard = mutex.lock();
                let (result, time) = run_test(function);
                if *should_process.lock() {
                    process_results(result, name, &header_clone, &passed_count_clone, &failed_count_clone, time);
                }
                *guard = true;
            },
            ()
        );

        Task::delay_yield();
    }
    for (mutex, start, timeout, should_process, name) in &mutex_vec{
        loop {
            match mutex.try_lock() {
                None => if *start + *timeout < system_time(){
                    if let Some(mut guard) = should_process.try_lock(){
                        *guard = false;
                        failed_count.fetch_add(1, SeqCst);
                        console_print(&format!("{} {} \"{}\" in {}ms, timed out\n", header, "[FAILED]".fg(FAILED_COLOR), name.fg(NAME_COLOR), (system_time() - *start).as_millis()));
                    }
                    break
                }
                else{
                    Task::delay_yield();
                },
                Some(guard) => if *guard {
                    break;
                }
                else {
                    Task::delay_yield();
                },
            }
        }
    }
    let parallels_end = system_time();
    console_print(&format!("{} ran {} parallel tests in {}ms\n", &header, parallels_size, (parallels_end - parallels_start).as_millis()));
}
fn run_test(function: impl TestFunction) -> (TestResult, Duration){
    let start = system_time();
    let out = function();
    let end = system_time();
    (out, end - start)
}
fn process_results(result: TestResult, name: String, header: &impl Display, passed_count: &AtomicUsize, failed_count: &AtomicUsize, time: Duration){
    match result {
        Ok(_) => {
            console_print(&format!("{} {} \"{}\" in {}ms\n", header, "[PASSED]".fg(PASSED_COLOR), name.fg(NAME_COLOR), time.as_millis()));
            passed_count.fetch_add(1, SeqCst);
        },
        Err(error) => {
            console_print(&format!("{} {} \"{}\" in {}ms, error message: {}\n", header, "[FAILED]".fg(FAILED_COLOR), name.fg(NAME_COLOR), time.as_millis(), error.fg(red())));
            failed_count.fetch_add(1, SeqCst);
        },
    }
}
