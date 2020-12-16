use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main () {
    let reader = BufReader::new(File::open("dataset.txt").unwrap());
    let mut hash_map: HashMap<u16, u8> = HashMap::new();

    reader.lines().enumerate()
        .map(|(i, line)| (i as u8, line.unwrap().parse::<u16>().unwrap()))
        .try_for_each(|(i, line)| Some({
            match hash_map.contains_key(&(2020 - line)) && *hash_map.get(&(2020 - line)).unwrap() != i {
                true => {
                    println!("{}", line * (2020 - line));
                },
                _ => {
                    hash_map.insert(line, i);
                }
            }
        }));
}