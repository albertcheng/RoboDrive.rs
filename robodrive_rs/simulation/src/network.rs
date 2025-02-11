pub struct NetworkChannel {
    pub active_transmissions: usize,
}

impl NetworkChannel {
    pub fn new() -> Self {
        NetworkChannel {
            active_transmissions: 0,
        }
    }

    pub fn start_transmission(&mut self) {
        self.active_transmissions += 1;
    }

    pub fn reset(&mut self) {
        self.active_transmissions = 0;
    }

    pub fn is_collision(&self) -> bool {
        self.active_transmissions > 1
    }
}

