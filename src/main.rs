use perceptron::csv_parser::*;
use perceptron::perceptron::*;

fn main() {
    let mut iris_records = Features::new();
    let record_parsing = iris_records.parse_csv("../data/Iris.csv");
    match record_parsing {
        Ok(_) => println!("Successfully parsed csv file"),
        Err(e) => println!("Error: {}", e),
    }
    iris_records.print();
    // retrieve flower classification for all rows
    // create ndarray of flower classification
    // create ndarray of features from iris_records
    // create perceptron
    // train perceptron
    // test perceptron
    // print results
    // plot results
    let mut perceptron = Perceptron::new(None, None);
}
