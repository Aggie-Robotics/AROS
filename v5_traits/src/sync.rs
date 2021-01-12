use core::ops::{Deref, DerefMut};
pub trait Mutex<T>: Send + Sync where T: Send{
    type Guard: MutexGuard<T>;

    fn new(value: T) -> Self where Self: Sized;
    fn lock(&self) -> Self::Guard;
    fn is_locked(&self) -> bool;
}
pub trait MutexGuard<T>: Deref<Target=T> + DerefMut<Target=T> where T: Send{}
