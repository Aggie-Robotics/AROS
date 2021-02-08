use core::time::Duration;
use core::cell::UnsafeCell;
use crate::EnsureSend;

pub trait Mutex: Send + Sync{
    type Inner: Send;

    fn new(val: Self::Inner) -> Self where Self: Sized;
    fn lock<R>(&self, f: impl FnOnce(&mut Self::Inner) -> R) -> R;
    fn is_locked(&self) -> bool;
    fn try_lock<R, F>(&self, f: F) -> Result<R, F> where F: FnOnce(&mut Self::Inner) -> R;
    fn into_inner(self) -> Self::Inner;
}
pub trait MutexTimeout: Mutex{
    fn lock_timeout<R, F>(&self, timeout: Duration, f: F) -> Result<R, F> where F: FnOnce(&mut Self::Inner) -> R;
}

pub trait MutexInner: Send + Sync + Default{
    fn lock(&self);
    fn is_locked(&self) -> bool;
    /// Returns true if this call locked the mutex
    fn try_lock(&self) -> bool;
    /// # Safety
    /// This function is not safe to call as singular access may be broken
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
impl<M, T> Mutex for CustomMutex<M, T> where M: MutexInner, T: Send{
    type Inner = T;

    fn new(val: T) -> Self where Self: Sized {
        Self{ mutex_inner: Default::default(), data: UnsafeCell::new(val) }
    }

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Inner) -> R) -> R {
        self.mutex_inner.lock();
        let out = f(unsafe { &mut *self.data.get() });
        unsafe { self.mutex_inner.unlock() };
        out
    }

    fn is_locked(&self) -> bool {
        self.mutex_inner.is_locked()
    }

    fn try_lock<R, F>(&self, f: F) -> Result<R, F> where F: FnOnce(&mut Self::Inner) -> R {
        if self.mutex_inner.try_lock(){
            let out = f(unsafe{ &mut *self.data.get() });
            unsafe { self.mutex_inner.unlock() };
            Ok(out)
        }
        else{
            Err(f)
        }
    }

    fn into_inner(self) -> T {
        self.data.into_inner()
    }
}
impl<M, T> MutexTimeout for CustomMutex<M, T> where M: MutexTimeoutInner, T: Send{
    fn lock_timeout<R, F>(&self, timeout: Duration, f: F) -> Result<R, F> where F: FnOnce(&mut Self::Inner) -> R {
        if self.mutex_inner.lock_timeout(timeout){
            unsafe{
                let out = f(&mut *self.data.get());
                self.mutex_inner.unlock();
                Ok(out)
            }
        }
        else{
            Err(f)
        }
    }
}
unsafe impl<M, T> Sync for CustomMutex<M, T> where M: MutexInner, T: Send{}
impl<M, T> EnsureSend for CustomMutex<M, T> where M: MutexInner, T: Send{}

