use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance = distance.trim().parse().unwrap();
            match direction {
                "U" => Instruction::Up(distance),
                "D" => Instruction::Down(distance),
                "L" => Instruction::Left(distance),
                "R" => Instruction::Right(distance),
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    None,
}

fn catch_up(positions: &mut [Point], direction: Direction) -> Direction {

    let head = positions[0];
    let tail = positions[1];

    // Move the head
    match direction {
        Direction::None => Direction::None,
        Direction::Up => {
            if (head.y - tail.y).cmp(&1) == Ordering::Greater {
                positions[1].y += 1;
                match head.x.cmp(&tail.x) {
                    Ordering::Equal => { Direction::Up },
                    Ordering::Greater => { positions[1].x += 1; Direction::UpRight },
                    Ordering::Less => { positions[1].x -= 1; Direction::UpLeft },
                }
            } else {
                Direction::None
            }
        }
        Direction::Down => {
            if (tail.y - head.y).cmp(&1) == Ordering::Greater {
                positions[1].y -= 1;
                match head.x.cmp(&tail.x) {
                    Ordering::Equal => { Direction::Down },
                    Ordering::Greater => {positions[1].x += 1; Direction::DownRight },
                    Ordering::Less => { positions[1].x -= 1; Direction::DownLeft },
                }
            } else {
                Direction::None
            }
        }
        Direction::Left => {
            if (tail.x - head.x).cmp(&1) == Ordering::Greater {
                positions[1].x -= 1;
                match head.y.cmp(&tail.y) {
                    Ordering::Equal => { Direction::Left },
                    Ordering::Greater => { positions[1].y += 1; Direction::UpLeft },
                    Ordering::Less => { positions[1].y -= 1; Direction::DownLeft },
                }
            } else {
                Direction::None
            }
        }
        Direction::Right => {
            if (head.x - tail.x).cmp(&1) == Ordering::Greater {
                positions[1].x += 1;
                match head.y.cmp(&tail.y) {
                    Ordering::Equal => { Direction::Right },
                    Ordering::Greater => { positions[1].y += 1; Direction::UpRight },
                    Ordering::Less => { positions[1].y -= 1; Direction::DownRight },
                }
            } else {
                Direction::None
            }
        }
        Direction::UpRight => {
            if manhattan_distance(positions) > 2 {
                positions[1].x += 1;
                positions[1].y += 1;
                Direction::UpRight
            } else if positions[0].y - positions[1].y > 1 {
                positions[1].y += 1;
                Direction::Up
            } else if positions[0].x - positions[1].x > 1 {
                positions[1].x += 1;
                Direction::Right
            } else {
                Direction::None
            }
        }
        Direction::UpLeft => {
            if manhattan_distance(positions) > 2 {
                positions[1].x -= 1;
                positions[1].y += 1;
                Direction::UpLeft
            } else if positions[0].y - positions[1].y > 1 {
                positions[1].y += 1;
                Direction::Up
            } else if positions[1].x - positions[0].x > 1 {
                positions[1].x -= 1;
                Direction::Left
            } else {
                Direction::None
            }
        }
        Direction::DownRight => {
            if manhattan_distance(positions) > 2 {
                positions[1].x += 1;
                positions[1].y -= 1;
                Direction::DownRight
            } else if positions[1].y - positions[0].y > 1 {
                positions[1].y -= 1;
                Direction::Down
            } else if positions[0].x - positions[1].x > 1 {
                positions[1].x += 1;
                Direction::Right
            } else {
                Direction::None
            }
        }
        Direction::DownLeft => {
            if manhattan_distance(positions) > 2 {
                positions[1].x -= 1;
                positions[1].y -= 1;
                Direction::DownLeft
            } else if positions[1].y - positions[0].y > 1 {
                positions[1].y -= 1;
                Direction::Down
            } else if positions[1].x - positions[0].x > 1 {
                positions[1].x -= 1;
                Direction::Left
            } else {
                Direction::None
            }
        }
    }
}

fn manhattan_distance(positions: &mut [Point]) -> usize {
   ((positions[0].x - positions[1].x).abs() + (positions[0].y - positions[1].y).abs()) as usize
}

