use fyrox::engine::Engine;
use fyrox::event::{Event, WindowEvent};
use fyrox::event_loop::{ControlFlow, EventLoop};
use fyrox::scene::Scene;
use fyrox::window::WindowBuilder;
use anyhow::Result;
use std::sync::Arc;

use crate::physics::{setup_physics, step_physics};
use crate::robot::SimulatorRobot;

pub fn run_simulator(robot_binary_path: &str) -> Result<()> {
    let event_loop = EventLoop::new();

    let mut engine = Engine::new(fyrox::engine::EngineInitParams {
        window_builder: WindowBuilder::new().with_title("Rust Robotics Simulator"),
        resource_manager: Default::default(),
        serialization_context: Arc::new(Default::default()),
        vsync: true,
    })?;

    let scene = Scene::new();
    let scene_handle = engine.scenes.add(scene);

    let (mut physics_pipeline, mut rigid_body_set, mut collider_set) = setup_physics();
    let mut robot = SimulatorRobot::load_from_binary(robot_binary_path)?;
    let robot_handle = robot.initialize(&mut rigid_body_set, &mut collider_set);

    engine.event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::MainEventsCleared => {
                step_physics(&mut physics_pipeline, &mut rigid_body_set, &mut collider_set);
                robot.update(&rigid_body_set, robot_handle);
                engine.update(fyrox::core::time::Instant::now());
            }
            _ => {}
        }
    });

    Ok(())
}

