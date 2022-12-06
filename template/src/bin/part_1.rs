use day_n::process_part_1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part_1(&file))
}
