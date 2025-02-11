// src/drone.rs
pub struct Drone {
    pub altitude: f32,
    pub position: (f32, f32),
}

impl Drone {
    pub fn new(altitude: f32, position: (f32, f32)) -> Self {
        Drone { altitude, position }
    }

    pub fn fly(&mut self) {
        self.altitude += 1.0;
    }

    pub fn land(&mut self) {
        self.altitude = 0.0;
    }
}

