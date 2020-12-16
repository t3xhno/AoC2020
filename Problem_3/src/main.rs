use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("dataset.txt").unwrap());
    let slopes: [(u8, u8); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let (mut part1, mut part2) = (0u16, 1u64);
    let mut forest: [[char;31]; 323] = [[' ';31]; 323];
    reader.lines().enumerate().for_each(|(i, line)| {
        line.unwrap().chars().enumerate().for_each(|(j, c)| forest[i][j] = c)
    });
    slopes.iter().enumerate().for_each(|(i, slope)| {
        let (mut pos_x, mut pos_y) = (0u16, 0u8);
        let mut tree_count = 0;
        while pos_x < 323 {
            match forest[pos_x as usize][pos_y as usize] {
                '#' => tree_count += 1,
                _ => ()
            }
            pos_y = (pos_y + slope.0) % 31;
            pos_x += slope.1 as u16;
        }
        if i == 1 { part1 = tree_count }
        part2 *= tree_count as u64;
    });
    println!("1: {}\n2: {}", part1, part2);
}
