use simulation::network::NetworkChannel;
use rand::Rng;

const TRANSMISSION_PROBABILITY: f64 = 0.1;
const MAX_RETRIES: u32 = 5;

struct MobileNode {
    id: u32,
    retry_count: u32,
}

impl MobileNode {
    fn new(id: u32) -> Self {
        Self { id, retry_count: 0 }
    }

    fn attempt_transmission(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < TRANSMISSION_PROBABILITY
    }
}

fn run_mobile_aloha_simulation(num_nodes: u32, max_iterations: u32) {
    let mut network = NetworkChannel::new();
    let mut rng = rand::thread_rng();

    for iteration in 1..=max_iterations {
        if rng.gen::<f64>() < 0.1 {
            network.start_transmission();
        }
        if network.is_collision() {
            println!("Collision detected in iteration {}", iteration);
        }
    }
}

fn main() {
    run_mobile_aloha_simulation(10, 100);
}
