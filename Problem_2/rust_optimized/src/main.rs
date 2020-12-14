use std::fs;

fn main () {
    let input = fs::read_to_string("dataset.txt").unwrap();
    let mut split_line = input.lines().map(|line| line.split(" ")).collect::<Vec<_>>();
    println!("{:?}", split_line);
    // let line2 = split_line.map(|mut x| (x.nth(0).unwrap(), x.nth(1).unwrap(), x.nth(2).unwrap()))
    //     .collect::<Vec<(&str, &str, &str)>>();
    // println!("{:?}", line2);
}