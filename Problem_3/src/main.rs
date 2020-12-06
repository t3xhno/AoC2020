use std::fs;
use std::env;

mod solution_1;
mod solution_2;

const FILE_NAME: &str = "dataset.txt";

fn main () {
    let dataset = fs::read_to_string(FILE_NAME).expect("Something went wrong...");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please enter either problem_1 or problem_2.");
    } else if args.len() == 2 {
        match args[1].as_str() {
            "problem_1" => solution_1::run(&dataset),
            "problem_2" => solution_2::run(&dataset),
            _ => println!("Invalid argument!")
        }
    }
}