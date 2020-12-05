use std::fs;

fn count_occurencies (letter: char, my_string: &String) -> u16 {
    let mut occurencies: u16 = 0;
    for i in 0..my_string.len() {
        if my_string.chars().nth(i).unwrap() == letter {
            occurencies += 1;
        }
    }
    occurencies
}

fn check_if_valid (bottom_limit: u16, upper_limit: u16, passletter: char, passphrase: &String) -> bool {
    match (count_occurencies(passletter, &passphrase) >= bottom_limit,
        count_occurencies(passletter, &passphrase) <= upper_limit) {
            (true, true) => true,
            _ => false
        }
}

pub fn run (file_name: &str) {
    let mut number_of_valid: u16 = 0;
    let dataset: String = fs::read_to_string(file_name).expect("Something went wrong!");
    for password in dataset.split_terminator("\n").collect::<Vec<&str>>() {
        let bottom_limit: u16 = password.split_whitespace().collect::<Vec<&str>>()[0]
            .split_terminator("-").nth(0).unwrap().parse().unwrap();
        let upper_limit: u16 = password.split_whitespace().collect::<Vec<&str>>()[0]
            .split_terminator("-").nth(1).unwrap().parse().unwrap();
        let passletter = String::from(password.split_whitespace().collect::<Vec<&str>>()[1]
        .chars().nth(0).unwrap());
        let passphrase = String::from(password.split_whitespace().collect::<Vec<&str>>()[2]);
        if check_if_valid(bottom_limit, upper_limit, passletter.chars().nth(0).unwrap(), &passphrase) {
            number_of_valid += 1;
        }
    }
    println!("{}", number_of_valid);
}