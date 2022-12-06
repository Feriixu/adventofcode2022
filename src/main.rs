use std::fs::read_to_string;

mod day1;
mod day2;

fn main() {
    println!("---------- Day 1 ----------");
    println!("Part 1: {}", day1::part1(read_to_string("input/day1/input.txt").unwrap()));
    println!("Part 2: {}", day1::part2(read_to_string("input/day1/input.txt").unwrap()));

    println!("---------- Day 2 ----------");
    println!("Part 1: {}", day2::part1(read_to_string("input/day2/input.txt").unwrap()));
    println!("Part 2: {}", day2::part2(read_to_string("input/day2/input.txt").unwrap()));
}
