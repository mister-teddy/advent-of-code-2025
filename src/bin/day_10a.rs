use std::ops::BitXor;

use advent_of_code_2025::get_input;

struct IndicatorLight(String);

impl Into<usize> for IndicatorLight {
    fn into(self) -> usize {
        let mut res = 0;
        let chars = self.0.chars();
        for c in chars.rev().skip(1).take_while(|&c| c != '[') {
            res *= 2;
            if c == '#' {
                res += 1;
            }
        }
        res
    }
}

struct Button(String);

impl Into<usize> for Button {
    fn into(self) -> usize {
        let mut res = 0;
        for num in self.0.split(',') {
            let num: String = num
                .chars()
                .skip_while(|&c| c == '(')
                .take_while(|&c| c != ')')
                .collect();
            let num: u32 = num.parse().unwrap();
            res += 2_i32.pow(num);
        }
        res as usize
    }
}

struct Machine {
    size: usize,
    desired_state: usize,
    buttons: Vec<usize>,
}

impl Machine {
    fn min_presses(&self) -> usize {
        // We can solve this problem with dynamic programming
        // Let f[i, state] be the fewest button presses required to put the machine into state `state` using `buttons[0..i]`
        //
        // So, for each button, we have 2 options:
        // - Do nothing: state doesn't change => f[i, state] = f[i-1][state]
        // - Press it:   state changes from XOR(state) => f[i, state] = f[i-1, XOR(state)] + 1
        //
        // Originally, f[0, 0] = 0, f[0, *] = +Inf
        let total_state = 2i32.pow(self.size as u32) as usize;
        let mut f = vec![vec![usize::MAX; total_state]; self.buttons.len() + 1];
        f[0][0] = 0;

        for i in 1..=self.buttons.len() {
            for state in 0..total_state {
                f[i][state] = f[i - 1][state]
                    .min(f[i - 1][state.bitxor(self.buttons[i - 1])].saturating_add(1));
            }
            println!(
                "f[{i}]: {:?}",
                f[i].iter()
                    .map(|&x| if x == usize::MAX {
                        "∞".to_string()
                    } else {
                        x.to_string()
                    })
                    .collect::<Vec<String>>()
            );
        }
        f.iter().map(|x| x[self.desired_state]).min().unwrap()
    }
}

fn main() {
    let input = get_input();

    // We can store the "states" of the indicator lights as 1 single number
    // For example: .###.# is 011101, which in base 10 is 1+0+4+8+16+0 = 29
    // Each button can also be expressed as a number, for example, (1,3) is 01010 == 0+2+0+8+0 = 10
    // So for each line of input (machine), we will have a length (usize), a desired state (usize), and a list of buttons (Vec<usize>)

    let mut machines = vec![];
    for line in input.lines() {
        let (lights, buttons) = line.split_once(' ').unwrap();
        let size = lights.len() - 2;
        let desired_state = IndicatorLight(lights.to_string()).into();
        let buttons = buttons
            .split(' ')
            .take_while(|part| part.starts_with('('))
            .map(|part| Button(part.to_string()).into())
            .collect();
        machines.push(Machine {
            size,
            desired_state,
            buttons,
        });
    }
    // Finished input

    // Start calculation
    let res: usize = machines.iter().map(|m| m.min_presses()).sum();
    println!("👉 {res}");
}
