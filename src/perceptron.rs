use crate::iris::Iris;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Perceptron {
    learning_rate: f64,
    epochs: u32,
    pub weights: Vec<Vec<f64>>,
    bias: f64,
    pub error_count: Vec<u32>,
}

#[allow(dead_code)]
impl Perceptron {
    pub fn new(learning_rate: Option<f64>, epochs: Option<u32>, input: Vec<Iris>) -> Self {
        Perceptron {
            learning_rate: learning_rate.unwrap_or(0.1),
            epochs: epochs.unwrap_or(10),
            weights: vec![vec![1e-4; 4]; input.len()],
            bias: 0.0,
            error_count: Vec::new(),
        }
    }

    pub fn fit(&mut self, X: &Vec<Iris>, y: Vec<f64>) -> Self {
        let mut update;
        let mut errors = 0;
        for _i in 0..self.epochs.clone() {
            for target in &y {
                // Bias update
                let y_hat = self.predict(X);
                update = self.learning_rate * (target - y_hat);

                // Weight update
                self.update_weights(&X, update);
                self.bias += update;
                // Count errors
                if update != 0.0 {
                    errors += 1;
                }
            }
            self.error_count.push(errors);
            errors = 0;
        }
        self.clone()
    }

    pub fn predict(&self, X: &Vec<Iris>) -> f64 {
        let output = self.net_input(X);
        match output >= 0.0 {
            true => 1.0,
            false => -1.0,
        }
    }

    fn net_input(&self, X: &Vec<Iris>) -> f64 {
        let mut output = 0.0;

        for (w, x) in self.weights.iter().zip(X.iter()) {
            for (w_i, x_i) in w.iter().zip(x.to_owned().into_iter()) {
                output += (w_i * x_i) + self.bias;
            }
        }

        output
    }

    fn update_weights(&mut self, X: &Vec<Iris>, update: f64) {
        let weights = &mut self.weights;
        for i in 0..100 {
            for j in 0..4 {
                weights[i][j] = weights[i][j] + (update * X[i][j]);
            }
        }
    }
}
