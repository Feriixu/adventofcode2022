use std::fs::read_to_string;
use std::time::Instant;
use itertools::Itertools;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

    println!("---------- Day 5 ----------");
    // Given representations of the stacks
//         [H]         [S]         [D]
//     [S] [C]         [C]     [Q] [L]
//     [C] [R] [Z]     [R]     [H] [Z]
//     [G] [N] [H] [S] [B]     [R] [F]
// [D] [T] [Q] [F] [Q] [Z]     [Z] [N]
// [Z] [W] [F] [N] [F] [W] [J] [V] [G]
// [T] [R] [B] [C] [L] [P] [F] [L] [H]
// [H] [Q] [P] [L] [G] [V] [Z] [D] [B]
//  1   2   3   4   5   6   7   8   9
    // equivalent 2D vector representation
    let stacks = vec![
        "HTZD".chars().collect_vec(),
        "QRWTGCS".chars().collect_vec(),
        "PBFQNRCH".chars().collect_vec(),
        "LCNFHZ".chars().collect_vec(),
        "GLFQS".chars().collect_vec(),
        "VPWZBRCS".chars().collect_vec(),
        "ZFJ".chars().collect_vec(),
        "DLVZRHW".chars().collect_vec(),
        "BHGNFZLD".chars().collect_vec(),
    ];
    println!("Part 1: {}", day5::part1(read_to_string("input/day5/input_instructions.txt").unwrap(), stacks.clone()));
    println!("Part 2: {}", day5::part2(read_to_string("input/day5/input_instructions.txt").unwrap(), stacks));

    // Print perf counter
    println!("Time: {}ms", start.elapsed().as_secs_f64() * 1000.0);
}
