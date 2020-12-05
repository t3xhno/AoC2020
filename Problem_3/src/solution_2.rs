pub fn run (dataset: String) {
    let matrix: Vec<&str> = dataset.split_terminator("\n").collect();
    const STEPS: [(u8, u8);5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result: u64 = 1;

    for step in STEPS.iter() {
        let mut pos_x = 0;
        let mut count_trees = 0;
        for i in (0..matrix.len()).step_by(usize::from(step.1)) {
            if matrix[i].chars().nth(pos_x % matrix[0].len()).unwrap() == "#".chars().nth(0).unwrap() {
                count_trees += 1;
            }
            pos_x += usize::from(step.0);
        }
        result *= count_trees;
    }
    println!("{}", result);
}