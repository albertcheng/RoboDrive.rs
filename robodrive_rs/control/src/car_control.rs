use crate::sensor::SensorData;
use crate::control::car_controller::CarController;

#[derive(Debug)]
pub enum DrivingAction {
    Accelerate(f32),
    Decelerate(f32),
    Turn(f32),
    Stop,
    MaintainSpeed,
}

pub struct SelfDrivingCar;

impl SelfDrivingCar {
    pub fn process_and_execute(sensor_data: &SensorData, controller: &mut CarController) {
        let action = SelfDrivingCar::make_decision(sensor_data);
        println!("Driving Decision: {:?}", action);
        controller.execute_action(action);
    }

    fn make_decision(sensor_data: &SensorData) -> DrivingAction {
        if let Some(true) = sensor_data.traffic_signals.get("red") {
            return DrivingAction::Stop;
        }

        if sensor_data.camera_objects.contains(&"pedestrian".to_string()) {
            return DrivingAction::Stop;
        }

        match sensor_data.road_condition.as_str() {
            "wet" | "icy" => return DrivingAction::Decelerate(5.0),
            _ => {}
        }

        if SelfDrivingCar::is_approaching_turn(sensor_data.gps_location) {
            return DrivingAction::Turn(30.0);
        }

        DrivingAction::MaintainSpeed
    }

    fn is_approaching_turn(gps_location: (f64, f64)) -> bool {
        let (lat, lon) = gps_location;
        (lat * lon) % 2.0 > 1.5
    }
}

