use crate::iris::Iris;
use rand::Rng;
use std::{error::Error, fs::File};
#[derive(Debug)]
pub struct Dataset {
    pub data: Vec<Iris>,
    random_sample: Vec<Iris>,
    target_output: Vec<f32>,
}

impl Dataset {
    pub fn new() -> Dataset {
        Dataset {
            data: Vec::new(),
            random_sample: Vec::new(),
            target_output: Vec::new(),
        }
    }

    pub fn parse_csv(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            match result {
                Ok(record) => self.data.push(record),
                Err(e) => println!("Error: {}", e),
            }
        }

        Ok(())
    }

    pub fn iris_as_vec(&self) -> Vec<Vec<f32>> {
        let mut iris_vec: Vec<Vec<f32>> = Vec::new();
        for iris in &self.data {
            iris_vec.push(iris.as_vec());
        }

        iris_vec
    }

    pub fn iris_as_array<const LEN: usize>(&self) -> [[f32; 4]; LEN] {
        let mut iris_array: [[f32; 4]; LEN] = [[0.0; 4]; LEN];
        for (i, iris) in self.data.iter().enumerate() {
            iris_array[i] = iris.as_array();
        }

        iris_array
    }

    pub fn print(&self) {
        for record in &self.data {
            println!("{:?}", record);
        }
    }

    pub fn create_random_sample(&mut self, sample_size: usize) {
        let mut sample: Vec<Iris> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..sample_size {
            let random_index = rng.gen_range(0..100);
            sample.push(self.data[random_index].clone());
        }

        self.random_sample = sample;
    }

    pub fn create_target_from_sample(&mut self) {
        let mut target_output: Vec<f32> = Vec::new();
        for row in self.random_sample.iter() {
            match row.species.as_str() {
                "Iris-setosa" => target_output.push(1.0),
                "Iris-versicolor" => target_output.push(-1.0),
                "Iris-virginica" => continue,
                _ => println!("Error: {} not found", row.species),
            }
        }

        self.target_output = target_output;
    }

    pub fn create_target_array_from_sample<const LEN: usize>(&self) -> [f32; LEN] {
        let mut target_output: [f32; LEN] = [0.0; LEN];
        for (i, row) in self.random_sample.iter().enumerate() {
            match row.species.as_str() {
                "Iris-setosa" => target_output[i] = 1.0,
                "Iris-versicolor" => target_output[i] = -1.0,
                "Iris-virginica" => continue,
                _ => println!("Error: {} not found", row.species),
            }
        }

        target_output
    }

    pub fn create_random_sample_array<const LEN: usize>(&self) -> [[f32; 4]; LEN] {
        let mut sample_array: [[f32; 4]; LEN] = [[0.0; 4]; LEN];
        for (i, iris) in self.random_sample.iter().enumerate() {
            sample_array[i] = iris.as_array();
        }

        sample_array
    }

    pub fn get_random_sample(&self) -> &Vec<Iris> {
        &self.random_sample
    }

    pub fn get_target_output(&self) -> &Vec<f32> {
        &self.target_output
    }
}
