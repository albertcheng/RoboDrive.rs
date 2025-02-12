/// PID Controller
/// This controller uses the Proportional-Integral-Derivative (PID) method
/// to compute the control signal for the system.
pub struct PIDController {
    kp: f64,  // Proportional gain
    ki: f64,  // Integral gain
    kd: f64,  // Derivative gain
    prev_error: f64,  // Previous error value for derivative calculation
    integral: f64,    // Integral term
    setpoint: f64,    // Desired value
}

impl PIDController {
    /// Create a new PIDController instance with specified gains
    pub fn new(kp: f64, ki: f64, kd: f64, setpoint: f64) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            prev_error: 0.0,
            integral: 0.0,
            setpoint,
        }
    }

    /// Update the PID controller with a new measurement.
    ///
    /// - `current_value`: The current value from the system (e.g., measured sensor value).
    /// Returns the control output, which can be used to adjust the system.
    pub fn update(&mut self, current_value: f64) -> f64 {
        let error = self.setpoint - current_value;
        self.integral += error;
        let derivative = error - self.prev_error;

        // Compute PID output
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;

        // Save the current error for the next update
        self.prev_error = error;

        output
    }

    /// Set the desired setpoint (target value) for the controller
    pub fn set_setpoint(&mut self, setpoint: f64) {
        self.setpoint = setpoint;
    }

    /// Reset the PID controller to clear accumulated error and integral.
    pub fn reset(&mut self) {
        self.prev_error = 0.0;
        self.integral = 0.0;
    }
}

