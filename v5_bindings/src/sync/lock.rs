use alloc::boxed::Box;
use core::ops::{Deref, DerefMut};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicIsize, Ordering};

use lock_api::{GuardSend, RawMutex, RawRwLock};

use crate::raw::pros::apix::*;
use crate::raw::pros::rtos::*;

pub type MutexInternal<T> = lock_api::Mutex<ProsMutex, T>;
pub struct Mutex<T: ?Sized>{
    inner: Box<MutexInternal<T>>,
}
impl<T> Mutex<T>{
    pub fn new(t: T) -> Self{
        Self{
            inner: Box::new(lock_api::Mutex::const_new(ProsMutex::new(), t)),
        }
    }

    pub fn into_inner(self) -> T{
        self.inner.into_inner()
    }
}
impl<T: ?Sized> Deref for Mutex<T>{
    type Target = MutexInternal<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T: ?Sized> DerefMut for Mutex<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
pub type MutexGuard<'a, T> = lock_api::MutexGuard<'a, ProsMutex, T>;

pub type RwLockInternal<T> = lock_api::RwLock<ProsRwLock, T>;
pub struct RwLock<T: ?Sized>{
    inner: RwLockInternal<T>,
}
impl<T> RwLock<T>{
    pub fn new(t: T) -> Self{
        Self{
            inner: RwLockInternal::const_new(ProsRwLock::new(), t),
        }
    }

    pub fn into_inner(self) -> T{
        self.inner.into_inner()
    }
}
impl<T: ?Sized> Deref for RwLock<T>{
    type Target = RwLockInternal<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized> DerefMut for RwLock<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub type RwLockReadGuard<'a, T> = lock_api::RwLockReadGuard<'a, ProsRwLock, T>;
pub type RwLockWriteGuard<'a, T> = lock_api::RwLockWriteGuard<'a, ProsRwLock, T>;

pub struct ProsMutex {
    mutex: mutex_t,
}

impl ProsMutex {
    fn new() -> Self {
        Self {
            mutex: unsafe { mutex_create() },
        }
    }

    const unsafe fn new_uninit() -> Self{
        Self{
            mutex: null_mut(),
        }
    }
}
unsafe impl RawMutex for ProsMutex {
    const INIT: Self = unsafe{Self::new_uninit()};
    type GuardMarker = GuardSend;

    fn lock(&self) {
        unsafe{mutex_take(self.mutex, TIMEOUT_MAX)};
    }

    fn try_lock(&self) -> bool {
        unsafe {mutex_take(self.mutex, 0)}
    }

    unsafe fn unlock(&self) {
        mutex_give(self.mutex);
    }
}
impl Drop for ProsMutex{
    fn drop(&mut self) {
        unsafe{sem_delete(self.mutex)}
    }
}
unsafe impl Send for ProsMutex{}
unsafe impl Sync for ProsMutex{}

pub struct ProsRwLock{
    write_mutex: ProsMutex,
    read_mutex: ProsMutex,
    ref_count: AtomicIsize,
}
impl ProsRwLock{
    fn new() -> Self{
        Self{
            write_mutex: ProsMutex::new(),
            read_mutex: ProsMutex::new(),
            ref_count: AtomicIsize::new(0),
        }
    }

    const unsafe fn new_uninit() -> Self{
        Self{
            write_mutex: ProsMutex::new_uninit(),
            read_mutex: ProsMutex::new_uninit(),
            ref_count: AtomicIsize::new(0),
        }
    }
}
unsafe impl RawRwLock for ProsRwLock{
    const INIT: Self = unsafe{Self::new_uninit()};
    type GuardMarker = GuardSend;

    fn lock_shared(&self) {
        self.read_mutex.lock();
        if self.ref_count.fetch_add(1, Ordering::SeqCst) == 0{
            self.write_mutex.lock();
        }
        unsafe {self.read_mutex.unlock()};
    }

    fn try_lock_shared(&self) -> bool {
        let mut out = false;
        if self.read_mutex.try_lock(){
            if self.ref_count.fetch_add(1, Ordering::SeqCst) == 0{
                out = self.write_mutex.try_lock()
            }
            else{
                out = true;
            }
            unsafe {self.read_mutex.unlock()};
        }
        out
    }

    unsafe fn unlock_shared(&self) {
        if self.ref_count.fetch_sub(1, Ordering::SeqCst) == 1{
            self.write_mutex.unlock()
        }
    }

    fn lock_exclusive(&self) {
        self.write_mutex.lock();
    }

    fn try_lock_exclusive(&self) -> bool {
        self.write_mutex.try_lock()
    }

    unsafe fn unlock_exclusive(&self) {
        self.write_mutex.unlock()
    }
}
unsafe impl Send for ProsRwLock{}
unsafe impl Sync for ProsRwLock{}

#[cfg(feature = "v5_test")]
pub mod test{
    use alloc::boxed::Box;
    use alloc::string::String;
    use core::time::Duration;

    use ansi_rgb::{Foreground, red};

    use crate::sync::lock::{Mutex, RwLock};
    use crate::test::{assert, TestItem, TestResult};
    use crate::test::TestType::Parallel;

    #[allow(unused_must_use)]
    pub fn mutex_test() -> TestItem{
        TestItem::new(String::from("mutex_test"), Parallel(Box::new(|| -> TestResult{
            let mutex = Mutex::new(153);
            let guard = mutex.try_lock();

            assert(guard.is_some(), Box::new("try_lock on empty failed".fg(red())))?;
            assert(mutex.try_lock().is_none(), Box::new("try_lock on locked succeeded".fg(red())))?;
            assert(*guard.unwrap() == 153, Box::new(format!("guard access failed!")))?;

            mutex.lock();

            Ok(())
        }), Duration::from_millis(100)))
    }

    #[allow(unused_must_use)]
    pub fn rw_lock_test() -> TestItem{
        TestItem::new(String::from("rw_lock_test"), Parallel(Box::new(|| -> TestResult{
            let rwlock = RwLock::new(7353);
            let shared_guard = rwlock.try_read();

            assert(shared_guard.is_some(), Box::new("try_read on empty failed".fg(red())))?;
            assert(rwlock.try_read().is_some(), Box::new("try_read on shared failed".fg(red())))?;
            assert(rwlock.try_write().is_none(), Box::new("try_write on shared succeeded".fg(red())))?;
            assert(*shared_guard.unwrap() == 7353, Box::new("shared_guard read failed".fg(red())))?;

            let exclusive_guard = rwlock.try_write();

            assert(exclusive_guard.is_some(), Box::new("try_write on empty failed".fg(red())))?;
            assert(rwlock.try_read().is_none(), Box::new("try_read on exclusive succeeded".fg(red())))?;
            assert(rwlock.try_write().is_none(), Box::new("try_write on exclusive succeeded".fg(red())))?;
            assert(*exclusive_guard.unwrap() == 7353, Box::new("exclusive_guard read failed".fg(red())))?;

            rwlock.read();
            rwlock.write();

            Ok(())
        }), Duration::from_millis(100)))
    }
}
