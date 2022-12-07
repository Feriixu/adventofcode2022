use itertools::Itertools;

pub fn part1(input: String, mut stacks: Vec<Vec<char>>) -> String {

    // input lines are in the format: move <n> from <stack> to <stack>
    // parse the lines
    input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let n = words.nth(1).unwrap().parse::<usize>().unwrap();
        let from = words.nth(1).unwrap().parse::<usize>().unwrap();
        let to = words.nth(1).unwrap().parse::<usize>().unwrap();
        (n, from, to)
    }).for_each(|(n, from, to)| {
        for _ in 0..n {
            let char = stacks[from-1].pop().unwrap();
            stacks[to-1].push(char);
        }
    });

    // Map to the last element in each stack
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}

pub fn part2(input: String, mut stacks: Vec<Vec<char>>) -> String {
    // input lines are in the format: move <n> from <stack> to <stack>
    // parse the lines
    input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let n = words.nth(1).unwrap().parse::<usize>().unwrap();
        let from = words.nth(1).unwrap().parse::<usize>().unwrap();
        let to = words.nth(1).unwrap().parse::<usize>().unwrap();
        (n, from, to)
    }).for_each(|(n, from, to)| {
        let stack_height = stacks[from-1].len();
        let to_move = stacks[from-1].split_off(stack_height - n);
        for char in to_move {
            stacks[to-1].push(char);
        }
    });

    // Map to the last element in each stack
    stacks.iter().map(|stack| stack.last().unwrap()).join("")

}

mod tests {
    use super::{part1, part2};

    fn get_example_stacks() -> Vec<Vec<char>> {
        let stacks = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        stacks
    }

    #[test]
    fn test_part1() {
        // Load the example input
        let input = std::fs::read_to_string("input/day5/example.txt").expect("Can't load example file");
        assert_eq!(part1(input, get_example_stacks()), "CMZ");
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input = std::fs::read_to_string("input/day5/example.txt").expect("Can't load example file");
        assert_eq!(part2(input, get_example_stacks()), "MCD");
    }
}