use std::collections::HashSet;

use helpers::read_lines_panicky;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
    println!("Part 2: {}", part2("input.txt"));
}

fn part1(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|line| rucksack_priority(&line))
        .sum()
}

fn part2(path: &str) -> usize {
    let lines = read_lines_panicky(path)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    lines.chunks(3).map(|chunk| group_priority(chunk)).sum()
}

fn group_priority(chunk: &[Vec<char>]) -> usize {
    let first = HashSet::<_>::from_iter(chunk[0].iter());
    let second = HashSet::<_>::from_iter(chunk[1].iter());
    let third = HashSet::<_>::from_iter(chunk[2].iter());

    let mut first_two = first.intersection(&second);
    let &&in_all_three = first_two.find(|&&c| third.contains(c)).unwrap();
    char_priority(in_all_three)
}

fn rucksack_priority(line: &[char]) -> usize {
    let halfway = line.len() / 2;
    let first = &line[..halfway];
    let second = &line[halfway..];

    let &in_both = first.into_iter().find(|c| second.contains(c)).unwrap();
    char_priority(in_both)
}

fn char_priority(c: char) -> usize {
    if c.is_lowercase() {
        (c as usize) - ('a' as usize) + 1
    } else {
        (c as usize) - ('A' as usize) + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(157, part1("test_input.txt"));
    }

    #[test]
    fn part1_final() {
        assert_eq!(7872, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(70, part2("test_input.txt"));
    }

    #[test]
    fn part2_final() {
        assert_eq!(2497, part2("input.txt"));
    }
}
