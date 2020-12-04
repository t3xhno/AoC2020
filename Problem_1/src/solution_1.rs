use std::fs;

const FILE_NAME: &str = "dataset.txt";

pub fn run () {
    let dataset = fs::read_to_string(FILE_NAME).expect("Something went wrong!");
    let mut salaries: Vec<u16> = vec![];

    for word in dataset.split_whitespace() {
        salaries.push(word.parse::<u16>().unwrap());
    }

    for i in 0..salaries.len() - 1 {
        for j in i + 1..salaries.len() {
            match salaries[i] + salaries[j] {
                2020 => println!("{}", salaries[i] * salaries[j]),
                _ => ()
            }
        }
    }
}