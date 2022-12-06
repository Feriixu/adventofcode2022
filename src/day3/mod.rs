use std::collections::HashSet;
use itertools::Itertools;

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

pub fn part1(input: String) -> u64 {
    input.lines()
        .map(|line| {
            let (compartement1, compartement2) = line.split_at(line.len() / 2);
            let types1: HashSet<char> = HashSet::from_iter(compartement1.chars());
            let types2: HashSet<char> = HashSet::from_iter(compartement2.chars());
            let common_letter = types1.intersection(&types2).next().unwrap();
            common_letter.priority()
        }).sum()
}

pub fn part2(input: String) -> u64 {
    input.lines()
        .map(|line| -> HashSet<char> {
             HashSet::from_iter(line.chars())
         })
        .chunks(3)
        .into_iter()
        .map(|group| group
            .reduce(|a, b| HashSet::from_iter(a.intersection(&b).cloned()))
            .unwrap().iter().next().unwrap().priority()
        )
        .sum()
}

mod tests {
    use super::{part1, part2, Priority};

    #[test]
    fn test_priority() {
        assert_eq!('a'.priority(), 1);
        assert_eq!('z'.priority(), 26);
        assert_eq!('A'.priority(), 27);
        assert_eq!('Z'.priority(), 52);
    }

    #[test]
    fn test_part1() {
        // Load the example input
        let input = std::fs::read_to_string("input/day3/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 157);
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input = std::fs::read_to_string("input/day3/example.txt").expect("Can't load example file");
        assert_eq!(part2(input), 70);
    }
}