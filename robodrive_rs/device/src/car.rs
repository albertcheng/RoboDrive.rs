// src/device/car.rs

pub struct Car {
    pub x: f64,
    pub y: f64,
    pub orientation: f64, // Angle in radians
    pub speed: f64,
    pub max_speed: f64,
}

impl Car {
    pub fn new(x: f64, y: f64, orientation: f64, max_speed: f64) -> Self {
        Car {
            x,
            y,
            orientation,
            speed: 0.0,
            max_speed,
        }
    }

    pub fn steer(&mut self, angle: f64) {
        self.orientation += angle;
        if self.orientation > 2.0 * std::f64::consts::PI {
            self.orientation -= 2.0 * std::f64::consts::PI;
        } else if self.orientation < 0.0 {
            self.orientation += 2.0 * std::f64::consts::PI;
        }
    }

    pub fn accelerate(&mut self, throttle: f64) {
        self.speed += throttle;
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        } else if self.speed < 0.0 {
            self.speed = 0.0;
        }
    }

    pub fn move_forward(&mut self) {
        self.x += self.speed * self.orientation.cos();
        self.y += self.speed * self.orientation.sin();
    }

    pub fn display(&self) {
        println!("Car position: ({:.2}, {:.2}), Orientation: {:.2} radians, Speed: {:.2} m/s", self.x, self.y, self.orientation, self.speed);
    }
}

