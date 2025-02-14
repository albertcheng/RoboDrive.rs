mod control;
mod sensor;

use control::{car_controller::CarController, SelfDrivingCar};
use sensor::SensorData;
use std::collections::HashMap;

fn main() {
    let sensor_data = SensorData {
        camera_objects: vec!["car".to_string(), "pedestrian".to_string()],
        gps_location: (47.6101, -122.2015),
        speed: 30.0,
        steering_angle: 5.0,
        traffic_signals: [("red".to_string(), true)].iter().cloned().collect(),
        road_condition: "clear".to_string(),
    };

    let mut car_controller = CarController::new();
    SelfDrivingCar::process_and_execute(&sensor_data, &mut car_controller);
}

