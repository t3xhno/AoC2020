use std::fs;
use std::collections::HashMap;

fn two_sum_hash (nums: Vec<u16>, target: u16) {
    let mut num_map: HashMap<u16, u16> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        num_map.insert(*num, i as u16);
    }
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = num_map.get(&complement) {
            if index != i as u16 {
                println!("{}", complement as u32 * *num as u32);
            }
        }
    }
}

const FILE_NAME: &str = "dataset.txt";

pub fn run () {
    let dataset = fs::read_to_string(FILE_NAME).expect("Something went wrong!");

    let salaries = dataset.lines()
        .map(|x| x.parse().unwrap()).collect::<Vec<u16>>();
    two_sum_hash(salaries, 2020);
}