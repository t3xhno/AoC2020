use std::fs;

fn main() {
    let input = fs::read_to_string("dataset.txt").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    input.lines().for_each(|line| {
        let mut range: [u8; 2] = [0, 0];
        let mut letter: char = ' ';
        let mut password: String = String::new();
        let mut first: char = ' ';
        let mut second: char = ' ';
        line.split(" ").enumerate().for_each(|(i, s)| {
            match i {
                0 => s.split("-").enumerate().for_each(|(i, n)| range[i] = n.parse().unwrap()),
                1 => letter = s.split(":").nth(0).unwrap().parse().unwrap(),
                2 => password = s.parse().unwrap(),
                _ => unreachable!()
            }
        });
        let letter_count: u8 = password.chars().enumerate().fold(0, |acc, (i, c)| {
            if i as u8 == range[0] - 1 { first = c } else if i as u8 == range[1] - 1 { second = c };
            if c == letter { acc + 1 } else { acc }
        });
        if letter_count >= range[0] && letter_count <= range[1] { part1 += 1 };
        match (first == letter, second == letter) {
            (true, false) | (false, true) => part2 += 1,
            _ => ()
        }
    });
    println!("1: {}\n2: {}", part1, part2);
}