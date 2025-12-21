use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    let mut shapes = vec![];
    let mut inputs = vec![];
    for line in input.lines() {
        if line.ends_with(':') {
            shapes.push(vec![]);
        } else if line.contains('#') {
            shapes
                .last_mut()
                .unwrap()
                .append(&mut line.chars().collect::<Vec<char>>());
        } else if line.contains('x') {
            let (area, counts) = line.split_once(':').unwrap();
            let (w, h) = area.split_once('x').unwrap();
            inputs.push((
                (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap()),
                counts
                    .split(' ')
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            ));
        }
    }

    let mut res = 0;
    for ((w, h), counts) in inputs {
        let total_area: usize = counts
            .iter()
            .enumerate()
            .map(|(i, &count)| {
                let area = &shapes[i].iter().filter(|&&c| c == '#').count();
                count * area
            })
            .sum();
        if total_area <= w * h {
            res += 1
        }
    }

    println!("👉 {res}");
}
