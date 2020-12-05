fn check_keys (keys: &Vec<&str>, keys_const: [&str; 7]) -> bool {
    for (_i, &key) in keys_const.iter().enumerate() {
        if !keys.contains(&key) { return false; }
    }
    true
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