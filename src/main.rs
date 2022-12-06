use std::fs::read_to_string;

mod day1;

fn main() {
    println!("---------- Day 1 ----------");
    println!("Part 1: {}", day1::part1(read_to_string("input/day1/input.txt").unwrap()));
    println!("Part 2: {}", day1::part2(read_to_string("input/day1/input.txt").unwrap()));
}
