use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut splitters = vec![vec![0 as u128; grid[0].len()]; grid.len()];
    splitters[0][grid[0].iter().position(|x| x == &'S').unwrap()] = 1;
    for i in 1..splitters.len() {
        for j in 0..splitters[i].len() {
            match grid[i][j] {
                '.' => splitters[i][j] += splitters[i - 1][j],
                '^' => {
                    if j > 0 {
                        splitters[i][j - 1] += splitters[i - 1][j];
                    }
                    if j < splitters[i].len() - 1 {
                        splitters[i][j + 1] += splitters[i - 1][j];
                    }
                }
                _ => {}
            }
        }
    }

    let res: u128 = splitters.last().unwrap().iter().sum();

    println!("👉 {res}");
}
