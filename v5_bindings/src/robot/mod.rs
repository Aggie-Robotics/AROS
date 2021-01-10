use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::any::Any;
use core::cell::*;
use core::ops::{Deref, DerefMut};

use crate::controller::Controller;
use crate::robot::motor::Motor;
use crate::robot::vision::Vision;
use crate::sync::lock::*;

pub mod motor;
pub mod port;
pub mod vision;

pub struct Robot {
    pub controllers: [Controller; 2],
    pub motors: Vec<RobotComponent<Motor>>,
    pub visions: Vec<RobotComponent<Vision>>,
    pub user_data: Option<Box<dyn Any>>,
}

impl Robot {
    pub const fn new() -> Self {
        Self {
            controllers: Controller::get_all(),
            motors: Vec::new(),
            visions: Vec::new(),
            user_data: None,
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

pub enum RobotComponent<T> {
    NoLock(T),
    SingleThreadLock(Rc<RefCell<T>>),
    MultiThreadLock(Arc<Mutex<T>>),
    MultiThreadRwLock(Arc<RwLock<T>>),
}

impl<T> RobotComponent<T> {
    pub const fn no_lock(component: T) -> Self {
        Self::NoLock(component)
    }
    pub fn single_thread_lock(component: T) -> Self {
        Self::SingleThreadLock(Rc::new(RefCell::new(component)))
    }
    pub fn multi_thread_lock(component: T) -> Self {
        Self::MultiThreadLock(Arc::new(Mutex::new(component)))
    }
    pub fn multi_thread_rw_lock(component: T) -> Self {
        Self::MultiThreadRwLock(Arc::new(RwLock::new(component)))
    }

    pub fn lock(&self) -> ComponentGuard<T> {
        match self {
            RobotComponent::NoLock(component) => ComponentGuard::NoGuard(component),
            RobotComponent::SingleThreadLock(component) => ComponentGuard::SingleThreadGuard(component.deref().borrow()),
            RobotComponent::MultiThreadLock(component) => ComponentGuard::MultiThreadGuard(component.lock()),
            RobotComponent::MultiThreadRwLock(component) => ComponentGuard::MultiThreadReadGuard(component.read()),
        }
    }
}

pub enum ComponentGuard<'a, T> {
    NoGuard(&'a T),
    SingleThreadGuard(Ref<'a, T>),
    MultiThreadGuard(MutexGuard<'a, T>),
    MultiThreadReadGuard(RwLockReadGuard<'a, T>),
}

impl<'a, T> Deref for ComponentGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ComponentGuard::NoGuard(component) => *component,
            ComponentGuard::SingleThreadGuard(component) => component.deref(),
            ComponentGuard::MultiThreadGuard(component) => component.deref(),
            ComponentGuard::MultiThreadReadGuard(component) => component.deref(),
        }
    }
}

pub enum MutComponentGuard<'a, T> {
    SingleThreadGuard(RefMut<'a, T>),
    MultiThreadGuard(MutexGuard<'a, T>),
    MultiThreadReadGuard(RwLockWriteGuard<'a, T>),
}

impl<'a, T> Deref for MutComponentGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MutComponentGuard::SingleThreadGuard(component) => component.deref(),
            MutComponentGuard::MultiThreadGuard(component) => component.deref(),
            MutComponentGuard::MultiThreadReadGuard(component) => component.deref(),
        }
    }
}

impl<'a, T> DerefMut for MutComponentGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MutComponentGuard::SingleThreadGuard(component) => component.deref_mut(),
            MutComponentGuard::MultiThreadGuard(component) => component.deref_mut(),
            MutComponentGuard::MultiThreadReadGuard(component) => component.deref_mut(),
        }
    }
}

