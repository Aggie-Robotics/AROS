use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr::null_mut;
use alloc::boxed::Box;

pub trait Mutex<T>: Send + Sync where T: Send{
    type Guard: MutexGuard<T>;

    fn new(value: T) -> Self where Self: Sized;
    fn lock(&self) -> Self::Guard;
    fn is_locked(&self) -> bool;
}
pub trait MutexGuard<T>: Deref<Target=T> + DerefMut<Target=T> where T: Send{}

pub struct SyncCell<T>{
    data: AtomicPtr<T>,
}
impl<T> SyncCell<T>{
    pub fn new(value: Option<Box<T>>) -> Self{
        match value{
            None => Self{ data: AtomicPtr::new(null_mut()) },
            Some(value) => Self{ data: AtomicPtr::new(Box::leak(value)) }
        }
    }

    pub fn swap(&self, new: Option<Box<T>>) -> Option<Box<T>>{
        let new = match new{
            None => null_mut(),
            Some(new) => Box::leak(new),
        };
        let taken = self.data.swap(new, Ordering::SeqCst);
        if taken.is_null(){
            None
        }
        else{
            let taken = unsafe {Box::from_raw(taken)};
            Some(taken)
        }
    }
}
impl<T> Drop for SyncCell<T>{
    fn drop(&mut self) {
        self.swap(None);
    }
}
impl<T> Default for SyncCell<T>{
    fn default() -> Self {
        Self::new(None)
    }
}
unsafe impl<T> Sync for SyncCell<T>{}
