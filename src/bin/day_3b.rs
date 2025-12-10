use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    fn max(s: &str) -> usize {
        // Find the first largest digit
        let digits: Vec<u8> = s.as_bytes().iter().map(|d| d - '0' as u8).collect();

        let mut num = 0;
        let mut i = 0;
        for position in (0..12).rev() {
            let range = &digits[i..digits.len() - position];
            let max = range.iter().max().unwrap();
            let max_index = range.iter().position(|x| x == max).unwrap() + i;
            num *= 10;
            num += digits[max_index] as usize;
            i = max_index + 1;
        }
        return num;
    }

    let mut res = 0 as u128;
    for line in input.lines() {
        res += max(line) as u128;
    }

    println!("👉 {res}");
}
