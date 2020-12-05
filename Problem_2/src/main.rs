use std::env;

mod solution_1;
mod solution_2;

const FILE_NAME: &str = "dataset.txt";

fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please enter either problem_1 or problem_2.");
    } else if args.len() == 2 {
        match args[1].as_str() {
            "problem_1" => solution_1::run(FILE_NAME),
            "problem_2" => solution_2::run(FILE_NAME),
            _ => println!("Invalid argument!")
        }
    }
}
