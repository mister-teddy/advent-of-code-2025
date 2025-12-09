use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();
    let mut dial = 50;
    let mut res = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        match (chars.next(), chars.as_str().parse::<i32>()) {
            (Some('L'), Ok(distance)) => dial -= distance,
            (Some('R'), Ok(distance)) => dial += distance,
            _ => {}
        }
        while dial < 0 {
            dial += 100
        }
        while dial > 99 {
            dial = dial % 100;
        }
        println!("{dial}");
        if dial == 0 {
            res += 1
        }
    }

    println!("👉 {res}");
}
