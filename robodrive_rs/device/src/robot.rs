use std::fs::File;
use std::io::Read;
use anyhow::Result;
use rapier3d::prelude::*;

pub trait RobotBase {
    fn load_from_binary(path: &str) -> Result<Self>
    where
        Self: Sized;
    fn update(&self, rigid_body_set: &RigidBodySet, body_handle: RigidBodyHandle);
}

pub struct SimulatorRobot {
    pub name: String,
}

impl RobotBase for SimulatorRobot {
    fn load_from_binary(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(SimulatorRobot {
            name: String::from_utf8_lossy(&buffer).to_string(),
        })
    }

    fn update(&self, rigid_body_set: &RigidBodySet, body_handle: RigidBodyHandle) {
        if let Some(rigid_body) = rigid_body_set.get(body_handle) {
            let position = rigid_body.position();
            println!("{} position: {:?}", self.name, position);
        }
    }
}

