// examples/yolo_example.rs

use perception::yolo::{YoloModel, Detection};

fn main() {
    // Load the YOLO model
    let model_path = "path_to_your_model/yolov4.onnx"; // Update with actual path
    let yolo_model = YoloModel::new(model_path).expect("Failed to load YOLO model");

    // Run detection on an image
    let image_path = "path_to_image/image.jpg"; // Update with actual image path
    let detections = yolo_model
        .detect(image_path)
        .expect("Failed to detect objects");

    // Display detected objects
    for detection in detections {
        println!(
            "Detected Object - x: {}, y: {}, w: {}, h: {}, confidence: {}",
            detection.x, detection.y, detection.w, detection.h, detection.confidence
        );
    }
}


