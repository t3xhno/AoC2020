use std::fs;

const FILE_NAME: &str = "dataset.txt";

pub fn run () {
    let dataset = fs::read_to_string(FILE_NAME).expect("Something went wrong!");
    let mut salaries: Vec<u32> = vec![];

    for word in dataset.split_whitespace() {
        salaries.push(word.parse::<u32>().unwrap());
    }

    for i in 0..salaries.len() - 2 {
        for j in i + 1..salaries.len() -1 {
            for k in j + 1..salaries.len() {
                match salaries[i] + salaries[j] + salaries[k] {
                    2020 => println!("{}", salaries[i] * salaries[j] * salaries[k]),
                    _ => ()
                }
            }
        }
    }
}