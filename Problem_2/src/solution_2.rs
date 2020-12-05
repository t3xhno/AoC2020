use std::fs;

fn check_if_valid (bottom_limit: u16, upper_limit: u16, passletter: char, passphrase: &String) -> bool {
    match (passphrase.chars().nth(usize::from(upper_limit - 1)).unwrap() == passletter,
        passphrase.chars().nth(usize::from(bottom_limit - 1)).unwrap() == passletter) {
            (true, false) => true,
            (false, true) => true,
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