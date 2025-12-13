use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut res = 0;
    let mut splitters = vec![vec![false; grid[0].len()]; grid.len()];
    splitters[0][grid[0].iter().position(|x| x == &'S').unwrap()] = true;
    for i in 1..splitters.len() {
        for j in 0..splitters[i].len() {
            if splitters[i - 1][j] {
                match grid[i][j] {
                    '.' => splitters[i][j] = true,
                    '^' => {
                        res += 1;
                        if j > 0 {
                            splitters[i][j - 1] = true;
                        }
                        if j < splitters[i].len() - 1 {
                            splitters[i][j + 1] = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // let res = splitters.last().unwrap().iter().filter(|x| **x).count();

    println!("👉 {res}");
}
