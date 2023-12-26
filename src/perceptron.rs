use ndarray::prelude::*;
#[allow(dead_code)]
pub struct Perceptron {
    learning_rate: f64,
    epochs: u32,
    weights: Array1<f64>,
    bias: f64,
    activation_function: fn(f64)->f64,
    y_hat: f64,
    linear_output: f64,
}

// TODO: need to decide if dimensions are decided based on features,
// and if we should just create N-dimensional arrays based on perceptron creation 
// should write the perceptron and test math with 1D arrays first on the IRIS dataset
// and then move to N-dimensional arrays
//
// TODO: need to decide if we should use a generic type for the activation activation_function
// create math module for activation functions and other formulas
// to speed up fitting, should look at using iter.zip() to iterate over X and y
#[allow(dead_code)]
impl Perceptron {
    fn new(mut self, learning_rate: Option<f64>,epochs: Option<u32>, activation_func: fn(f64)->f64) -> Self {
        Perceptron {
            learning_rate: learning_rate.unwrap_or(0.1),
            epochs: epochs.unwrap_or(100),
            weights: Array1::zeros(0),
            bias: 0.0,
            activation_function: activation_func,
            y_hat: 0.0,
            linear_output: 0.0,
        }
    }

    fn fit(mut self,X: Array1<f64>, mut y: Array1<f64>) {
        let mut n_features = X.shape();
        let mut update;

        // TODO: try to use random weight initilisation in future
        self.weights = Array1::zeros(n_features.len());
        // set bias to 0 in initialisation
        y = y.mapv(|y| if y > 0.0 { 1.0 } else { 0.0 });

        // learn weights
        for _ in 0..self.epochs {
            for (i, x_i) in X.indexed_iter() {
                self.linear_output = self.weights.dot(&X) + self.bias;
                self.y_hat = (self.activation_function)(self.linear_output); 
                
                // Perceptron update rule 
                update = self.learning_rate * (y[i] - self.y_hat);
                self.weights += update * x_i;
                self.bias += update;    
            }
        }
    }

    pub fn predict(mut self, X: Array1<f64>) -> f64 {
        self.linear_output = self.weights.dot(&X) + self.bias;
        self.y_hat = (self.activation_function)(self.linear_output);
        self.y_hat
    }
}
