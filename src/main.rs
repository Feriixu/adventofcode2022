use std::fs::read_to_string;
use std::time::Instant;

mod day1;
mod day2;
mod day3;

fn main() {
    // Start perf counter
    let start = Instant::now();

    println!("---------- Day 1 ----------");
    println!("Part 1: {}", day1::part1(read_to_string("input/day1/input.txt").unwrap()));
    println!("Part 2: {}", day1::part2(read_to_string("input/day1/input.txt").unwrap()));

    println!("---------- Day 2 ----------");
    println!("Part 1: {}", day2::part1(read_to_string("input/day2/input.txt").unwrap()));
    println!("Part 2: {}", day2::part2(read_to_string("input/day2/input.txt").unwrap()));

    println!("---------- Day 3 ----------");
    println!("Part 1: {}", day3::part1(read_to_string("input/day3/input.txt").unwrap()));
    println!("Part 2: {}", day3::part2(read_to_string("input/day3/input.txt").unwrap()));

    // Print perf counter
    println!("Time: {}ms", start.elapsed().as_secs_f64() * 1000.0);
}
