use std::fs;

const FILE_NAME: &str = "dataset.txt";

fn main () {
    let dataset = fs::read_to_string(FILE_NAME)
        .expect("Smething went wrong!");
    println!("{}", dataset);
}