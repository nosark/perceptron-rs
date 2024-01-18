use perceptron::csv_parser::*;
use perceptron::perceptron::*;

fn main() {
    let mut iris_records = Features::new();
    let record_parsing = iris_records.parse_csv("./data/Iris.csv");
    match record_parsing {
        Ok(_) => println!("Successfully parsed csv file"),
        Err(e) => println!("Error: {}", e),
    }
    // Get data from csv [x]
    // create perceptron [x]
    // train perceptron [x]
    // test perceptron [x]
    // print results [x]
    // plot results TODO

    let mut output: Vec<f64> = Vec::new();
    for row in iris_records.data.iter() {
        match row.species.as_str() {
            "Iris-setosa" => output.push(1.0),
            "Iris-versicolor" => output.push(-1.0),
            "Iris-virginica" => continue,
            _ => println!("Error: {} not found", row.species),
        }
    }

    let mut percy = Perceptron::new(None, None, &iris_records.data);
    let percy = percy.fit(&iris_records.data, &output);

    println!("{:?}", percy.error_count);
}
