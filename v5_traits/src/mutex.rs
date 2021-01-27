use core::time::Duration;
use core::cell::UnsafeCell;
use crate::EnsureSend;
use core::ops::{Deref, DerefMut};

pub trait Mutex<'m>: Send + Sync{
    type Inner: Send;
    type Guard: DerefMut<Target=Self::Inner>;

    fn new(val: Self::Inner) -> Self where Self: Sized;
    fn lock(&'m self) -> Self::Guard;
    fn is_locked(&self) -> bool;
    fn try_lock(&'m self) -> Option<Self::Guard>;
    fn into_inner(self) -> Self::Inner;
}
pub trait MutexTimeout<'m>: Mutex<'m>{
    fn lock_timeout(&self, timeout: Duration) -> Option<Self::Guard>;
}

pub trait MutexInner: Send + Sync + Default{
    fn lock(&self);
    fn is_locked(&self) -> bool;
    /// Returns true if this call locked the mutex
    fn try_lock(&self) -> bool;
    unsafe fn unlock(&self);
}
pub trait MutexTimeoutInner: MutexInner + Send + Sync{
    fn lock_timeout(&self, timeout: Duration) -> bool;
}

pub struct CustomMutex<M, T> where M: MutexInner, T: Send{
    mutex_inner: M,
    data: UnsafeCell<T>,
}
impl<M, T> CustomMutex<M, T> where M: MutexInner, T: Send{
    pub fn from_inner(mutex_inner: M, val: T) -> Self{
        Self{ mutex_inner, data: UnsafeCell::new(val) }
    }
}
impl<'m, M, T> Mutex<'m> for CustomMutex<M, T> where M: 'static + MutexInner, T: 'static + Send{
    type Inner = T;
    type Guard = MutexGuard<'m, M, T>;

    fn new(val: T) -> Self where Self: Sized {
        Self{ mutex_inner: Default::default(), data: UnsafeCell::new(val) }
    }

    fn lock(&'m self) -> Self::Guard {
        self.mutex_inner.lock();
        MutexGuard{ mutex: self }
    }

    fn is_locked(&self) -> bool {
        self.mutex_inner.is_locked()
    }

    fn try_lock(&'m self) -> Option<Self::Guard> {
        if self.mutex_inner.try_lock(){
            Some(MutexGuard{ mutex: self })
        }
        else{
            None
        }
    }

    fn into_inner(self) -> T {
        self.data.into_inner()
    }
}
unsafe impl<M, T> Sync for CustomMutex<M, T> where M: MutexInner, T: Send{}
impl<M, T> EnsureSend for CustomMutex<M, T> where M: MutexInner, T: Send{}

pub struct MutexGuard<'m, M, T> where M: MutexInner, T: Send{
    mutex: &'m CustomMutex<M, T>,
}
impl<'m, M, T> Deref for MutexGuard<'m, M, T> where M: MutexInner, T: Send{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{&*self.mutex.data.get()}
    }
}
impl<'m, M, T> DerefMut for MutexGuard<'m, M, T> where M: MutexInner, T: Send{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.mutex.data.get()}
    }
}
impl<'m, M, T> Drop for MutexGuard<'m, M, T> where M: MutexInner, T: Send{
    fn drop(&mut self) {
        unsafe { self.mutex.mutex_inner.unlock() }
    }
}
