use std::{cmp::Ordering, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Num(a) => write!(f, "{}", a),
            Self::List(v) => {
                write!(f, "[")?;
                for item in v {
                    write!(f, "{}", item)?;
                    write!(f, ", ")?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Packet {
    fn as_list(&self) -> Self {
        match self {
            Packet::Num(a) => Self::List(vec![Self::Num(*a)]),
            Packet::List(_) => self.clone(),
        }
    }

    fn compare_as_lists(&self, other: &Self) -> Ordering {
        let fst = match self.as_list() {
            Self::List(v) => v,
            _ => panic!(),
        };

        let snd = match other.as_list() {
            Self::List(v) => v,
            _ => panic!(),
        };

        if !fst.is_empty() && snd.is_empty() {
            return Ordering::Greater;
        }

        for i in 0..fst.len() {
            // If a number on the second list is greater than
            // it's equivalent on the first list, or all the
            // equaivalents have the same value, but the second
            // list is larger

            if i > snd.len() - 1 || fst[i] > snd[i] {
                return Ordering::Greater;
            } else if fst[i] < snd[i] {
                // If some number on the first list is smaller than
                // its equivalent on the second list
                return Ordering::Less;
            }
        }

        // If all the equivalents are equal but the first
        // list is shorter
        if fst.len() < snd.len() {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => a == b,
            (a, b) => a.as_list() == b.as_list(),
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => a.partial_cmp(b),
            (a, b) => Some(a.compare_as_lists(b)),
        }
    }
}

// impl PartialOrd for Packet {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match (self, other) {
//             (Self::Num(a), Self::Num(b)) => a.partial_cmp(b),
//             (Self::List(a), Self::List(b)) => a.partial_cmp(b),
//             (Self::List(a), Self::Num(b)) => {
//                 a.partial_cmp(&NumList(vec![*b]))
//             }
//             (Self::Num(a), Self::List(b)) => {
//                 b.partial_cmp(&NumList(vec![*a]))
//             }
//         }
//     }
// }

// Grammar:
// Packet ::= [Packet | int]

fn list(input: &str) -> IResult<&str, Packet> {
    let (input, v) = delimited(
        tag("["),
        separated_list0(tag(","), packet),
        tag("]"),
    )(input)?;

    Ok((input, Packet::List(v)))
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((list, nom::character::complete::u32.map(Packet::Num)))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list0(
        tag("\n\n"),
        separated_pair(packet, tag("\n"), packet),
    )(input)
}

fn binary_search<T: Ord>(list: &Vec<T>, item: T) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = list.len() - 1;
    let mut mid = list.len() / 2;

    while low != high {
        match list[mid].cmp(&item) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid,
        }
        mid = (low + high) / 2;
    }
    None
}

pub fn process_part_1(input: &str) -> usize {
    let (_, pairs) = pairs(input).unwrap();
    let mut index_sum = 0;

    for (i, pair) in pairs.iter().enumerate() {
        if pair.0.lt(&pair.1) {
            index_sum += i + 1;
        }
    }

    index_sum
}

pub fn process_part_2(input: &str) -> usize {
    let (_, pairs) = pairs(input).unwrap();
    let mut packets = vec![];
    let (_, packet_2) = packet("[[2]]").unwrap();
    let (_, packet_6) = packet("[[6]]").unwrap();

    for pair in pairs {
        packets.push(pair.0);
        packets.push(pair.1);
    }
    packets.push(packet_2);
    packets.push(packet_6);
    let (_, packet_2) = packet("[[2]]").unwrap();
    let (_, packet_6) = packet("[[6]]").unwrap();
    packets.sort_unstable();

    (binary_search(&packets, packet_2).unwrap() + 1)
        * (binary_search(&packets, packet_6).unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1() {
        let input = fs::read_to_string("./test.txt").unwrap();
        assert_eq!(process_part_1(&input), 13);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("./test.txt").unwrap();
        assert_eq!(process_part_2(&input), 140);
    }
}
