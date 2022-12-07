use std::{collections::HashMap, path::{PathBuf, Path}};

use helpers::read_lines_panicky;

fn main() {
    let tree = parse_file_system("input.txt");
    println!("Part 1: {}", part1(&tree));
    println!("Part 2: {}", part2(&tree));
}

fn parse_file_system(path: &str) -> HashMap<PathBuf, Dir> {
    let lines: Vec<String> = read_lines_panicky(path).skip(1).collect(); // skip cd into /
    let root = Dir::default();
    let mut path = PathBuf::from("/"); 
    let mut tree: HashMap<PathBuf, Dir> = HashMap::new();
    tree.insert(path.clone(), root);

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        i += 1;
        let command = Command::parse(&line);
        match command {
            Command::CdIn(dir_name) => {
                // thanks AxlLind for showing me this API
                path.push(dir_name);
            }
            Command::CdOut => {
                let _ = path.pop();
            }
            Command::Ls => {
                let mut current_dir = tree.remove(&path).unwrap();
                for line in (&lines[i..]).iter().take_while(|l| !l.starts_with("$")) {
                    i += 1;
                    if line.starts_with("dir") {
                        let name = line.split_at(4).1;
                        current_dir.child_dirs.push(name.to_owned());
                        let new_path = path.join(name);
                        tree.insert(new_path, Dir::default());
                    } else {
                        let (size, _) = line.split_once(" ").unwrap();
                        current_dir.size_of_files += size.parse::<usize>().unwrap();
                    }
                }
                tree.insert(path.clone(), current_dir);
            }
        }
    }

    tree
}

fn part1(tree: &HashMap<PathBuf, Dir>) -> usize {
    let mut sizes = Vec::new();
    let root = PathBuf::from("/");
    get_all_sizes(tree, &root, &mut sizes);

    sizes.into_iter().filter(|&s| s < 100_000).sum()
}

fn part2(tree: &HashMap<PathBuf, Dir>) -> usize {
    let mut sizes = Vec::new();
    let root = PathBuf::from("/");
    let total_size = get_all_sizes(tree, &root, &mut sizes);
    let minimum_to_delete = total_size - 40_000_000;

    sizes.sort();
    sizes.into_iter().find(|&s| s > minimum_to_delete).unwrap()
}

fn get_all_sizes(tree: &HashMap<PathBuf, Dir>, path: &Path, sizes: &mut Vec<usize>) -> usize {
    let dir = tree.get(path).unwrap();
    let mut size = dir.size_of_files;
    for child in &dir.child_dirs {
        let path = path.join(child);
        size += get_all_sizes(tree, &path, sizes);
    }

    sizes.push(size);

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

    #[test]
    fn part2_sample() {
        let tree = parse_file_system("test_input.txt");
        assert_eq!(24933642, part2(&tree));
    }

    #[test]
    fn part2_final() {
        let tree = parse_file_system("input.txt");
        assert_eq!(366028, part2(&tree));
    }
}
