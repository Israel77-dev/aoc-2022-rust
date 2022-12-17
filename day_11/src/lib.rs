use std::cmp::Reverse;

use nom::{bytes::complete::tag, multi::separated_list1};

pub mod parser;

pub fn process_part_1(input: &str) -> usize {
    let (_, mut monkeys) =
        separated_list1(tag("\n\n"), parser::monkey)(input).unwrap();

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            for _ in 0..monkeys[index].items.len() {
                let monkey = monkeys.get_mut(index).unwrap();
                let item = monkey.inspect();
                let destination_index = monkey.test(item);

                monkeys[destination_index].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| Reverse(m.activity));
    monkeys
        .iter()
        .take(2)
        .map(|m| m.activity)
        .reduce(|a, b| a * b)
        .unwrap() as usize
}

pub fn process_part_2(input: &str) -> usize {
    let (_, mut monkeys) =
        separated_list1(tag("\n\n"), parser::monkey)(input).unwrap();

    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            for _ in 0..monkeys[index].items.len() {
                let monkey = monkeys.get_mut(index).unwrap();
                let item = monkey.inspect();
                let destination_index = monkey.test(item);

                monkeys[destination_index].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| Reverse(m.activity));
    monkeys
        .iter()
        .take(2)
        .map(|m| m.activity)
        .reduce(|a, b| a * b)
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1() {
        let input = &fs::read_to_string("./test.txt").unwrap();

        assert_eq!(process_part_1(input), 10605)
    }

    #[test]
    fn part_2() {
        todo!()
    }
}
