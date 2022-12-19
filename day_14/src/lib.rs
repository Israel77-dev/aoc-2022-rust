use std::{collections::HashSet, fmt::Display};

use nom::{
    bytes::complete::tag, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl Display for Point {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Draws a horizontal or vertical line
fn draw_line(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut result = vec![];
    match (p1.x == p2.x, p1.y == p2.y, p1.x < p2.x, p1.y < p2.y) {
        (true, _, _, true) => {
            for y in p1.y + 1..p2.y {
                result.push(Point { x: p1.x, y });
            }
        }
        (true, _, _, false) => {
            for y in p2.y + 1..p1.y {
                result.push(Point { x: p1.x, y });
            }
        }
        (_, true, true, _) => {
            for x in p1.x + 1..p2.x {
                result.push(Point { x, y: p1.y });
            }
        }
        (_, true, false, _) => {
            for x in p2.x + 1..p1.x {
                result.push(Point { x, y: p1.y });
            }
        }
        (_, _, _, _) => panic!(),
    }
    result
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, coords) = separated_pair(
        nom::character::complete::u16,
        tag(","),
        nom::character::complete::u16,
    )(input)?;

    Ok((
        input,
        Point {
            x: coords.0,
            y: coords.1,
        },
    ))
}

fn lines(input: &str) -> IResult<&str, HashSet<Point>> {
    let (input, points) = separated_list1(tag(" -> "), point)(input)?;
    let mut result = HashSet::new();

    for window in points.windows(2) {
        result.insert(window[0]);
        for point in draw_line(&window[0], &window[1]) {
            result.insert(point);
        }
        result.insert(window[1]);
    }
    Ok((input, result))
}

fn update_position(
    sand: &Point,
    blocked_tiles: &HashSet<Point>,
) -> Option<Point> {
    let check_tiles = [
        Point {
            x: sand.x,
            y: sand.y + 1,
        },
        Point {
            x: sand.x - 1,
            y: sand.y + 1,
        },
        Point {
            x: sand.x + 1,
            y: sand.y + 1,
        },
    ];

    for tile in check_tiles {
        if !blocked_tiles.contains(&tile) {
            return Some(tile);
        }
    }

    None
}

pub fn process_part_1(input: &str) -> usize {
    let (_, lines) =
        separated_list1(tag("\n"), lines)(input).unwrap();

    let mut blocked_tiles = HashSet::new();
    for line in lines {
        blocked_tiles.extend(line);
    }

    let lowest_y = blocked_tiles.iter().map(|p| p.y).max().unwrap();
    let mut sand_in_rest = 0;

    'outer: loop {
        let mut sand = Point { x: 500, y: 0 };

        // Repeats until there sand is blocked
        while let Some(new_sand) =
            update_position(&sand, &blocked_tiles)
        {
            // If sand is lower than the last line of rocks
            // the simulation is over
            if new_sand.y > lowest_y {
                break 'outer;
            }
            sand = new_sand;
        }

        blocked_tiles.insert(sand);
        sand_in_rest += 1;
    }

    sand_in_rest
}

pub fn process_part_2(input: &str) -> usize {
    let (_, lines) =
        separated_list1(tag("\n"), lines)(input).unwrap();

    let mut blocked_tiles = HashSet::new();
    for line in lines {
        blocked_tiles.extend(line);
    }

    let lowest_y =
        blocked_tiles.iter().map(|p| p.y).max().unwrap() + 2;
    let mut sand_in_rest = 0;

    'outer: loop {
        let mut sand = Point { x: 500, y: 0 };

        // Repeats until there sand is blocked
        while let Some(new_sand) =
            update_position(&sand, &blocked_tiles)
        {
            // If sand is lower than the last line of rocks
            // the simulation is over
            if new_sand.y == lowest_y {
                break;
            }
            sand = new_sand;
        }

        if sand == (Point { x: 500, y: 0 }) {
            break;
        }

        blocked_tiles.insert(sand);
        sand_in_rest += 1;
    }

    sand_in_rest + 1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("./test.txt").unwrap();
        assert_eq!(process_part_1(&input), 24);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("./test.txt").unwrap();
        assert_eq!(process_part_2(&input), 93);
    }
}
