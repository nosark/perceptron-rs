use perceptron::csv_parser::*;

fn main() {
    let mut iris_records = Features::new();
    let record_parsing = iris_records.parse_csv("../data/Iris.csv");   
    match record_parsing {
        Ok(_) => println!("Successfully parsed csv file"),
        Err(e) => println!("Error: {}", e),
    }
    iris_records.print(); 
}
