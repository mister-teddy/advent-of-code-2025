use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    let mut fresh_ranges = vec![];

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
        fresh_ranges.push((a, b));
    }

    // Sort the range by ascending start

    fresh_ranges.sort_by_key(|r| r.0);

    let mut res = 0;
    let mut start = 1;
    let mut end = 0;
    for range in fresh_ranges {
        if range.0 <= end {
            // merge 2 ranges together
            end = end.max(range.1);
        } else {
            // a gap
            res += end + 1 - start;
            start = range.0;
            end = range.1;
        }
    }
    res += end + 1 - start;

    println!("👉 {res}");
}
