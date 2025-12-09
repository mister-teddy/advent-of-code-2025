use advent_of_code_2025::get_input;

fn main() {
    let input = get_input();

    fn is_invalid(x: usize) -> bool {
        let s = x.to_string();
        if s.len() % 2 == 1 {
            return false;
        }
        // overkill
        'a: for size in s.len() / 2..=s.len() / 2 {
            // Split s into chunks of length len
            let chunks: Vec<&[u8]> = s.as_bytes().chunks(size).collect();
            // Check if all chunks are the same
            for chunk in &chunks {
                if chunk != &chunks[0] {
                    continue 'a;
                }
            }
            return true;
        }
        return false;
    }

    let mut res = 0;
    for line in input.split(",") {
        let (start, end) = line
            .split_once("-")
            .map(|(start, end)| {
                (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                )
            })
            .unwrap();
        res += (start..=end)
            .filter(|&x| is_invalid(x))
            .map(|x| {
                println!("❌ {x}");
                x
            })
            .sum::<usize>();
    }

    println!("👉 {res}");
}
