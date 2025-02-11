// src/lawn_mower.rs
pub struct LawnMower {
    pub speed: f32,
    pub position: (f32, f32),
}

impl LawnMower {
    pub fn new(speed: f32, position: (f32, f32)) -> Self {
        LawnMower { speed, position }
    }

    pub fn mow(&mut self) {
        self.position.0 += self.speed;
    }

    pub fn stop(&mut self) {
        self.speed = 0.0;
    }
}

