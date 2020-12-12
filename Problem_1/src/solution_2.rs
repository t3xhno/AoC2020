use std::fs;

pub fn run () {
    let dataset = fs::read_to_string("dataset.txt").unwrap();

    let salaries = dataset.lines()
        .map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

    for i in 0..salaries.len() - 2 {
        for j in i + 1..salaries.len() -1 {
            for k in j + 1..salaries.len() {
                match salaries[i] + salaries[j] + salaries[k] {
                    2020 => println!("{}", salaries[i] * salaries[j] * salaries[k]),
                    _ => ()
                }
            }
        }
    }
}