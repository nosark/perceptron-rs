use ndarray::prelude::*;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Perceptron {
    learning_rate: f64,
    epochs: u32,
    weights: Array1<f64>,
    bias: f64,
    y_hat: f64,
    linear_output: f64,
    errors: Vec<u32>,
}

#[allow(dead_code)]
impl Perceptron {
    pub fn new(learning_rate: Option<f64>, epochs: Option<u32>) -> Self {
        Perceptron {
            learning_rate: learning_rate.unwrap_or(0.1),
            epochs: epochs.unwrap_or(100),
            weights: Array1::zeros(0),
            bias: 0.0,
            y_hat: 0.0,
            linear_output: 0.0,
            errors: Vec::new(),
        }
    }

    pub fn fit(mut self, X: Array1<f64>, mut y: Array1<f64>) -> Self {
        let n_features = X.shape();
        let mut update;
        let mut errors = 0;
        // TODO: try to use random weight initilisation in future
        self.weights = Array1::zeros(n_features.len());
        // set bias to 0 in initialisation
        y = y.mapv(|y| if y >= 0.0 { 1.0 } else { -1.0 });

        // learn weights
        for _ in 0..self.epochs {
            for (x, target) in X.iter().zip(y.iter()) {
                update = self.learning_rate * (target - &self.clone().predict(*x));
                self.weights.mapv(|w| w + (update * x));
                self.bias += update;
                if update != 0.0 {
                    errors += 1;
                }
                self.errors.push(errors);
            }
        }

        self
    }

    pub fn predict(self, X: f64) -> f64 {
        let output = self.net_input(X);
        match output >= 0.0 {
            true => 1.0,
            false => -1.0,
        }
    }

    fn net_input(self, X: f64) -> f64 {
        self.weights.mapv(|w| w * X).sum() + self.bias
    }
}
