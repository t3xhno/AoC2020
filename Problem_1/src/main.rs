use std::env;

mod solution_1;
mod solution_2;

fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please enter either program_1 or program_2.");
    } else if args.len() == 2 {
        match args[1].as_str() {
            "program_1" => solution_1::run(),
            "program_2" => solution_2::run(),
            _ => println!("Invalid argument!")
        }
    }
}