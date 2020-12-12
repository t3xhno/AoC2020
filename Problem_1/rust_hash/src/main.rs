use std::fs;
use std::collections::HashMap;

fn main () {
    let input = fs::read_to_string("dataset.txt").unwrap();
    let mut hash_map: HashMap<u16, u8> = HashMap::new();
    input.lines().enumerate()
        .map(|x| (x.0 as u8, x.1.parse::<u16>().unwrap()))
        .try_for_each(|x| Some({
            if hash_map.contains_key(&(2020 - x.1)) && *hash_map.get(&(2020 - x.1)).unwrap() != x.0 {
                println!("{}", x.1 * (2020 - x.1));
                return None;
            }
            else {
                hash_map.insert(x.1, x.0);
            }
    }));
}