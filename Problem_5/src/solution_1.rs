pub fn run (dataset: &String) {
    let mut max: (u8, u8) = (0, 0);
    for line in dataset.lines() {
        if max.0 < find_row(&line[..7]) {
            max.0 = find_row(&line[..7]);
            max.1 = find_col(&line[7..]);
        }
        else if max.0 == find_row(&line[..7]) {
            if max.1 < find_col(&line[7..]) {
                max.1 = find_col(&line[7..]);
            } else {}
        } else {}
    }
    println!("{}", max.0 as u16 * 8 + max.1 as u16);
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