pub fn run (dataset: &String) {
    let matrix: Vec<&str> = dataset.split_terminator("\n").collect();
    let mut count_trees = 0;
    let mut pos_x = 0;

    for i in 0..matrix.len() {
        if matrix[i].chars().nth(pos_x % matrix[0].len()).unwrap() == "#".chars().nth(0).unwrap() { count_trees += 1 }
        pos_x += 3;
    }
    println!("{}", count_trees);
}