fn print_field(positions: &[Point], field: &HashSet<Point>) {
    // Find the bounds
    let min_x = positions.iter().map(|p| p.x).min().unwrap();
    let max_x = positions.iter().map(|p| p.x).max().unwrap();
    let min_y = positions.iter().map(|p| p.y).min().unwrap();
    let max_y = positions.iter().map(|p| p.y).max().unwrap();

    // Also find max and min in field
    let min_x = field.iter().map(|p| p.x).min().unwrap().min(min_x);
    let max_x = field.iter().map(|p| p.x).max().unwrap().max(max_x);
    let min_y = field.iter().map(|p| p.y).min().unwrap().min(min_y);
    let max_y = field.iter().map(|p| p.y).max().unwrap().max(max_y);


    for y in (min_y..=max_y).rev() {
        'print_position: for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("s");
                continue;
            }

            let point = Point { x, y };
            if positions.contains(&point) {
                for (i, p) in positions.iter().enumerate() {
                    if p == &point {
                        if i == 0 {
                            print!("H");
                        } else {
                            print!("{}", i);
                        }
                        continue 'print_position;
                    }
                }
            } else if field.contains(&point) {
                print!("#");
            } else if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part1(input: String) -> usize {
    let instructions = parse_input(input);
    let mut positions = [Point { x: 0, y: 0 }; 2];
    let mut visited = HashSet::new();
    for instruction in instructions {
        match instruction {
            Instruction::Up(distance) => {
                for _ in 0..distance {
                    positions[0].y += 1;
                    catch_up(&mut positions, Direction::Up);
                    visited.insert(positions[1]);
                }
            }
            Instruction::Down(distance) => {
                for _ in 0..distance {
                    positions[0].y -= 1;
                    catch_up(&mut positions, Direction::Down);
                    visited.insert(positions[1]);
                }
            }
            Instruction::Left(distance) => {
                for _ in 0..distance {
                    positions[0].x -= 1;
                    catch_up(&mut positions, Direction::Left);
                    visited.insert(positions[1]);
                }
            }
            Instruction::Right(distance) => {
                for _ in 0..distance {
                    positions[0].x += 1;
                    catch_up(&mut positions, Direction::Right);
                    visited.insert(positions[1]);
                }
            }
        }
    }

    visited.len()
}

pub fn part2(input: String) -> usize {
    let instructions = parse_input(input);

    let mut positions = [Point { x: 0, y: 0 }; 10];
    let mut visited = HashSet::from([Point { x: 0, y: 0 }]);
    // println!("\n== Initial position ==\n");
    // print_field(&positions, &visited);
    for instruction in instructions {
        // println!("\n== {:?} ==\n", &instruction);
        match instruction {
            Instruction::Up(distance) => {
                for _ in 0..distance {
                    positions[0].y += 1;
                    let mut direction = Direction::Up;
                    for i in 0..9 {
                        direction = catch_up(&mut positions[i..=i+1], direction);
                        visited.insert(positions[9]);
                    }
                }
            }
            Instruction::Down(distance) => {
                for _ in 0..distance {
                    positions[0].y -= 1;
                    let mut direction = Direction::Down;
                    for i in 0..9 {
                        direction = catch_up(&mut positions[i..=i+1], direction);
                        visited.insert(positions[9]);
                    }
                }
            }
            Instruction::Left(distance) => {
                for _ in 0..distance {
                    positions[0].x -= 1;
                    let mut direction = Direction::Left;
                    for i in 0..9 {
                        direction = catch_up(&mut positions[i..=i+1], direction);
                        visited.insert(positions[9]);
                    }
                }
            }
            Instruction::Right(distance) => {
                for _ in 0..distance {
                    positions[0].x += 1;
                    let mut direction = Direction::Right;
                    for i in 0..9 {
                        direction = catch_up(&mut positions[i..=i+1], direction);
                        visited.insert(positions[9]);
                    }
                }
            }
        }
        // print_field(&positions, &visited);
    }

    visited.len()
}

mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day9/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2_small() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day9/example.txt").expect("Can't load example file");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_part2_large() {
        let input = std::fs::read_to_string("input/day9/example2.txt").expect("Can't load example file");
        assert_eq!(part2(input), 36);
    }
}
