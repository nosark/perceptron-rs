use crate::iris::Iris;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Perceptron {
    learning_rate: f64,
    epochs: u32,
    weights: Vec<Vec<f64>>,
    bias: f64,
    error_count: Vec<u32>,
}

#[allow(dead_code)]
impl Perceptron {
    pub fn new(learning_rate: Option<f64>, epochs: Option<u32>, input: &Vec<Iris>) -> Self {
        Perceptron {
            learning_rate: learning_rate.unwrap_or(0.1),
            epochs: epochs.unwrap_or(10),
            weights: vec![vec![1e-4; 4]; input.len()],
            bias: 0.0,
            error_count: Vec::new(),
        }
    }

    pub fn fit(&mut self, X: &Vec<Iris>, y: &Vec<f64>) -> Self {
        let mut update;
        let mut errors = 0;
        let mut weight_index = 0;
        for _i in 0..self.epochs.clone() {
            for (target, xi) in y.iter().zip(X.iter()) {
                // Bias update
                let y_hat = self.predict(weight_index, xi);
                update = self.learning_rate * (target - y_hat);

                // Weight update
                self.update_weights(weight_index, xi, update);
                self.bias += update;
                // Count errors
                let error_rate = target - y_hat;
                if error_rate != 0.0 {
                    errors += 1;
                }
                weight_index += 1;
            }
            self.error_count.push(errors);
            errors = 0;
            weight_index = 0;
        }
        self.clone()
    }

    pub fn predict(&self, wi: usize, X: &Iris) -> f64 {
        let output = self.net_input(wi, X);
        match output >= 0.0 {
            true => 1.0,
            false => -1.0,
        }
    }

    fn net_input(&self, wi: usize, X: &Iris) -> f64 {
        let output;
        output = self.dot(wi, X) + self.bias;
        output
    }

    fn update_weights(&mut self, wi: usize, X: &Iris, update: f64) {
        let weights = &mut self.weights;
        for i in 0..X.len() {
            weights[wi][i] += update * X[i];
        }
    }

    pub fn get_error_count(&self) -> Vec<u32> {
        self.error_count.clone()
    }

    fn dot(&self, wi: usize, X: &Iris) -> f64 {
        let mut output = 0.0;
        for i in 0..X.len() {
            output += self.weights[wi][i] * X[i];
        }
        output
    }
}
