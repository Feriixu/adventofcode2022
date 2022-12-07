use itertools::Itertools;
use std::ops::RangeInclusive;

trait Priority {
    fn priority(&self) -> u64;
}

impl Priority for char {
    /// a to z is 1 to 26
    /// A to Z is 27 to 52
    fn priority(&self) -> u64 {
        match self {
            'a'..='z' => *self as u64 - 96,
            'A'..='Z' => *self as u64 - 38,
            _ => 0,
        }
    }
}

trait ToRangeInclusive {
    fn to_range_inclusive(&self) -> RangeInclusive<u64>;
}

fn is_range_overlap(range1: &RangeInclusive<u64>, range2: &RangeInclusive<u64>) -> bool {
    range1.contains(range2.start())
        || range1.contains(range2.end())
        || range2.contains(range1.start())
        || range2.contains(range1.end())
}

impl ToRangeInclusive for &str {
    fn to_range_inclusive(&self) -> RangeInclusive<u64> {
        let mut split = self.split('-');
        let start = split.next().unwrap().parse::<u64>().unwrap();
        let end = split.next().unwrap().parse::<u64>().unwrap();
        start..=end
    }
}

pub fn part1(input: String) -> usize {
    input
        .lines()
        .filter(|&line| {
            line.split(',')
                .map(|range_str| range_str.to_range_inclusive())
                .permutations(2)
                .any(|permutation| {
                    // Check if the one range is a subset of the other
                    (permutation[0].contains(permutation[1].start())
                        && permutation[0].contains(permutation[1].end()))
                        || (permutation[1].contains(permutation[0].start())
                            && permutation[1].contains(permutation[0].end()))
                })
        })
        .count()
}

pub fn part2(input: String) -> usize {
    input
        .lines()
        .filter(|&line| {
            line.split(',')
                .map(|range_str| range_str.to_range_inclusive())
                .permutations(2)
                .any(|permutation| is_range_overlap(&permutation[0], &permutation[1]))
        })
        .count()
}

mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day4/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day4/example.txt").expect("Can't load example file");
        assert_eq!(part2(input), 4);
    }
}
