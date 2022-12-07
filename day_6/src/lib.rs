fn all_unique(elements: &[char]) -> bool {
    let mut previous = Vec::new();

    for element in elements {
        if previous.contains(&element) {
            return false;
        } else {
            previous.push(element)
        }
    }

    true
}

pub fn process_part_1(input: &str) -> usize {
    for (i, window) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
        if all_unique(window) {
            return i + 4;
        }
    }
    0
}

pub fn process_part_2(input: &str) -> usize {
    for (i, window) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
        if all_unique(window) {
            return i + 14;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(process_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(process_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(process_part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn part_2() {
        assert_eq!(process_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(process_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(process_part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(process_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
}
