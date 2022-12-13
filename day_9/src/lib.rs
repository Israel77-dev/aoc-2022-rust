use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    fn is_touching(&self, other: &Point) -> bool {
        // This will be true if the points overlap
        self == other
        // This part will be true iff the points are at most 1 unit apart in either direction
        || (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn follow(&mut self, other: &Point) {
        if self.is_touching(other) {
            return;
        }

        if self.x < other.x {
            self.x += 1;
        }

        if self.x > other.x {
            self.x -= 1;
        }

        if self.y < other.y {
            self.y += 1;
        }

        if self.y > other.y {
            self.y -= 1;
        }
    }
}

fn update(head: &mut Point, rope: &mut Point, direction: Direction) {
    match direction {
        Direction::Left => head.x -= 1,
        Direction::Right => head.x += 1,
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
    }

    rope.follow(head)
}

pub fn process_part_1(input: &str) -> usize {
    let mut head = Point::new();
    let mut rope = Point::new();

    let mut visited: HashSet<Point> = HashSet::new();

    let re_command = Regex::new(r"(U|D|L|R) (\d+)").unwrap();

    for line in input.lines() {
        let caps = re_command.captures(line).unwrap();

        let direction = match &caps[1] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        let amount = caps[2].parse::<u8>().unwrap();

        for _ in 0..amount {
            update(&mut head, &mut rope, direction);
            visited.insert(rope);
        }
    }

    visited.len()
}

pub fn process_part_2(input: &str) -> usize {
    let mut knots = vec![Point::new(); 11];

    let mut visited: HashSet<Point> = HashSet::new();

    let re_command = Regex::new(r"(U|D|L|R) (\d+)").unwrap();

    for line in input.lines() {
        let caps = re_command.captures(line).unwrap();

        let direction = match &caps[1] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        let amount = caps[2].parse::<u8>().unwrap();

        for _ in 0..amount {
            match direction {
                Direction::Left => knots[0].x -= 1,
                Direction::Right => knots[0].x += 1,
                Direction::Up => knots[0].y += 1,
                Direction::Down => knots[0].y -= 1,
            }
            for i in 1..knots.len() - 1 {
                let head = knots[i - 1];
                knots[i].follow(&head);
                visited.insert(knots[9]);
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(process_part_1(INPUT), 13);
    }

    #[test]
    fn part_2() {
        const INPUT: &str = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";

        assert_eq!(process_part_2(INPUT), 36);
    }
}
