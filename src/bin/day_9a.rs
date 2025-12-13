use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    let mut tiles = vec![];
    for line in input.lines() {
        let nums = line
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        tiles.push((nums[0], nums[1]));
    }

    let mut res = 0;
    for (x1, y1) in &tiles {
        for (x2, y2) in &tiles {
            res = res.max((1 + x1.abs_diff(*x2)) * (1 + y1.abs_diff(*y2)))
        }
    }

    println!("👉 {res}");
}
