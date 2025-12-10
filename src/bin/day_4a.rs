use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    let mut grid: Vec<Vec<bool>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().map(|c| c == '@').collect());
    }

    let count_surrounding = |x: usize, y: usize| -> usize {
        let mut res = 0;
        for i in (if x == 0 { 0 } else { x - 1 })..=(x + 1).min(grid.len() - 1) {
            for j in (if y == 0 { 0 } else { y - 1 })..=(y + 1).min(grid[0].len() - 1) {
                if grid[i][j] && ((i != x) || (j != y)) {
                    res += 1;
                }
            }
        }
        res
    };

    let mut res = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] && count_surrounding(i, j) < 4 {
                res += 1;
            }
        }
    }

    println!("👉 {res}");
}
