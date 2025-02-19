use tch::{nn, nn::Module, nn::OptimizerConfig, Device, Tensor};

pub struct NeuralNet {
    pub model: nn::Sequential,
}

impl NeuralNet {
    pub fn new(vs: &nn::Path) -> Self {
        let model = nn::seq()
            .add(nn::linear(vs, 128, 64, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(vs, 64, 32, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(vs, 32, 10, Default::default())); // 10 output classes

        Self { model }
    }

    pub fn forward(&self, x: &Tensor) -> Tensor {
        self.model.forward(x)
    }
}

