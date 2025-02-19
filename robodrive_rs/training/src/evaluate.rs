use tch::{Device, Kind, Tensor};
use crate::model::NeuralNet;

pub fn evaluate_model(device: Device, model: &NeuralNet, test_images: &Tensor, test_labels: &Tensor) {
    let test_images = test_images.to(device);
    let test_labels = test_labels.to(device);

    // Forward pass
    let logits = model.forward(&test_images);
    let predictions = logits.argmax(1, false);

    // Compute accuracy
    let correct = predictions.eq_tensor(test_labels).to_kind(Kind::Float).sum(Kind::Float);
    let accuracy = f64::from(correct) / test_labels.size()[0] as f64;

    println!("Test Accuracy: {:.2}%", accuracy * 100.0);
}

