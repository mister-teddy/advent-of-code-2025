use advent_of_code_2025::get_input;
use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let input = get_input();

    let mut boxes: Vec<(usize, usize, usize)> = vec![];
    for line in input.lines() {
        let nums = line
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        boxes.push((nums[0], nums[1], nums[2]));
    }

    let distance_sqr = |i: usize, j: usize| -> usize {
        let (x1, y1, z1) = boxes[i];
        let (x2, y2, z2) = boxes[j];
        x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2) // Not taking sqrt here to keep distances as usize, so that they can be compared (floating points are not comparable in Rust)
    };

    let mut distances = BinaryHeap::new();

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.push(Reverse((distance_sqr(i, j), i, j)));
        }
    }

    let mut group_count = 0;
    let mut groups = vec![0; boxes.len()];

    let mut connect = || {
        let &mut Reverse((d, i, j)) = &mut distances.pop().unwrap();
        match (groups[i], groups[j]) {
            (0, 0) => {
                // Asign a new group
                group_count += 1;
                groups[i] = group_count;
                groups[j] = group_count;
            }
            (0, g) => {
                groups[i] = g;
            }
            (g, 0) => {
                groups[j] = g;
            }
            (g, old) => {
                // Assign all boxes with old group to new group
                for i in 0..groups.len() {
                    if groups[i] == old {
                        groups[i] = g;
                    }
                }
            }
        };
    };

    for _i in 0..1000 {
        connect();
    }

    let res = groups
        .into_iter()
        .filter(|g| *g != 0)
        .counts()
        .iter()
        .k_largest_by(3, |a, b| a.1.cmp(b.1))
        .map(|(_, size)| size)
        .fold(1, |prod, x| prod * x);

    println!("👉 {res}");
}
