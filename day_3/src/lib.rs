fn get_letter_value(c: char) -> u8 {
    if c.is_lowercase() {
        (c as u8) - ('a' as u8) + 1
    } else {
        (c as u8) - ('A') as u8 + 27
    }
}

fn get_shared_item(fst: &str, snd: &str) -> Option<char> {
    for character in fst.chars() {
        if snd.contains(character) {
            return Some(character);
        }
    }

    None
}

// Find the common character in a group of 3 strings
fn get_shared_badge(fst: &str, snd: &str, trd: &str) -> Option<char> {
    for character in fst.chars() {
        if snd.contains(character) && trd.contains(character) {
            return Some(character);
        }
    }

    None
}

pub fn process_part_1(input: &str) -> usize {
    input
        .lines() // Rucksacks
        .map(|s| s.split_at(s.len() / 2)) // Splits in half
        .fold(
            0,
            |acc, pair| {
                acc + (get_letter_value(
                    get_shared_item(pair.0, pair.1).unwrap(), // Get the common letter
                ) as usize)
            }, // Sum up the values
        )
}

pub fn process_part_2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            get_letter_value(get_shared_badge(group[0], group[1], group[2]).unwrap()) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(process_part_1(INPUT), 157)
    }

    #[test]
    fn part_2() {
        const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(process_part_2(INPUT), 70)
    }
}
