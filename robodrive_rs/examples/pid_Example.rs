use control::pid_controller::PIDController;

fn main() {
    // Initialize PID controller with sample gains
    let mut pid = PIDController::new(1.0, 0.1, 0.01, 100.0); // Kp = 1, Ki = 0.1, Kd = 0.01, Setpoint = 100

    // Simulate a process loop where we update the PID controller with current measurements
    let current_value = 95.0;  // Example current value from the system
    let control_signal = pid.update(current_value);

    println!("Control Signal: {}", control_signal);
    
    // Adjust the setpoint if needed
    pid.set_setpoint(110.0);
    let new_control_signal = pid.update(95.0);
    println!("New Control Signal after setpoint change: {}", new_control_signal);
    
    // Reset the controller if necessary
    pid.reset();
}

