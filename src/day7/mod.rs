use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap};
use std::fmt::Display;
use itertools::Itertools;

#[derive(Debug)]
enum FsNode {
    File {
        name: String,
        size: usize
    },
    Folder {
        name: String,
        contents: HashMap<String, FsNode>,
        size: usize
    },
}

impl FsNode {
    fn pretty_print(&self, level: usize) {
        let indent = "  ".repeat(level);
        match self {
            FsNode::File { name, size } => println!("{}- {} (file, size={})", indent, name, size),
            FsNode::Folder { name, contents, size } => {
                println!("{}- {} (dir, size={})", indent, name, size);
                for node in contents.values() {
                    node.pretty_print(level + 1);
                }
            }
        }
    }

    fn update_dir_sizes(&mut self) -> usize {
        match self {
            FsNode::File { size, .. } => *size,
            FsNode::Folder { contents, size, .. } => {
                let mut total_size = 0;
                for node in contents.values_mut() {
                    total_size += node.update_dir_sizes();
                }
                *size = total_size;
                total_size
            }
        }
    }

    fn sum_small_dirs(&mut self, max_size: usize) -> usize {
        let mut total_size = 0;
        // Traverse filesystem, and add the sizes of all Folders, that are smaller than max_size
        match self {
            FsNode::File { .. } => (),
            FsNode::Folder { contents, size, .. } => {
                if *size <= max_size {
                    total_size += *size;
                }
                for node in contents.values_mut() {
                    total_size += node.sum_small_dirs(max_size);
                }
            }
        }
        total_size
    }
}

impl FsNode {
    fn parse(input: &str) -> (String, Self) {
        if input.starts_with("dir") {
            let dir_name = input.strip_prefix("dir ").unwrap().to_string();
            (dir_name.clone(), FsNode::Folder {
                name: dir_name,
                contents: HashMap::new(),
                size: 0,
            })
        } else {
            let (file_size, file_name) = input.split_once(' ').unwrap();
            (file_name.to_string(), FsNode::File { name: file_name.to_string(), size: file_size.parse().unwrap() })
        }
    }

    fn get_all_dir_sizes(&self) -> Vec<usize> {
        let mut sizes = Vec::new();
        match self {
            FsNode::File { .. } => (),
            FsNode::Folder { contents, size, .. } => {
                sizes.push(*size);
                for node in contents.values() {
                    sizes.extend(node.get_all_dir_sizes());
                }
            }
        }
        sizes
    }

    fn select_smallest_dir_to_delete(&self) -> usize {
        const AVAILABLE_SPACE: usize = 70000000;
        const NEEDED_SPACE: usize = 30000000;
        let FsNode::Folder { contents, size, .. } = self else { return 0 };
        let unused_space = AVAILABLE_SPACE - size;
        let space_to_delete = NEEDED_SPACE - unused_space;

        let dir_sizes = self.get_all_dir_sizes();
        *dir_sizes.iter().sorted().find(|&&size| size > space_to_delete).unwrap()
    }
}

fn parse_filesystem(input: String) -> FsNode {
    let mut filesystem = FsNode::Folder { name: "".to_string(), contents: HashMap::new(), size: 0 };
    let mut dir_stack = Vec::new();

    let mut lines = input.lines().peekable();
    lines.next(); // Skip the first line

    'parsing_input: while let Some(line) = lines.next() {
        if line.starts_with("$ cd ") {
            let cd_path = line.strip_prefix("$ cd ").unwrap().to_string();
            match cd_path.as_str() {
                ".." => {
                    dir_stack.pop();
                }
                _ => {
                    dir_stack.push(cd_path);
                }
            }
            // println!("/{}", dir_stack.clone().join("/"));
        } else if line == "$ ls" {
            let FsNode::Folder { contents, .. } = dir_stack.iter().fold(&mut filesystem, |node, dir| {
                let FsNode::Folder { contents, .. } = node else { unreachable!() };
                contents.entry(dir.clone()).or_insert(FsNode::Folder { name: dir.to_string(), contents: HashMap::new(), size: 0 })
            }) else { unreachable!() };

            'parsing_ls: loop {
                if let Some(next_line) = lines.peek() {
                    if next_line.starts_with("$") {
                        break 'parsing_ls;
                    }
                }

                let Some(line) = lines.next() else { break 'parsing_input; };
                let (name, fs_node) = FsNode::parse(line);
                contents.entry(name).or_insert(fs_node);
            }
        }
    }
    filesystem
}

pub fn part1(input: String) -> usize {
    let mut filesystem = parse_filesystem(input);
    filesystem.update_dir_sizes();
    filesystem.sum_small_dirs(100000)
}

pub fn part2(input: String) -> usize {
    let mut filesystem = parse_filesystem(input);
    filesystem.update_dir_sizes();
    filesystem.select_smallest_dir_to_delete()
}

mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        // Load the example input
        let input = std::fs::read_to_string("input/day7/example.txt").expect("Can't load example file");
        assert_eq!(part1(input), 95437);
    }

    #[test]
    fn test_part2() {
        // Load the example input
        let input = std::fs::read_to_string("input/day7/example.txt").expect("Can't load example file");
        assert_eq!(part2(input), 24933642);
    }
}