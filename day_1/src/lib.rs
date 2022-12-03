pub fn process_part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf_carry| {
            elf_carry
                .lines()
                .map(|item| item.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

pub fn process_part_2(input: &str) -> usize {
    let mut elves = input
        .split("\n\n")
        .map(|elf_carry| {
            elf_carry
                .lines()
                .map(|item| item.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    elves.sort_by(|a, b| b.cmp(a));

    elves.iter().take(3).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        const INPUT: &str = r"1000
2000
3000
 
4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(process_part_1(INPUT), 24000)
    }

    #[test]
    fn part_2() {
        const INPUT: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(process_part_2(INPUT), 45000)
    }
}
