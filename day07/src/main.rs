use std::collections::HashMap;

use helpers::read_lines_panicky;

fn main() {
    let tree = parse_file_system("input.txt");
    println!("Part 1: {}", part1(&tree));
}

fn parse_file_system(path: &str) -> HashMap<String, Dir> {
    let lines: Vec<String> = read_lines_panicky(path).skip(1).collect(); // skip cd into /
    let root = Dir::default();
    let mut stack = vec!["".to_owned()];
    let mut tree: HashMap<String, Dir> = HashMap::new();
    tree.insert("".to_owned(), root);

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        i += 1;
        let command = Command::parse(&line);
        match command {
            Command::CdIn(dir_name) => {
                stack.push(dir_name.to_owned());
            }
            Command::CdOut => {
                let _ = stack.pop().unwrap();
            }
            Command::Ls => {
                let path = stack.join("/");
                let mut current_dir = tree.remove(&path).unwrap();
                for line in (&lines[i..]).iter().take_while(|l| !l.starts_with("$")) {
                    i += 1;
                    if line.starts_with("dir") {
                        let name = line.split_at(4).1;
                        current_dir.child_dirs.push(name.to_owned());
                        let new_path = format!("{}/{}", &path, name);
                        tree.insert(new_path, Dir::default());
                    } else {
                        let (size, _) = line.split_once(" ").unwrap();
                        current_dir.size_of_files += size.parse::<usize>().unwrap();
                    }
                }
                tree.insert(path, current_dir);
            }
        }
    }

    tree
}

fn part1(tree: &HashMap<String, Dir>) -> usize {
    let mut total = 0;
    get_sizes_part1(tree, "", &mut total);

    total
}

fn get_sizes_part1(tree: &HashMap<String, Dir>, path: &str, total: &mut usize) -> usize {
    let dir = tree.get(path).unwrap();
    let mut size = dir.size_of_files;
    for child in &dir.child_dirs {
        let path = format!("{}/{}", path, child);
        size += get_sizes_part1(tree, &path, total);
    }

    if size < 100_000 {
        *total += size;
    }

    size
}

enum Command<'a> {
    CdIn(&'a str),
    CdOut,
    Ls,
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> Command<'a> {
        if input.starts_with("$ cd ") {
            if input.starts_with("$ cd ..") {
                Command::CdOut
            } else {
                let dir = input.split_at(5).1;
                Command::CdIn(dir)
            }
        } else {
            Command::Ls
        }
    }
}

#[derive(Default)]
struct Dir {
    size_of_files: usize,
    child_dirs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let tree = parse_file_system("test_input.txt");
        assert_eq!(95437, part1(&tree));
    }

    #[test]
    fn part1_final() {
        let tree = parse_file_system("input.txt");
        assert_eq!(1086293, part1(&tree));
    }
}
