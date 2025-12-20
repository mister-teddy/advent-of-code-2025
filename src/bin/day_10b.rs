use advent_of_code_2025::get_input;
use std::{collections::HashMap, ops::BitXor};

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

impl Button {
    fn as_state(&self) -> usize {
        let mut res = 0;
        for num in &self.0 {
            res += 2_i32.pow(*num as u32);
        }
        res as usize
    }
}

struct Machine {
    buttons: Vec<Button>,
    desired_voltages: JoltageLevels,
    cache: HashMap<(Vec<usize>, usize), usize>,
    solvable_states: HashMap<usize, Vec<usize>>,
}

impl Machine {
    fn init(buttons: Vec<Button>, desired_voltages: JoltageLevels) -> Self {
        let machine = Self {
            buttons,
            desired_voltages,
            cache: HashMap::new(),
            solvable_states: HashMap::new(),
        };
        let solvable_states: HashMap<usize, Vec<usize>> = (1..2_usize
            .pow(machine.desired_voltages.0.len() as u32))
            .filter_map(|state| {
                machine
                    .button_presses(state)
                    .map(|presses| (state, presses))
            })
            .collect();
        Self {
            buttons: machine.buttons,
            desired_voltages: machine.desired_voltages,
            cache: HashMap::new(),
            solvable_states,
        }
    }

    fn button_presses(&self, desired_state: usize) -> Option<Vec<usize>> {
        // Check day_10a for explanation
        // We also add a new table to trace back the button presses
        let size = self.desired_voltages.0.len();
        let total_state = 2i32.pow(size as u32) as usize;

        let mut trace = vec![vec![false; total_state]; self.buttons.len() + 1];

        let mut f = vec![vec![usize::MAX; total_state]; self.buttons.len() + 1];
        f[0][0] = 0;

        for i in 1..=self.buttons.len() {
            for state in 0..total_state {
                let without_press = f[i - 1][state];
                let with_press =
                    f[i - 1][state.bitxor(self.buttons[i - 1].as_state())].saturating_add(1);
                if with_press < without_press {
                    trace[i - 1][state] = true;
                }
                f[i][state] = without_press.min(with_press);
            }
        }

        let mut button_presses = vec![0; self.buttons.len()];
        let mut current_state = desired_state;
        while current_state != 0 {
            let min_index = f
                .iter()
                .map(|row| row[current_state])
                .enumerate()
                .min_by_key(|&(_, val)| val)
                .unwrap()
                .0;

            if min_index == 0 {
                // All Infinity, no solution
                return None;
            }

            let min_index = min_index - 1; // Adjust for the extra row in f
            button_presses[min_index] += 1;
            current_state ^= self.buttons[min_index].as_state();
        }
        Some(button_presses)
    }

    fn solve_joltage_to_desired_state(
        &mut self,
        current_joltage: &JoltageLevels,
        desired_state: usize,
    ) -> usize {
        if let Some(&cached) = self.cache.get(&(current_joltage.0.clone(), desired_state)) {
            return cached;
        }

        let button_presses = self.solvable_states.get(&desired_state);

        match button_presses {
            Some(button_presses) => {
                let mut new_joltage = JoltageLevels(current_joltage.0.clone());
                for (i, &count) in button_presses.iter().enumerate() {
                    if count > 0 {
                        for index in self.buttons[i].0.iter() {
                            if new_joltage.0[*index] == 0 {
                                return usize::MAX;
                            }
                            new_joltage.0[*index] -= 1
                        }
                    }
                }
                let res = button_presses
                    .iter()
                    .sum::<usize>()
                    .saturating_add(self.solve_joltage(&new_joltage));
                self.cache
                    .insert((current_joltage.0.clone(), desired_state), res);
                res
            }
            None => return usize::MAX,
        }
    }

    fn solve_joltage(&mut self, joltage: &JoltageLevels) -> usize {
        if joltage.0.iter().all(|&v| v == 0) {
            return 0;
        }
        let states = self.solvable_states.keys().cloned().collect::<Vec<usize>>();
        states
            .iter()
            .map(|state| {
                let res = self.solve_joltage_to_desired_state(joltage, *state);
                res
            })
            .min()
            .unwrap()
    }

    fn solve(mut self) -> usize {
        let joltage = self.desired_voltages.clone();
        println!(
            "Solving machine with desired voltages: {:?} and solvable states: {:?}",
            joltage.0,
            self.solvable_states.keys()
        );
        self.solve_joltage(&joltage)
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
    // Finished input

    // Start calculation
    let res: usize = machines.into_iter().map(|m| m.solve()).sum();
    println!("👉 {res}");
}
