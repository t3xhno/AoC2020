use std::fs;

const FILE_NAME: &str = "dataset.txt";

fn main () {
    let dataset = match fs::read_to_string(FILE_NAME) {
        Ok(value) => value,
        Err(e) => e.to_string()
    };
    let mut salaries: Vec<u16> = vec![];

    for word in dataset.split_whitespace() {
        salaries.push(word.parse::<u16>().unwrap());
    }

    for i in 0..salaries.len() - 1 {
        for j in i + 1..salaries.len() {
            match salaries[i] + salaries[j] {
                2020 => {
                    println!("{}", salaries[i] * salaries[j]);
                },
                _ => (),
            }
        }
    }
}