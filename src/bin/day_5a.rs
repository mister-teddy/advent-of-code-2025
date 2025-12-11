use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    let mut fresh_ranges = vec![];
    let mut part_1 = true;
    let mut res = 0;

    for line in input.lines() {
        if line.is_empty() {
            part_1 = false;
            continue;
        }
        if part_1 {
            let (a, b) = line.split_once("-").unwrap();
            let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
            fresh_ranges.push((a, b));
        } else {
            let id = line.parse::<usize>().unwrap();
            for (a, b) in fresh_ranges.iter() {
                if id >= *a && id <= *b {
                    res += 1;
                    break;
                }
            }
        }
    }

    println!("👉 {res}");
}
