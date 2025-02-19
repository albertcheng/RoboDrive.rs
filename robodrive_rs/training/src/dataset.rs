use hdf5::File;
use ndarray::{Array2, Axis};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use tch::{Tensor, Device};

pub struct Dataset {
    pub features: Array2<f32>,
    pub labels: Array2<f32>,
}

impl Dataset {
    /// Load dataset from an HDF5 file
    pub fn load_from_h5(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;

        let features = file.dataset("features")?.read::<f32, _>()?;
        let labels = file.dataset("labels")?.read::<f32, _>()?;

        Ok(Self { features, labels })
    }

    /// Normalize feature values between 0 and 1
    pub fn normalize(&mut self) {
        let min = self.features.mapv(f32::min).min().unwrap();
        let max = self.features.mapv(f32::max).max().unwrap();
        self.features.mapv_inplace(|x| (x - min) / (max - min));
    }

    /// Shuffle dataset randomly
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut indices: Vec<usize> = (0..self.features.nrows()).collect();
        indices.shuffle(&mut rng);

        self.features = self.features.select(Axis(0), &indices);
        self.labels = self.labels.select(Axis(0), &indices);
    }

    /// Convert dataset to Torch tensors (optionally move to GPU)
    pub fn to_tensors(&self, device: Device) -> (Tensor, Tensor) {
        let features_tensor = Tensor::of_slice(self.features.as_slice().unwrap())
            .reshape(&[self.features.nrows() as i64, self.features.ncols() as i64])
            .to(device);

        let labels_tensor = Tensor::of_slice(self.labels.as_slice().unwrap())
            .reshape(&[self.labels.nrows() as i64, self.labels.ncols() as i64])
            .to(device);

        (features_tensor, labels_tensor)
    }

    /// Get dataset as mini-batches
    pub fn get_batches(&self, batch_size: usize) -> Vec<(Array2<f32>, Array2<f32>)> {
        let num_samples = self.features.nrows();
        let mut batches = Vec::new();

        for i in (0..num_samples).step_by(batch_size) {
            let end = (i + batch_size).min(num_samples);
            let batch_features = self.features.slice(s![i..end, ..]).to_owned();
            let batch_labels = self.labels.slice(s![i..end, ..]).to_owned();
            batches.push((batch_features, batch_labels));
        }

        batches
    }
}
