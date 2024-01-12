use crate::iris::Iris;
use std::{error::Error, fs::File};

#[derive(Debug)]
pub struct Features {
    pub data: Vec<Iris>,
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
