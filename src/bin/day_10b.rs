use advent_of_code_2025::get_input;

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

struct Machine {
    desired_voltages: JoltageLevels,
    buttons: Vec<Button>,
}

impl Machine {
    fn min_presses(&self) -> usize {
        // This is a linear algebra problem
        // Let A[i][j] = 1 if button i toggles voltage j, else 0
        // Let x[i] = number of times button i is pressed
        // And b[j] = the desired voltage of j
        // Then, we need to solve the equation:
        // A * x = b
        let mut matrix = vec![vec![0i32; self.buttons.len() + 1]; self.desired_voltages.0.len()];
        for j in 0..self.buttons.len() {
            for &i in &self.buttons[j].0 {
                matrix[i][j] = 1;
            }
        }
        for (i, row) in matrix.iter_mut().enumerate() {
            row[self.buttons.len()] = self.desired_voltages.0[i] as i32;
        }

        loop {
            // Now, we can perform Gaussian elimination on the matrix
            let mut finished = 0;
            loop {
                if matrix[finished][finished] == 0 {
                    let i = matrix
                        .iter()
                        .enumerate()
                        .position(|(i, row)| i > finished && row[finished] != 0);
                    if let Some(i) = i {
                        matrix.swap(i, finished); // Make sure the pivot row is at the top
                    }
                }
                let scalar = matrix[finished][finished];
                if scalar != 0 {
                    matrix[finished].iter_mut().for_each(|val| {
                        *val /= scalar;
                    });
                    for i in 0..matrix.len() {
                        if i != finished {
                            let scalar = matrix[i][finished];
                            for j in 0..matrix[i].len() {
                                matrix[i][j] -= scalar * matrix[finished][j];
                            }
                        }
                    }
                }
                finished += 1;
                if finished >= matrix.len() || finished >= matrix[0].len() - 1 {
                    break;
                }
            }

            println!("Matrix after elimination: {:?}", matrix);
            let solved_buttons: Vec<&Vec<i32>> = matrix
                .iter()
                .filter(|row| {
                    let zero_count = row.iter().take(row.len() - 1).filter(|&&x| x == 0).count();
                    let one_count = row.iter().take(row.len() - 1).filter(|&&x| x == 1).count();
                    return zero_count == row.len() - 2 && one_count == 1;
                })
                .collect();

            println!("Solved buttons: {:?}", solved_buttons);
            break;
        }

        matrix.iter().map(|row| row.last().unwrap()).sum::<i32>() as usize
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
        machines.push(Machine {
            desired_voltages,
            buttons,
        });
    }
    // Finished input

    // Start calculation
    let res: usize = machines.iter().map(|m| m.min_presses()).sum();
    println!("👉 {res}");
}
