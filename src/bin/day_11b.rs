use std::collections::HashMap;

use advent_of_code_2025::get_input;

#[derive(PartialEq, Eq, Hash)]
struct Origin {
    from: usize,
    visited_fft: bool,
    visited_dac: bool,
}

fn main() {
    let input = get_input();

    let mut adjs: Vec<Vec<usize>> = vec![vec![]]; // Instead of "aaa", we store 0, and keep track of the index of "you"
    let mut index_map = HashMap::new();
    let mut svr = 0;
    let mut out = 0;
    index_map.insert("out", out);
    for line in input.lines() {
        let (from, _to) = line.split_once(':').unwrap();
        let index = index_map.len();
        index_map.insert(from, index);
        match from {
            "svr" => svr = index,
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

    fn solve(
        adjs: &Vec<Vec<usize>>,
        index_map: &HashMap<&str, usize>,
        origin: Origin,
        to: usize,
        cache: &mut HashMap<Origin, usize>, // Without cache, it takes forever
    ) -> usize {
        if let Some(&res) = cache.get(&origin) {
            return res;
        }
        let Origin {
            from,
            visited_fft,
            visited_dac,
        } = origin;
        if from == to {
            return if visited_dac && visited_fft { 1 } else { 0 };
        }
        let res = adjs[from]
            .iter()
            .map(|next| {
                solve(
                    adjs,
                    index_map,
                    Origin {
                        from: *next,
                        visited_fft: visited_fft || origin.from == index_map["fft"],
                        visited_dac: visited_dac || origin.from == index_map["dac"],
                    },
                    to,
                    cache,
                )
            })
            .sum();

        cache.insert(origin, res);

        res
    }

    let res = solve(
        &adjs,
        &index_map,
        Origin {
            from: svr,
            visited_fft: false,
            visited_dac: false,
        },
        out,
        &mut HashMap::new(),
    );

    println!("👉 {res}");
}
