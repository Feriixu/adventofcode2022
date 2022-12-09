use std::cmp::Ordering;

/// Parses the lines of the grid into a vector of vectors of u8
/// The grid represents a patch of trees
/// The number is the height of the tree
fn parse_input(input: String) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.chars().map(|char| char.to_string().parse().unwrap()).collect()).collect()
}

pub fn part1(input: String) -> usize {
    // Iterate over all trees, except the ones on the edges
    // If a tree is surrounded by trees of smaller height in any direction, it is not visible
    let trees = parse_input(input);
    let mut visible_trees = trees.len() * trees[0].len();
    // dbg!(visible_trees);
    for x in 1..(trees.len()-1) {
        for y in 1..(trees[0].len()-1) {
            let this_tree = trees[x][y];

            let mut right_hidden = false;
            let mut left_hidden = false;
            let mut up_hidden = false;
            let mut down_hidden = false;


            // Check to the right
            for xd in (x+1)..trees.len() {
                if trees[xd][y] >= this_tree {
                    right_hidden = true;
                    break;
                }
            }

            // Check to the left
            for xd in (0..x).rev() {
                if trees[xd][y] >= this_tree {
                    left_hidden = true;
                    break;
                }
            }

            // Check up
            for yd in (y+1)..trees[0].len() {
                if trees[x][yd] >= this_tree {
                    up_hidden = true;
                    break;
                }
            }

            // Check down
            for yd in (0..y).rev() {
                if trees[x][yd] >= this_tree {
                    down_hidden = true;
                    break;
                }
            }

            if right_hidden && left_hidden && up_hidden && down_hidden {
                visible_trees -= 1;
            }
        }
    }


    visible_trees

}

pub fn part2(input: String) -> usize {
    let trees = parse_input(input);
    let mut max_score = 0;

    for x in 0..trees.len() {
        for y in 0..trees[0].len() {
            let this_tree = trees[x][y];

            let mut right_trees = 0;
            let mut left_trees = 0;
            let mut up_trees = 0;
            let mut down_trees = 0;

            // To measure the viewing distance from a given tree, look up, down, left, and right from that tree;
            // stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration.
            // (If a tree is right on the edge, at least one of its viewing distances will be zero.)

            // Count to the right
            for xd in (x+1)..trees.len() {
                // If this tree is taller or equal than the one we're looking from, stop
                match trees[xd][y].cmp(&this_tree) {
                    Ordering::Greater | Ordering::Equal => {
                        right_trees += 1;
                        break;
                    },
                    Ordering::Less => right_trees += 1,
                }
            }

            // Count to the left
            for xd in (0..x).rev() {
                // If this tree is taller or equal than the one we're looking from, stop
                match trees[xd][y].cmp(&this_tree) {
                    Ordering::Greater | Ordering::Equal => {
                        left_trees += 1;
                        break;
                    },
                    Ordering::Less => left_trees += 1,
                }

            }

            // Count up
            for yd in (y+1)..trees[0].len() {
                // If this tree is taller or equal than the one we're looking from, stop
                match trees[x][yd].cmp(&this_tree) {
                    Ordering::Greater | Ordering::Equal => {
                        up_trees += 1;
                        break;
                    },
                    Ordering::Less => up_trees += 1,
                }
            }

            // Count down
            for yd in (0..y).rev() {
                // If this tree is taller or equal than the one we're looking from, stop
                match trees[x][yd].cmp(&this_tree) {
                    Ordering::Greater | Ordering::Equal => {
                        down_trees += 1;
                        break;
                    },
                    Ordering::Less => down_trees += 1,
                }
            }

            // Multiply to calculate the score
            let score = right_trees * left_trees * up_trees * down_trees;
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day8/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input =
            std::fs::read_to_string("input/day8/example.txt").expect("Can't load example file");
        assert_eq!(part2(input), 8);
    }
}
