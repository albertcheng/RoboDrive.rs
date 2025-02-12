use crate::manipulation::robot_arm::RobotArm;
use std::f64::consts::PI;

impl RobotArm {
    // Inverse Kinematics Calculation
    pub fn inverse_kinematics(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        // Calculate the distance to the target
        let r = (x * x + y * y).sqrt();

        // Check if the target is reachable
        if r > self.link1_length + self.link2_length {
            // Target is out of reach
            return None;
        }

        // Law of cosines to calculate theta2 (elbow angle)
        let cos_theta2 = (r * r - self.link1_length * self.link1_length - self.link2_length * self.link2_length)
            / (2.0 * self.link1_length * self.link2_length);
        let theta2 = cos_theta2.acos();

        // Calculate theta1 (shoulder angle)
        let k1 = self.link1_length + self.link2_length * cos_theta2;
        let k2 = self.link2_length * (1.0 - cos_theta2 * cos_theta2).sqrt();
        let theta1 = (y.atan2(x) - (k2.atan2(k1))) % (2.0 * PI);

        Some((theta1, theta2))
    }
}

