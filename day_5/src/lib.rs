use regex::Regex;
struct Command {
    amount: usize,
    start: usize,
    end: usize,
}

type ProcessedInput = (Vec<Vec<char>>, Vec<Command>);

fn move_crates(state: &mut Vec<Vec<char>>, cmd: Command) {
    for _ in 0..cmd.amount {
        let c = state[cmd.start as usize].pop().unwrap();
        state[cmd.end as usize].push(c)
    }
}

fn move_crates_in_order(state: &mut Vec<Vec<char>>, cmd: Command) {
    let new_size = state[cmd.start].len() - cmd.amount;

    let mut cranes = Vec::from(&state[cmd.start][new_size..]);

    state[cmd.start].truncate(new_size);
    state[cmd.end].append(&mut cranes);
}

fn input_setup(input: &str) -> ProcessedInput {
    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut commands: Vec<Command> = Vec::new();

    let cmd_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in input.lines() {
        if line.contains("[") {
            let num_crates = (line.len() + 1) / 4;
            while crates.len() < num_crates {
                // Create stacks
                crates.push(Vec::new())
            }

            for (col, c) in line.chars().enumerate() {
                let stack_num = col / 4;
                if c.is_ascii_alphabetic() {
                    let stack = &mut crates[stack_num];
                    stack.insert(0, c)
                }
            }
        }

        if cmd_re.is_match(line) {
            let caps = cmd_re.captures(line).unwrap();

            commands.push(Command {
                amount: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                start: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                end: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            })
        }
    }

    (crates, commands)
}

pub fn process_part_1(input: &str) -> String {
    let (mut state, commands) = input_setup(input);

    for cmd in commands {
        move_crates(&mut state, cmd)
    }

    return String::from_iter(state.iter().map(|s| s.last().unwrap()));
}

pub fn process_part_2(input: &str) -> String {
    let (mut state, commands) = input_setup(input);

    for cmd in commands {
        move_crates_in_order(&mut state, cmd)
    }

    return String::from_iter(state.iter().map(|s| s.last().unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(process_part_1(INPUT), "CMZ")
    }

    #[test]
    fn part_2() {
        const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(process_part_2(INPUT), "MCD")
    }
}
