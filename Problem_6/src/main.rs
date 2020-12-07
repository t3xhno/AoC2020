use std::env;
use std::fs;

mod solution_1;
mod solution_2;

const FILE_NAME: &str = "dataset.txt";

fn main () {
    let args = env::args().collect::<Vec<String>>();
    let dataset: String = fs::read_to_string(FILE_NAME).expect("Something went wrong...");
    if args.len() != 2 {
        println!("Enter problem_1 or problem_2.");
    }
    else {
        match args[1].as_str() {
            "problem_1" => solution_1::run(&dataset),
            "problem_2" => solution_2::run(&dataset),
            _ => println!("Invalid argument.")
        }
    }
}