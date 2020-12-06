fn check_keys (keys: &Vec<&str>, keys_const: [&str; 7]) -> bool {
    for (_i, &key) in keys_const.iter().enumerate() {
        if !keys.contains(&key) { return false; }
    }
    true
}

fn check_byr (byr: u16) -> bool {
    if byr <= 2002 && byr >= 1920 { true } else { false }
}

fn check_iyr (iyr: u16) -> bool {
    if iyr <= 2020 && iyr >= 2010 { true } else { false }
}

fn check_eyr (eyr: u16) -> bool {
    if eyr <= 2030 && eyr >= 2020 { true } else { false }
}

pub fn run (dataset: &String) {
    const KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports: Vec<&str> = dataset.split_terminator("\n\n").collect();
    let mut number_of_valid = 0;
    for (_i, &passport) in passports.iter().enumerate() {
        let mut keys: Vec<&str> = vec![];
        let pairs = passport.split_whitespace().collect::<Vec<&str>>();
        for (_j, &pair) in pairs.iter().enumerate() {
            keys.push(pair.split_terminator(":").collect::<Vec<&str>>()[0]);
        }
        if check_keys(&keys, KEYS) { number_of_valid += 1; }
    }
    println!("{}", number_of_valid);
}