use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    fn max(s: &str) -> u8 {
        // Find the first largest digit
        let digits: Vec<u8> = s.as_bytes().iter().map(|d| d - '0' as u8).collect();
        let first = digits.iter().take(s.len() - 1).max().unwrap();
        let last = digits
            .iter()
            .skip(digits.iter().position(|d| d == first).unwrap() + 1)
            .max()
            .unwrap();
        return first * 10 + last;
    }

    let mut res = 0 as usize;
    for line in input.lines() {
        res += max(line) as usize;
    }

    println!("👉 {res}");
}
