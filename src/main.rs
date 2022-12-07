use std::fs::read_to_string;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day6;
mod day7;

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

    println!("---------- Day 4 ----------");
    println!("Part 1: {}", day4::part1(read_to_string("input/day4/input.txt").unwrap()));
    println!("Part 2: {}", day4::part2(read_to_string("input/day4/input.txt").unwrap()));

    println!("---------- Day 6 ----------");
    println!("Part 1: {}", day6::part1(read_to_string("input/day6/input.txt").unwrap()));
    println!("Part 2: {}", day6::part2(read_to_string("input/day6/input.txt").unwrap()));

    println!("---------- Day 7 ----------");
    println!("Part 1: {}", day7::part1(read_to_string("input/day7/input.txt").unwrap()));
    println!("Part 2: {}", day7::part2(read_to_string("input/day7/input.txt").unwrap()));

    // Print perf counter
    println!("Time: {}ms", start.elapsed().as_secs_f64() * 1000.0);
}
