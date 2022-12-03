fn calculate_score_1(opponent: char, me: char) -> usize {
    match (opponent, me) {
        // Rock-rock
        ('A', 'X') => 3 + 1,
        // Rock-paper
        ('A', 'Y') => 6 + 2,
        // Rock-scissor
        ('A', 'Z') => 0 + 3,

        // Paper-rock
        ('B', 'X') => 0 + 1,
        // Paper-paper
        ('B', 'Y') => 3 + 2,
        // Paper-scissor
        ('B', 'Z') => 6 + 3,

        // Scissor-rock
        ('C', 'X') => 6 + 1,
        // Scissor-paper
        ('C', 'Y') => 0 + 2,
        // Scissor-scissor
        ('C', 'Z') => 3 + 3,
        _ => panic!(),
    }
}

fn calculate_score_2(fst: char, snd: char) -> usize {
    match (fst, snd) {
        // Rock-lose (scissor)
        ('A', 'X') => 0 + 3,
        // Rock-draw (rock)
        ('A', 'Y') => 3 + 1,
        // Rock-win (paper)
        ('A', 'Z') => 6 + 2,

        // Paper-lose (rock)
        ('B', 'X') => 0 + 1,
        // Paper-draw (paper)
        ('B', 'Y') => 3 + 2,
        // Paper-win (scissor)
        ('B', 'Z') => 6 + 3,

        // Scissor-lose (paper)
        ('C', 'X') => 0 + 2,
        // Scissor-draw (scissor)
        ('C', 'Y') => 3 + 3,
        // Scissor-win (rock)
        ('C', 'Z') => 6 + 1,
        _ => panic!(),
    }
}

pub fn process_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<_>>()
        })
        .map(|pair| calculate_score_1(pair[0], pair[1]))
        .sum()
}

pub fn process_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<_>>()
        })
        .map(|pair| calculate_score_2(pair[0], pair[1]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scores() {
        assert_eq!(calculate_score_1('A', 'X'), 4);
        assert_eq!(calculate_score_1('B', 'Y'), 5);
        assert_eq!(calculate_score_1('C', 'Z'), 6);

        assert_eq!(calculate_score_1('A', 'Y'), 8);
        assert_eq!(calculate_score_1('B', 'X'), 1);
    }

    #[test]
    fn part_1() {
        const INPUT: &str = "A Y
B X
C Z";

        assert_eq!(process_part_1(INPUT), 15)
    }

    #[test]
    fn part_2() {
        const INPUT: &str = "A Y
B X
C Z";

        assert_eq!(process_part_2(INPUT), 12)
    }
}
