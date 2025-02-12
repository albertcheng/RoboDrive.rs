// perception/src/yolo.rs

use onnxruntime::{environment::Environment, tensor::Tensor, session::Session};
use ndarray::Array;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

#[derive(Debug)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub confidence: f32,
}

pub struct YoloModel {
    session: Session,
}

impl YoloModel {
    // Initialize YOLO with the given model path
    pub fn new(model_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let environment = Environment::builder()
            .with_name("yolo_runtime")
            .build()?;
        
        let session = environment
            .new_session_builder()?
            .with_model_from_file(model_path)?;

        Ok(YoloModel { session })
    }

    // Run inference on an image and return detections
    pub fn detect(&self, image_path: &str) -> Result<Vec<Detection>, Box<dyn std::error::Error>> {
        let image = self.load_image(image_path)?;
        let input_tensor = self.preprocess_image(image);
        let output_tensor = self.run_inference(input_tensor)?;
        let detections = self.postprocess_output(output_tensor);
        
        Ok(detections)
    }

    // Load and preprocess the image for YOLO input
    fn load_image(&self, image_path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let image = DynamicImage::open(image_path)?;
        Ok(image)
    }

    // Convert the image into a tensor format suitable for YOLO input
    fn preprocess_image(&self, image: DynamicImage) -> Tensor {
        let resized_image = image.resize(416, 416, image::imageops::FilterType::Nearest);
        let image_data = resized_image.to_rgb8().to_vec();
        let input_array = Array::from_shape_vec((1, 3, 416, 416), image_data).unwrap();
        Tensor::from(input_array)
    }

    // Run inference using ONNX runtime
    fn run_inference(&self, input_tensor: Tensor) -> Result<Tensor, Box<dyn std::error::Error>> {
        let inputs = vec![input_tensor];
        let outputs = self.session.run(inputs)?;
        Ok(outputs[0].clone()) // Return the first output tensor
    }

    // Post-process the output to extract bounding boxes and class information
    fn postprocess_output(&self, output_tensor: Tensor) -> Vec<Detection> {
        let output_data = output_tensor.to_array_view::<f32>().unwrap();
        let mut detections = Vec::new();

        for row in output_data.outer_iter() {
            let (x, y, w, h, conf) = (
                row[0], // x-center of bounding box
                row[1], // y-center of bounding box
                row[2], // width of bounding box
                row[3], // height of bounding box
                row[4], // confidence score
            );

            if conf > 0.5 {
                detections.push(Detection {
                    x,
                    y,
                    w,
                    h,
                    confidence: conf,
                });
            }
        }

        detections
    }
}

