use control::manipulation::robot_arm::RobotArm;

fn main() {
    // Create a new RobotArm with specific link lengths
    let arm = RobotArm::new(5.0, 3.0);
    
    // Define a target position for the end effector
    let target_x = 6.0;
    let target_y = 4.0;

    // Compute inverse kinematics
    match arm.inverse_kinematics(target_x, target_y) {
        Some((theta1, theta2)) => {
            println!("Joint Angles (in radians):");
            println!("Theta1: {:.2}", theta1);
            println!("Theta2: {:.2}", theta2);
        }
        None => {
            println!("Target is out of reach!");
        }
    }
}

