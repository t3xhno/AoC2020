use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: &str) -> Vec<i64> {
    input.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn sum_of_two_is(sum: i64, vec: &[i64]) -> bool {
    vec.iter().enumerate()
        .flat_map(|(i, a)| vec.iter().skip(i+1).map(move |b| a + b))
        .find(|x| *x == sum)
        .is_some()
}

fn find_first_invalid(vec: &Vec<i64>, preamble: usize) -> Option<i64> {
    let index = (preamble..vec.len()).find(
        |index| !sum_of_two_is(vec[*index], &vec[index-preamble..*index])
    );

    index.map(|i| vec[i])
}

fn find_contiguous_set_summing_to(target: i64, vec: &[i64]) -> Option<&[i64]> {
    let mut start = 0;
    let mut end = 1;
    let mut sum: i64 = vec[start..=end].iter().sum();

    while end < vec.len() {
        if sum == target {
            return Some(&vec[start..=end]);
        }
        if sum > target {
            sum -= vec[start];
            start += 1;
        } else {
            end += 1;
            sum += vec[end];
        }
    }

    None
}

fn sum_of_min_and_max(vec: &[i64]) -> Option<i64> {
    vec.iter().min()
        .and_then(|min|
            vec.iter().max().map(|max| 
                min + max
            )
        )
}

fn part1(sequence: &Vec<i64>) -> Option<i64> {
    find_first_invalid(sequence, 25)
}

fn part2(sequence: &Vec<i64>) -> Option<i64> {
    let target = part1(sequence).unwrap();
    find_contiguous_set_summing_to(target, sequence)
        .and_then(sum_of_min_and_max)
}   


fn main() {
    let input = read_file("dataset").unwrap();
    let sequence = parse_input(&input);
    println!("{:?}", part1(&sequence));
    println!("{:?}", part2(&sequence));
}