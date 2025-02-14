mod engine;
mod physics;
mod robot;

use anyhow::Result;
use engine::run_simulator;
use robot::SimulatorRobot;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <robot_binary>", args[0]);
        return Ok(());
    }
    let robot_binary_path = &args[1];
    if !Path::new(robot_binary_path).exists() {
        eprintln!("Error: Specified robot binary '{}' does not exist.", robot_binary_path);
        return Ok(());
    }

    // Start the simulation.
    run_simulator(robot_binary_path)
}
