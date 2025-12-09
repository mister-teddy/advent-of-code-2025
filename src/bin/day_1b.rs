use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();
    let mut dial = 50;
    let mut res = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        match (chars.next(), chars.as_str().parse::<i32>()) {
            (Some('L'), Ok(distance)) => {
                while dial < distance {
                    dial += 100;
                }
                res += (dial - 1) / 100;
                if dial == distance {
                    res += 1
                }
                dial -= distance;
            }
            (Some('R'), Ok(distance)) => {
                dial += distance;
                res += dial / 100;
                dial %= 100;
            }
            _ => {}
        }
    }

    println!("👉 {res}");
}
