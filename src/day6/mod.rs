use std::collections::HashSet;
use itertools::Itertools;

pub fn part1(input: String) -> usize {
    let mut index = 4;
    for (a,b,c,d) in input.chars().tuple_windows() {
        if HashSet::from([a,b,c,d]).len() == 4 {
            return index;
        } else {
            index += 1;
        }
    }
    0
}

pub fn part2(input: String) -> usize {
    let mut index = 14;
    let chars = input.chars().collect_vec();
    while index < chars.len() {
        let window = chars[(index - 14)..index].iter().cloned();
        let set = HashSet::<char>::from_iter(window);
        if set.len() == 14 {
            return index;
        } else {
            index += 1;
        }
    }
    0
}

mod tests {
    use itertools::Itertools;
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        // Load the example input
        let input = std::fs::read_to_string("input/day6/example.txt").expect("Can't load example file");
        for line in input.lines() {
            let (stream, solution) = line.split_whitespace().collect_tuple().unwrap();
            assert_eq!(part1(stream.to_string()), solution.parse::<usize>().unwrap());
        }
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input = std::fs::read_to_string("input/day6/example2.txt").expect("Can't load example file");
        for line in input.lines() {
            let (stream, solution) = line.split_whitespace().collect_tuple().unwrap();
            assert_eq!(part2(stream.to_string()), solution.parse::<usize>().unwrap());
        }
    }
}