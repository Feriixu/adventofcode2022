#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddxPrepare,
    Addx(i32),
}

fn parse_input(input: String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    input.lines().for_each(|line| {
        if line == "noop" {
            instructions.push(Instruction::Noop);
        } else if line.starts_with("addx") {
            let x = line.split_once(' ').unwrap().1.parse().unwrap();
            instructions.push(Instruction::AddxPrepare);
            instructions.push(Instruction::Addx(x));
        }
    });
    instructions
}

pub fn part1(input: String) -> usize {
    let instructions = parse_input(input);
    let mut x = 1;
    let mut total_signal_strength = 0;
    for (mut cycle, instruction) in instructions.iter().enumerate() {
        cycle += 1;
        if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
            let signal_strength = (cycle as i32) * x;
            total_signal_strength += signal_strength;
            println!("Cycle {}: x = {}, signal strength = {}", cycle, x, signal_strength);
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::AddxPrepare => {}
            Instruction::Addx(x2) => x += x2,
        }
    }

    total_signal_strength as usize
}

pub fn part2(input: String) {
    let instructions = parse_input(input);

    let mut x: i32 = 1;
    for (mut cycle, instruction) in instructions.iter().enumerate() {
        cycle += 1;

        if (x-1..=x+1).contains(&(cycle as i32 % 40 - 1)) {
            print!("#");
        } else {
            print!(" ");
        }

        if cycle % 40 == 0 {
            println!();
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::AddxPrepare => {}
            Instruction::Addx(to_add) => x += to_add
        }
    }

}

mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_small() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day10/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part1_big() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day10/example2.txt").expect("Can't load input file");
        assert_eq!(part1(input), 13140);
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day10/example.txt").expect("Can't load example file");
        part2(input);
        assert!(true);
    }
}
