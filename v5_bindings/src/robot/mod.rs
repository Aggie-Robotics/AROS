pub mod motor;
pub mod port;
pub mod vision;
pub mod controller;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;

use controller::*;

use crate::robot::motor::Motor;
use crate::robot::vision::Vision;

pub struct Robot {
    pub master_controller: Controller,
    pub partner_controller: Controller,
    pub motors: Vec<Motor>,
    pub visions: Vec<Vision>,
    pub user_data: Option<Box<dyn Any>>,
}
impl Robot {
    pub const fn new() -> Self {
        Self {
            master_controller: Controller::new(Master),
            partner_controller: Controller::new(Partner),
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
