use itertools::Itertools;

/// Returns the calories of the elf, that is carrying the most calories.
pub fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap_or(-1))
        .coalesce(|x, y| {
        if y == -1  {
            Err((x, y))
        } else if x == -1 {
            Ok(y)
        } else {
            Ok(x + y)
        }
    }).max().unwrap()
}

/// Returns the sum of the calories of the three elves, that are carrying the most calories.
pub fn part2(input: String) -> i32 {
    input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap_or(-1))
        .coalesce(|x, y| {
        if y == -1  {
            Err((x, y))
        } else if x == -1 {
            Ok(y)
        } else {
            Ok(x + y)
        }
    }).sorted().rev().take(3).sum()
}

mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        // Load the example input
        let input = std::fs::read_to_string("input/day1/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input = std::fs::read_to_string("input/day1/example.txt").expect("Can't load example file");
        assert_eq!(part2(input), 45000);
    }
}