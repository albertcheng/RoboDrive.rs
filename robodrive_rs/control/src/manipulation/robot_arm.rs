#[derive(Debug)]
pub struct RobotArm {
    pub link1_length: f64,
    pub link2_length: f64,
}

impl RobotArm {
    pub fn new(link1_length: f64, link2_length: f64) -> Self {
        RobotArm {
            link1_length,
            link2_length,
        }
    }
}

