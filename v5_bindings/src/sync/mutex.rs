use crate::raw::pros::rtos::*;
use v5_traits::mutex::{MutexInner, MutexTimeoutInner};
use crate::sync::option_to_timeout;
use crate::console_println;
use core::time::Duration;
use crate::raw::pros::apix::sem_delete;

#[derive(Debug)]
pub struct ProsMutexInner {
    mutex: mutex_t,
}
impl Default for ProsMutexInner {
    fn default() -> Self {
        Self{ mutex: unsafe { mutex_create() } }
    }
}
impl Drop for ProsMutexInner{
    fn drop(&mut self) {
        unsafe { sem_delete(self.mutex) }
    }
}
impl MutexInner for ProsMutexInner{
    fn lock(&self) {
        unsafe { mutex_take(self.mutex, option_to_timeout(None)) };
    }

    fn is_locked(&self) -> bool {
        if unsafe { mutex_take(self.mutex, 0) }{
            unsafe { mutex_give(self.mutex) };
            false
        }
        else{
            true
        }
    }

    fn try_lock(&self) -> bool {
        unsafe { mutex_take(self.mutex, 0) }
    }

    unsafe fn unlock(&self) {
        while !mutex_give(self.mutex) {
            console_println("Could not give up mutex!!!");
        }
    }
}
impl MutexTimeoutInner for ProsMutexInner{
    fn lock_timeout(&self, timeout: Duration) -> bool {
        unsafe { mutex_take(self.mutex, option_to_timeout(Some(timeout))) }
    }
}
unsafe impl Send for ProsMutexInner{}
unsafe impl Sync for ProsMutexInner{}
