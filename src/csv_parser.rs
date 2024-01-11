use std::ops::Index;
use std::{error::Error, fs::File};

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Iris {
    //    id: u32,
    sepal_length_cm: f64,
    sepal_width_cm: f64,
    petal_length_cm: f64,
    petal_width_cm: f64,
    //   species: String,
}

impl Index<usize> for Iris {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.sepal_length_cm,
            1 => &self.sepal_width_cm,
            2 => &self.petal_length_cm,
            3 => &self.petal_width_cm,
            _ => panic!("Invalid index"),
        }
    }
}

#[derive(Debug)]
pub struct Features {
    data: Vec<Iris>,
}

impl Features {
    pub fn new() -> Features {
        Features { data: Vec::new() }
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
    pub fn print(&self) {
        for record in &self.data {
            println!("{:?}", record);
        }
    }
}
