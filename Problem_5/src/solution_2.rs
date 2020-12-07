pub fn run (dataset: &String) {
    let mut seat_ids: [u8; 127 * 8 + 7] = [0; 127 * 8 + 7];
    for line in dataset.lines() {
        seat_ids[find_row(&line[..7]) as usize * 8 + find_col(&line[7..]) as usize] = 1;
    }
    for i in 1..seat_ids.len() - 1 {
        match (seat_ids[i - 1], seat_ids[i], seat_ids[i + 1]) {
            (1, 0, 1) => println!("{}", i),
            _ => ()
        }
    }
    // println!("{:?}", seat_ids);
}
/* ASSOCIATED FUNCTIONS */
fn find_row (s: &str) -> u8 {
    let mut row: u8 = 127;
    for (i, c) in s.chars().enumerate() {
        match c {
            'F' => row -= u8::pow(2, 6 - i as u32),
            _ => ()
        }
    }
    u8::from(row)
}
fn find_col (s: &str) -> u8 {
    let mut col: u8 = 7;
    for (i, c) in s.chars().enumerate() {
        match c {
            'L' => col -= u8::pow(2, 2 - i as u32),
            _ => ()
        }
    }
    col
}