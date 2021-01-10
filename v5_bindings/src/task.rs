use alloc::boxed::Box;
use alloc::string::String;
use core::time::Duration;

use cstr_core::CStr;
use cty::*;

use crate::{NotifyAction, State};
use crate::raw::pros::rtos::*;
use crate::raw::str_to_char_ptr;

pub trait TaskArgument: 'static + Send{}
impl<T> TaskArgument for T where T: 'static + Send{}
pub trait TaskFunction<T>: 'static + FnOnce(T) + Send{}
impl<T, U> TaskFunction<T> for U where T: TaskArgument, U: 'static + FnOnce(T) + Send{}

pub struct Task{
    task: task_t,
}
impl Task{
    pub fn new<F, T>(priority: Option<u32>, stack_depth: Option<u16>, name: &str, function: F, arg: T) -> Self
        where F: TaskFunction<T>,
              T: TaskArgument{
        let task_arg = Box::new(TaskArg{ function, arg });
        let parameters = (Box::leak(task_arg) as *mut TaskArg<F, T>) as *mut c_void;
        unsafe{Self{
            task: task_create(
                task_function::<F, T>,
                parameters,
                match priority{
                    None => TASK_PRIORITY_DEFAULT,
                    Some(priority) => priority,
                },
                match stack_depth{
                    None => TASK_STACK_DEPTH_DEFAULT,
                    Some(stack_depth) => stack_depth,
                },
                str_to_char_ptr(name).as_ptr()),
        }}
    }

    pub fn delay(duration: Duration){
        unsafe {task_delay(duration.as_millis() as uint32_t)}
    }
    pub fn delay_until(initial_time: Duration, delta: Duration){
        unsafe {task_delay_until(&mut (initial_time.as_millis() as uint32_t) as *mut uint32_t, delta.as_millis() as uint32_t)}
    }
    pub fn delay_yield(){
        Self::delay(Duration::from_millis(0));
    }

    pub fn priority(&self) -> u32{
        unsafe {task_get_priority(self.task)}
    }
    pub fn set_priority(&mut self, priority: u32){
        unsafe {task_set_priority(self.task, priority)}
    }
    pub fn state(&self) -> State{
        unsafe {task_get_state(self.task)}
    }
    pub fn suspend(&mut self){
        unsafe {task_suspend(self.task)}
    }
    pub fn resume(&mut self) {
        unsafe {task_resume(self.task)}
    }

    pub fn count() -> u32{
        unsafe {task_get_count()}
    }
    pub fn name(&self) -> String{
        unsafe{String::from(CStr::from_ptr(task_get_name(self.task)).to_str().unwrap())}
    }

    pub fn notify(&mut self){
        unsafe {task_notify(self.task);}
    }
    ///returns (prev_value, {true if written, false if not})
    pub fn notify_ext(&mut self, value: u32, action: NotifyAction) -> (u32, bool){
        let mut out = Default::default();
        let result = unsafe {task_notify_ext(self.task, value, action, &mut out as *mut uint32_t)};
        (out, result == 0)
    }
    pub fn notify_take(clear_on_exit: bool, timeout: u32) -> u32{
        unsafe {task_notify_take(clear_on_exit, timeout)}
    }
    pub fn notify_clear(&mut self) -> bool{
        unsafe {task_notify_clear(self.task)}
    }
}
impl !Send for Task{}
struct TaskArg<F: TaskFunction<T>, T: TaskArgument>{
    pub function: F,
    pub arg: T,
}
extern "C" fn task_function<F, T>(arg: *mut c_void)
    where F: TaskFunction<T>,
          T: TaskArgument{
    let task_arg;
    unsafe {
        task_arg = Box::from_raw(arg as *mut TaskArg<F, T>);
    }
    (task_arg.function)(task_arg.arg);
}
