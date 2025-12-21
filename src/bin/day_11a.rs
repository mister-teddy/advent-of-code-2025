use std::collections::HashMap;

use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    let mut adjs: Vec<Vec<usize>> = vec![vec![]]; // Instead of "aaa", we store 0, and keep track of the index of "you"
    let mut index_map = HashMap::new();
    let mut you = 0;
    let mut out = 0;
    index_map.insert("out", out);
    for line in input.lines() {
        let (from, _to) = line.split_once(':').unwrap();
        let index = index_map.len();
        index_map.insert(from, index);
        match from {
            "you" => you = index,
            "out" => out = index,
            _ => {}
        }
    }
    for line in input.lines() {
        let (_from, to) = line.split_once(':').unwrap();
        adjs.push(
            to.split(' ')
                .filter_map(|device| index_map.get(device).map(|index| *index))
                .collect(),
        );
    }

    fn solve(adjs: &Vec<Vec<usize>>, from: usize, to: usize) -> usize {
        if from == to {
            return 1; // There is one way, is to go no further
        }
        // if adjs[from].is_empty() {
        //     return 0; // It is impossible to reach the destination from here
        // }
        adjs[from].iter().map(|next| solve(adjs, *next, to)).sum()
    }

    let res = solve(&adjs, you, out);

    println!("👉 {res}");
}
