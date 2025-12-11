use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    // Read from the top right
    let mut res: u128 = 0;

    let mut nums = vec![];
    'a: for i in (0..grid[0].len()).rev() {
        let mut num = 0;
        for j in 0..grid.len() {
            match grid[j][i] {
                ' ' => {}
                '*' => {
                    res += nums.iter().fold(1, |product, x| product * x) * num;
                    nums.clear();
                    continue 'a;
                }
                '+' => {
                    res += nums.iter().sum::<u128>() + num;
                    nums.clear();
                    continue 'a;
                }
                digit => {
                    num *= 10;
                    num += digit as u128 - '0' as u128;
                }
            }
        }
        if num != 0 {
            nums.push(num);
        }
    }

    println!("👉 {res}");
}
