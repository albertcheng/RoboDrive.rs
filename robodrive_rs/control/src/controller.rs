// src/control/src/controller.rs

use crate::device::car::Car; // Correct import path

pub struct Controller;

impl Controller {
    pub fn control(car: &mut Car, throttle: f64, steering: f64) {
        car.accelerate(throttle);
        car.steer(steering);
    }
}

