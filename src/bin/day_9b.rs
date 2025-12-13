use std::collections::VecDeque;

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
    tiles.push(tiles[0]); // The list wraps, so the first red tile is also connected to the last red tile.

    let m = tiles.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let n = tiles.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let mut grid = vec![vec![false; n]; m];

    grid[tiles[0].0][tiles[0].1] = true;
    for i in 1..tiles.len() {
        grid[tiles[i].0][tiles[i].1] = true;
        if tiles[i - 1].0 == tiles[i].0 {
            for y in tiles[i - 1].1.min(tiles[i].1)..=tiles[i - 1].1.max(tiles[i].1) {
                grid[tiles[i].0][y] = true;
            }
        } else {
            for x in tiles[i - 1].0.min(tiles[i].0)..=tiles[i - 1].0.max(tiles[i].0) {
                grid[x][tiles[i].1] = true;
            }
        }
    }

    // flood-fill from outside to find all outside false cells
    let mut visited = vec![vec![false; n]; m];
    let mut q = VecDeque::new();

    for x in 0..m {
        if !grid[x][0] && !visited[x][0] {
            visited[x][0] = true;
            q.push_back((x, 0));
        }
        if !grid[x][n - 1] && !visited[x][n - 1] {
            visited[x][n - 1] = true;
            q.push_back((x, n - 1));
        }
    }
    for y in 0..n {
        if !grid[0][y] && !visited[0][y] {
            visited[0][y] = true;
            q.push_back((0, y));
        }
        if !grid[m - 1][y] && !visited[m - 1][y] {
            visited[m - 1][y] = true;
            q.push_back((m - 1, y));
        }
    }

    while let Some((x, y)) = q.pop_front() {
        let neigh = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];
        for &(nx, ny) in &neigh {
            if nx < m && ny < n && !grid[nx][ny] && !visited[nx][ny] {
                visited[nx][ny] = true;
                q.push_back((nx, ny));
            }
        }
    }

    // fill inside cells
    for x in 0..m {
        for y in 0..n {
            if !visited[x][y] {
                grid[x][y] = true;
            }
        }
    }

    let mut areas = vec![vec![0; n]; m];
    areas[0][0] = if grid[0][0] { 1 } else { 0 };
    for i in 1..m {
        areas[i][0] = areas[i - 1][0] + if grid[i][0] { 1 } else { 0 };
    }
    for j in 1..n {
        areas[0][j] = areas[0][j - 1] + if grid[0][j] { 1 } else { 0 };
    }
    for i in 1..m {
        for j in 1..n {
            areas[i][j] = areas[i - 1][j] + areas[i][j - 1] - areas[i - 1][j - 1]
                + if grid[i][j] { 1 } else { 0 };
        }
    }

    let mut res = 0;
    for i in 0..tiles.len() {
        let (x1, y1) = tiles[i];
        for j in i + 1..tiles.len() {
            let (x2, y2) = tiles[j];
            let area = (1 + x1.abs_diff(x2)) * (1 + y1.abs_diff(y2));
            if area > res {
                let min_x = x1.min(x2);
                let max_x = x1.max(x2);
                let min_y = y1.min(y2);
                let max_y = y1.max(y2);
                let valid = areas[max_x][max_y]
                    + if min_x == 0 || min_y == 0 {
                        0
                    } else {
                        areas[min_x - 1][min_y - 1]
                    }
                    - if min_x == 0 {
                        0
                    } else {
                        areas[min_x - 1][max_y]
                    }
                    - if min_y == 0 {
                        0
                    } else {
                        areas[max_x][min_y - 1]
                    }
                    == area;

                if valid {
                    res = area
                }
            }
        }
    }

    println!("👉 {res}");
}
