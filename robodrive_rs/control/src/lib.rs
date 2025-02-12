pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod manipulation {
    pub mod robot_arm; // Import the robot arm struct definition
}

pub mod kinematics; 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
