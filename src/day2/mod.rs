use std::cmp::Ordering;
use std::ops::Add;
use std::str::FromStr;
use itertools::Itertools;
use strum::EnumString;

#[derive(EnumString, PartialEq, Debug, Copy, Clone)]
#[repr(u64)]
enum Shape {
    #[strum(serialize = "A", serialize = "X")]
    Rock = 1,
    #[strum(serialize = "B", serialize = "Y")]
    Paper = 2,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors = 3,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => Some(Ordering::Greater),
            (Shape::Paper, Shape::Rock) => Some(Ordering::Greater),
            (Shape::Scissors, Shape::Paper) => Some(Ordering::Greater),
            (Shape::Rock, Shape::Paper) => Some(Ordering::Less),
            (Shape::Paper, Shape::Scissors) => Some(Ordering::Less),
            (Shape::Scissors, Shape::Rock) => Some(Ordering::Less),
            (Shape::Rock, Shape::Rock) => Some(Ordering::Equal),
            (Shape::Paper, Shape::Paper) => Some(Ordering::Equal),
            (Shape::Scissors, Shape::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl Add<Action> for Shape {
    type Output = Self;

    fn add(self, rhs: Action) -> Self::Output {
        match rhs {
            Action::Loose => {
                match self {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                }
            }
            Action::Draw => {
                self
            }
            Action::Win => {
                match self {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            }
        }
    }
}

#[derive(EnumString, Debug, Clone, Copy)]
#[repr(u64)]
enum Action {
    #[strum(serialize = "X")]
    Loose = 0,
    #[strum(serialize = "Y")]
    Draw = 3,
    #[strum(serialize = "Z")]
    Win = 6,
}

impl From<Ordering> for Action {
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => Action::Loose,
            Ordering::Equal => Action::Draw,
            Ordering::Greater => Action::Win,
        }
    }
}

pub fn part1(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let (opponent, player) = line.split_whitespace().map(|shape| Shape::from_str(shape).unwrap()).collect_tuple().unwrap();
            player as u64 + Action::from(player.partial_cmp(&opponent).unwrap()) as u64
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let opponent = iter.next().unwrap().parse::<Shape>().unwrap();
            let action = iter.next().unwrap().parse::<Action>().unwrap();
            let player = opponent + action;
            player as u64 + action as u64
        })
        .sum()

}

mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        // Load the example input
        let input = std::fs::read_to_string("input/day2/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input = std::fs::read_to_string("input/day2/example.txt").expect("Can't load example file");
        assert_eq!(part2(input), 12);
    }
}