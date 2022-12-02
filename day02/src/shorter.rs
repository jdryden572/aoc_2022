use helpers::read_lines_panicky;

pub fn part1(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|line| split(&line))
        .map(|(op, me)| score_part1(op, me))
        .sum()
}

pub fn part2(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|line| split(&line))
        .map(|(op, me)| score_part2(op, me))
        .sum()
}

fn split(line: &str) -> (char, char) {
    let mut chars = line.chars();
    let op = chars.next().unwrap();
    let me = chars.skip(1).next().unwrap();
    (op, me)
}

// Basically stolen from jonathanpaulson's python solution, but I added the numbers manually.
// https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/2.py
fn score_part1(op: char, me: char) -> usize {
    match (op, me) {
        ('A', 'X') => 4,
        ('A', 'Y') => 8,
        ('A', 'Z') => 3,

        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,

        ('C', 'X') => 7,
        ('C', 'Y') => 2,
        ('C', 'Z') => 6,

        _ => panic!()
    }
}

fn score_part2(op: char, me: char) -> usize {
    match (op, me) {
        ('A', 'X') => 3,
        ('A', 'Y') => 4,
        ('A', 'Z') => 8,

        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,

        ('C', 'X') => 2,
        ('C', 'Y') => 6,
        ('C', 'Z') => 7,

        _ => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(15, part1("test_input.txt"));
    }

    #[test]
    fn part1_final() {
        assert_eq!(9759, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(12, part2("test_input.txt"));
    }

    #[test]
    fn part2_final() {
        assert_eq!(12429, part2("input.txt"));
    }
}