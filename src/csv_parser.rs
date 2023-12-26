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

// Need to :
// 1. Read the file
// 2. Parse the file
// 3. Store Records in a vector
// 5. Handle errors
// 6. Handle missing values
// 7. Handle invalid values
// 8. Handle invalid file
// 9. Clean Data?
// 10. Return the vector
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
