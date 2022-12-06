fn is_fully_contained(a: (usize, usize), b: (usize, usize)) -> bool {
    if a.0 > b.0 {
        a.1 <= b.1
    } else if a.0 < b.0 {
        a.1 >= b.1
    } else {
        true
    }
}

fn overlaps(a: (usize, usize), b: (usize, usize)) -> bool {
    if a.0 < b.0 {
        b.0 <= a.1
    } else if a.0 > b.0 {
        overlaps(b, a)
    } else {
        true
    }
}

fn get_section_from_str(str: &str) -> Vec<usize> {
    str.split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

pub fn process_part_1(input: &str) -> usize {
    let mut total: usize = 0;

    for line in input.lines() {
        let pair = line.split(',').collect::<Vec<_>>();

        let sections_1 = get_section_from_str(pair[0]);

        let sections_2 = get_section_from_str(pair[1]);

        total += is_fully_contained(
            (sections_1[0], sections_1[1]),
            (sections_2[0], sections_2[1]),
        ) as usize
    }

    total
}

pub fn process_part_2(input: &str) -> usize {
    let mut total: usize = 0;

    for line in input.lines() {
        let pair = line.split(',').collect::<Vec<_>>();

        let sections_1 = get_section_from_str(pair[0]);

        let sections_2 = get_section_from_str(pair[1]);

        total += overlaps(
            (sections_1[0], sections_1[1]),
            (sections_2[0], sections_2[1]),
        ) as usize
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_contained_test() {
        // One range contains the other
        assert!(is_fully_contained((3, 5), (1, 8)));
        assert!(is_fully_contained((2, 9), (3, 8)));
        assert!(is_fully_contained((2, 9), (2, 8)));
        assert!(is_fully_contained((9, 29), (9, 81)));

        // One range does not contain the other
        assert!(!is_fully_contained((3, 9), (4, 15)));
    }

    #[test]
    fn overlaping() {
        // Does overlap
        assert!(overlaps((13, 45), (15, 90)));
        assert!(overlaps((15, 90), (13, 45)));
        assert!(overlaps((15, 90), (15, 45)));
        assert!(overlaps((15, 45), (15, 90)));

        // Does not overlap
        assert!(!overlaps((1, 6), (7, 12)));
        assert!(!overlaps((7, 16), (1, 6)));
    }

    #[test]
    fn part_1() {
        const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(process_part_1(INPUT), 2);
    }

    #[test]
    fn part_2() {
        const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(process_part_2(INPUT), 4);
    }
}
