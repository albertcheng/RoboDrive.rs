use DrivingAction;

pub struct CarController {
    speed: f32,
    steering_angle: f32,
}

impl CarController {
    pub fn new() -> Self {
        CarController {
            speed: 0.0,
            steering_angle: 0.0,
        }
    }

    pub fn execute_action(&mut self, action: DrivingAction) {
        match action {
            DrivingAction::Accelerate(amount) => self.accelerate(amount),
            DrivingAction::Decelerate(amount) => self.decelerate(amount),
            DrivingAction::Turn(angle) => self.turn(angle),
            DrivingAction::Stop => self.stop(),
            DrivingAction::MaintainSpeed => self.maintain_speed(),
        }
    }

    fn accelerate(&mut self, amount: f32) {
        self.speed += amount;
        println!("Accelerating by {} m/s². Current speed: {} m/s", amount, self.speed);
    }

    fn decelerate(&mut self, amount: f32) {
        self.speed -= amount;
        if self.speed < 0.0 {
            self.speed = 0.0;
        }
        println!("Decelerating by {} m/s². Current speed: {} m/s", amount, self.speed);
    }

    fn turn(&mut self, angle: f32) {
        self.steering_angle += angle;
        println!("Turning by {} degrees. Current steering angle: {}°", angle, self.steering_angle);
    }

    fn stop(&mut self) {
        self.speed = 0.0;
        println!("Car stopped. Current speed: {} m/s", self.speed);
    }

    fn maintain_speed(&self) {
        println!("Maintaining current speed: {} m/s", self.speed);
    }
}

