use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let my_map = reader.lines().enumerate().fold(HashMap::new(), |mut hm1, (i, line)| {
        if i < 20 {
            let mut first = String::new();
            let mut second: HashMap<u16, u16> = HashMap::new();
            line.unwrap().split(":").enumerate().for_each(|(i, side)| {
                match i {
                    0 => first = String::from(side),
                    1 => {
                        let mut ranges: [[u16; 2]; 2] = [[0; 2]; 2];
                        side.split("or").enumerate().for_each(|(z, range_total)| {
                            match z {
                                0 | 1 => range_total.trim().split("-").enumerate().for_each(|(k, r)| {
                                    match k {
                                        0 | 1 => ranges[z][k] = r.parse().unwrap(),
                                        _ => unreachable!()
                                    }
                                }),
                                _ => unreachable!()
                            }
                        });
                        let numbers: Vec<_> = vec![
                            (ranges[0][0]..ranges[0][1]).collect::<Vec<_>>(),
                            (ranges[1][0]..ranges[1][1]).collect::<Vec<_>>()
                        ];
                        let total_range: Vec<u16> = numbers.into_iter().flatten().collect::<Vec<u16>>();
                        second = total_range.into_iter().enumerate().fold(HashMap::new(), |mut hm2, (v, num)| {
                            hm2.insert(num, v as u16);
                            hm2
                        });
                    },
                    _ => unreachable!()
                }
            });
            hm1.insert(first, second);
        }
        hm1
    });
}
