pub fn run (dataset: &String) {
    let mut total: u16 = 0;
    for group in dataset.split("\n\n") {
        let mut group_ans: [u8; 26] = [0; 26];
        for person in group.split("\n") {
            for c in person.chars() {
                group_ans[(c as u8 - 97) as usize] += 1;
            }
        }
        for i in 0..26 {
            if group_ans[i] > 0 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}