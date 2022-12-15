// TODO: A lot of cleanup

use std::fmt::Debug;

use regex::Regex;

struct State {
    cycle: usize,
    // last_execution: usize,
    x: isize,
}

// #[derive(Debug, Clone, Copy)]
// struct Task {
//     when: usize,
//     command: Instruction,
// }

#[derive(Debug, Clone, Copy)]
enum Instruction {
    AddX(isize),
    Noop,
}

impl State {
    fn execute(&mut self, instruction: Instruction) -> () {
        // print!("{} -> ", self.x);
        match instruction {
            Instruction::AddX(v) => {
                // print!("({}) -> ", x);
                self.cycle += 2;
                self.x += v;
            }
            Instruction::Noop => {
                // print!("() -> ");
                self.cycle += 1;
            }
        }
        // println!("{}", self.x);
    }

    fn execute_with_drawing(&mut self, instruction: Instruction, buffer: &mut String) {
        match instruction {
            Instruction::AddX(v) => {
                // print!("({}) -> ", x);
                buffer.push(draw_char(&self.cycle, &self.x));
                self.cycle += 1;
                buffer.push(draw_char(&self.cycle, &self.x));
                self.cycle += 1;
                self.x += v;
            }
            Instruction::Noop => {
                // print!("() -> ");
                buffer.push(draw_char(&self.cycle, &self.x));
                self.cycle += 1;
            }
        }
    }

    fn execute_with_check(&mut self, instruction: Instruction, check_at: usize) -> Option<isize> {
        // print!("{} -> ", self.x);
        let mut result = None;
        match instruction {
            Instruction::AddX(v) => {
                // print!("({}) -> ", x);
                self.cycle += 2;
                if self.cycle >= check_at {
                    result = Some(self.x);
                }
                self.x += v;
            }
            Instruction::Noop => {
                // print!("() -> ");
                self.cycle += 1;
            }
        }

        result
        // println!("{}", self.x);
    }
}
// fn add_task(tasks: &mut VecDeque<Task>, state: &State, command: Command) {
//     tasks.push_back(match command {
//         Command::Add(x) => Task {
//             when: state.last_execution + 2,
//             command: Command::Add(x),
//         },
//         Command::Noop => Task {
//             when: state.last_execution,
//             command: Command::Noop,
//         },
//     });
// }

// fn execute_cycle(tasks: &mut VecDeque<Task>, state: &mut State) {
//     // println!("{} -> {:?}", state.cycle, tasks);
//     let mut remove_idx: Vec<usize> = vec![];
//     for (i, task) in tasks.iter().enumerate() {
//         if task.when == state.cycle {
//             state.execute(task.command);
//             remove_idx.push(i);
//         }
//     }

//     while let Some(i) = remove_idx.pop() {
//         tasks.swap_remove_back(i);
//     }

//     println!("({}): {}", state.cycle, state.x);
//     state.cycle += 1;
// }

fn draw_char(cycle: &usize, x: &isize) -> char {
    if (*x >= *cycle as isize - 1) && (*x <= *cycle as isize + 1) {
        return '#';
    } else {
        '.'
    }
}

pub fn process_part_1(input: &str) -> isize {
    let re_noop = Regex::new(r"noop").unwrap();
    let re_add_x = Regex::new(r"addx ([-]?\d+)").unwrap();

    let mut state = State {
        cycle: 0,
        // last_execution: 1,
        x: 1,
    };
    let mut instructions = Vec::<Instruction>::new();
    let mut strength = 0;
    let mut check = 20;

    for line in input.lines() {
        if re_add_x.is_match(line) {
            let v = re_add_x
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap();

            // add_task(&mut tasks, &state, Command::Add(v));
            instructions.push(Instruction::AddX(v));
        } else if re_noop.is_match(line) {
            // add_task(&mut tasks, &state, Command::Noop)
            instructions.push(Instruction::Noop);
        }
    }

    for instruction in instructions {
        if let Some(x) = state.execute_with_check(instruction, check) {
            println!("At {} -> {}", check, x * check as isize);
            strength += x * check as isize;
            check += 40;
        }
    }

    strength
}

pub fn process_part_2(input: &str) -> String {
    let re_noop = Regex::new(r"noop").unwrap();
    let re_add_x = Regex::new(r"addx ([-]?\d+)").unwrap();

    let mut state = State { cycle: 1, x: 1 };
    let mut instructions = Vec::<Instruction>::new();

    let mut buffer = String::new();

    for line in input.lines() {
        if re_add_x.is_match(line) {
            let v = re_add_x
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap();

            // add_task(&mut tasks, &state, Command::Add(v));
            instructions.push(Instruction::AddX(v));
        } else if re_noop.is_match(line) {
            // add_task(&mut tasks, &state, Command::Noop)
            instructions.push(Instruction::Noop);
        }
    }

    for instruction in instructions {
        state.execute_with_drawing(instruction, &mut buffer);
    }

    let mut formatted_result = String::new();

    for i in 0..buffer.len() / 40 {
        formatted_result.push_str(&buffer[i..i + 40]);
        formatted_result.push('\n');
    }

    formatted_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        assert_eq!(process_part_1(INPUT), 13140)
    }

    #[test]
    fn part_2() {
        const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(
            process_part_2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
