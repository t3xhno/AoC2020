fn add (a: u8, b: u8) -> u8 {
    a + b
}

fn main () {
    println!("{}", add(150, 105));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add () {
        const CASES: [(u8, u8, u8); 3] = [(15, 53, 68), (220, 32, 252), (0, 0, 0)];
        for case in CASES.iter() {
            assert_eq!(add(case.0, case.1), case.2);
        }
    }

    #[test]
    #[should_panic]
    fn test_add_panic () {
        const CASES: [(u8, u8); 1] = [(255, 1)];
        for case in CASES.iter() {
            add(case.0, case.1);
        }
    }
}