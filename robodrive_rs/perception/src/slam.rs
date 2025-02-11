use nalgebra::{Isometry3, Matrix3, Vector3};
use crate::feature::{extract_features, match_features};
use crate::pose_estimation::estimate_pose;

pub struct VisualSLAM {
    pub current_pose: Isometry3<f64>,
}

impl VisualSLAM {
    pub fn new() -> Self {
        VisualSLAM {
            current_pose: Isometry3::identity(),
        }
    }

    pub fn process_frame(&mut self, frame: &Image) {
        // Step 1: Extract features
        let features = extract_features(frame);

        // Step 2: Match features with the previous frame (not shown here)
        let matched_features = match_features(&features);

        // Step 3: Estimate pose from matched features
        if let Some(pose) = estimate_pose(&matched_features) {
            self.current_pose = self.current_pose * pose;
            println!("Updated Pose: {:?}", self.current_pose);
        }
    }
}

