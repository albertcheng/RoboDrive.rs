use tch::{nn, nn::OptimizerConfig, Device, Kind, Tensor};
use crate::model::NeuralNet;

pub fn train_model(device: Device, epochs: usize, batch_size: i64, learning_rate: f64) {
    // Load dataset (replace with actual dataset)
    let train_images = Tensor::randn(&[1000, 128], (Kind::Float, device)); // Fake data
    let train_labels = Tensor::randint(10, &[1000], (Kind::Int64, device));

    // Initialize model
    let vs = nn::VarStore::new(device);
    let net = NeuralNet::new(&vs.root());

    // Define optimizer
    let mut optimizer = nn::Adam::default().build(&vs, learning_rate).unwrap();

    for epoch in 0..epochs {
        let mut total_loss = 0.0;
        for i in (0..1000).step_by(batch_size as usize) {
            let batch_x = train_images.narrow(0, i, batch_size).to(device);
            let batch_y = train_labels.narrow(0, i, batch_size).to(device);

            optimizer.zero_grad();
            let output = net.forward(&batch_x);
            let loss = output.cross_entropy_for_logits(&batch_y);
            total_loss += f64::from(loss);
            
            loss.backward();
            optimizer.step();
        }

        println!("Epoch {}: Loss: {:.4}", epoch + 1, total_loss / (1000 / batch_size) as f64);
    }
}

