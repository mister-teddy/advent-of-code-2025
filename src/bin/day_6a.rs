use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    let ops: Vec<bool> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|c| c == "*")
        .collect(); // true is *, false is +

    let mut problems: Vec<u128> = ops
        .iter()
        .map(|is_mul| if *is_mul { 1 } else { 0 })
        .collect();
    for line in input.lines().rev().skip(1) {
        line.split_whitespace().enumerate().for_each(|(i, x)| {
            let x: u128 = x.parse().unwrap();
            if ops[i] {
                problems[i] *= x;
            } else {
                problems[i] += x;
            }
        });
    }

    let res: u128 = problems.iter().sum();

    println!("👉 {res}");
}
