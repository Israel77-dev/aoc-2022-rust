use day_6::process_part_2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part_2(&file))
}
