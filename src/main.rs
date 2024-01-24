use perceptron::dataset::*;
use perceptron::perceptron::*;
fn main() {
    let mut iris_dataset = Dataset::new();
    let record_parsing = iris_dataset.parse_csv("./data/Iris.csv");
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
    // TODO: add multiclass classification

    iris_dataset.create_random_sample(50);
    iris_dataset.create_target_from_sample();
    let rand_sample = iris_dataset.get_random_sample();
    let target_output = iris_dataset.get_target_output();
    let mut percy = Perceptron::new(None, Some(10), &rand_sample);
    let percy = percy.fit(&rand_sample, &target_output);

    println!("{:?}", percy.get_error_count());
}
