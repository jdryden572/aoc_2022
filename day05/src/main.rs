use helpers::read_lines_panicky;
use lazy_static::lazy_static;
use regex::Regex;

const RADIX: u32 = 10;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
    println!("Part 2: {}", part2("input.txt"));
}

fn part1(path: &str) -> String {
    let mut stacks = parse_stacks(path);

    for Step { count, from, to } in read_lines_panicky(path)
        .filter(|l| l.starts_with("move"))
        .map(|l| parse_step(&l))
    {
        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    top_crates(&stacks)
}

fn part2(path: &str) -> String {
    let mut stacks = parse_stacks(path);
    let mut temp = Vec::new();

    for Step { count, from, to } in read_lines_panicky(path)
        .filter(|l| l.starts_with("move"))
        .map(|l| parse_step(&l))
    {
        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            temp.push(c);
        }

        while let Some(c) = temp.pop() {
            stacks[to].push(c);
        }
    }

    top_crates(&stacks)
}

fn top_crates(stacks: &[Vec<char>]) -> String {
    let mut result = String::new();
    for mut stack in stacks {
        result.push(*stack.last().unwrap());
    }
    result
}

fn parse_stacks(path: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<String> = read_lines_panicky(path)
        .take_while(|l| !l.starts_with("move"))
        .filter(|l| !l.is_empty())
        .collect();

    let labels = lines.pop().unwrap();
    let num_stacks = labels
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(RADIX)
        .unwrap() as usize;

    let mut stacks = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new())
    }

    for line in lines.into_iter().rev() {
        let bytes = line.as_bytes();
        for i in 0..num_stacks {
            let idx = (i * 4) + 1;
            let c = bytes[idx];
            if c != b' ' {
                stacks[i].push(c as char);
            }
        }
    }

    stacks
}

// fn print(stacks: &[Vec<char>]) {
//     let max_height = stacks.into_iter().map(|s| s.len()).max().unwrap();
//     for i in (0..max_height).rev() {
//         use std::fmt::Write;
//         let mut line = String::new();
//         for stack in stacks {
//             if stack.len() > i {
//                 write!(&mut line, "[{}] ", stack[i]).unwrap();
//             } else {
//                 write!(&mut line, "    ").unwrap();
//             }
//         }
//         println!("{}", line)
//     }
//     for i in 1..=stacks.len() {
//         print!(" {}  ", i);
//     }
//     println!();
// }

#[derive(PartialEq, Eq, Debug)]
struct Step {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_step(line: &str) -> Step {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let caps = RE.captures(line).unwrap();
    let count = caps[1].parse::<usize>().unwrap();
    let from = caps[2].parse::<usize>().unwrap() - 1;
    let to = caps[3].parse::<usize>().unwrap() - 1;

    Step { from, to, count }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!("CMZ", &part1("test_input.txt"));
    }

    #[test]
    fn test_parse_stacks() {
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(expected, parse_stacks("test_input.txt"));
    }

    #[test]
    fn test_parse_step() {
        assert_eq!(
            Step {
                count: 13,
                from: 6,
                to: 7
            }, // intentionally one less, for indexing position
            parse_step("move 13 from 7 to 8")
        );
    }

    #[test]
    fn part1_final() {
        assert_eq!("SHMSDGZVC", &part1("test_input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!("MCD", &part2("test_input.txt"));
    }

    // #[test]
    // fn part2_final() {
    //     assert_eq!(827, part2("input.txt"));
    // }
}
