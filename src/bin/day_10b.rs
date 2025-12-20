use advent_of_code_2025::get_input;
use std::collections::HashMap;

#[derive(Clone)]
struct JoltageLevels(Vec<usize>);

impl From<&str> for JoltageLevels {
    fn from(value: &str) -> Self {
        Self(
            value
                .chars()
                .skip_while(|&c| c == '{')
                .take_while(|&c| c != '}')
                .collect::<String>()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>(),
        )
    }
}

struct Button(Vec<usize>);

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        Self(
            value
                .chars()
                .skip_while(|&c| c == '(')
                .take_while(|&c| c != ')')
                .collect::<String>()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>(),
        )
    }
}

#[derive(Clone)]
struct PressCombo {
    press: Vec<i32>,
    len: usize,
}

struct Machine {
    desired_voltages: JoltageLevels,
    press_patterns: HashMap<Vec<i32>, Vec<PressCombo>>,
}

impl Machine {
    fn init(buttons: Vec<Button>, desired_voltages: JoltageLevels) -> Self {
        let num_buttons = buttons.len();
        let num_lights = desired_voltages.0.len();

        let mut press_patterns: HashMap<Vec<i32>, Vec<PressCombo>> = HashMap::new();
        let total_combos = 1usize << num_buttons;

        for mask in 0..total_combos {
            let mut press = vec![0_i32; num_lights];
            let mut len = 0usize;

            for btn_idx in 0..num_buttons {
                if (mask & (1 << btn_idx)) != 0 {
                    len += 1;
                    for &light_idx in &buttons[btn_idx].0 {
                        press[light_idx] += 1;
                    }
                }
            }

            let pattern: Vec<i32> = press.iter().map(|p| p.rem_euclid(2)).collect();
            press_patterns.entry(pattern).or_default().push(PressCombo {
                press: press.clone(),
                len,
            });
        }

        Self {
            desired_voltages,
            press_patterns,
        }
    }

    fn cost(&mut self, jolts: Vec<i32>) -> usize {
        if !jolts.iter().any(|&j| j != 0) {
            return 0;
        }

        let base_cost: usize = self.desired_voltages.0.iter().sum();

        if jolts.iter().any(|&j| j < 0) {
            return base_cost;
        }

        let pattern: Vec<i32> = jolts.iter().map(|&j| j.rem_euclid(2)).collect();

        let combos = self.press_patterns.get(&pattern).cloned();
        let res = if let Some(combos) = combos {
            combos
                .iter()
                .map(|combo| {
                    let next: Vec<i32> = jolts
                        .iter()
                        .zip(combo.press.iter())
                        .map(|(a, b)| (a - b).div_euclid(2))
                        .collect();
                    combo.len + 2 * self.cost(next)
                })
                .min()
                .unwrap_or(base_cost)
        } else {
            base_cost
        };

        res
    }

    fn solve(mut self) -> usize {
        let start = self
            .desired_voltages
            .0
            .iter()
            .map(|&v| v as i32)
            .collect::<Vec<i32>>();
        self.cost(start)
    }
}

fn main() {
    let input = get_input();
    let mut machines = vec![];
    for line in input.lines() {
        let (_, buttons) = line.split_once(' ').unwrap();
        let (buttons, voltages) = buttons.rsplit_once(' ').unwrap();
        let desired_voltages = voltages.into();
        let buttons = buttons
            .split(' ')
            .take_while(|part| part.starts_with('('))
            .map(|part| part.into())
            .collect();
        machines.push(Machine::init(buttons, desired_voltages));
    }

    let res: usize = machines.into_iter().map(|m| m.solve()).sum();
    println!("👉 {res}");
}
