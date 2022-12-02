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

// Basically stolen from jonathanpaulson's python solution
// https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/2.py
fn score_part1(op: char, me: char) -> usize {
    let mut score = match me {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!()
    };
    score += match (op, me) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,

        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,

        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,

        _ => panic!()
    };
    score
}

fn score_part2(op: char, me: char) -> usize {
    let mut score = match me {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!()
    };
    score += match (op, me) {
        ('A', 'X') => 3,
        ('A', 'Y') => 1,
        ('A', 'Z') => 2,

        ('B', 'X') => 1,
        ('B', 'Y') => 2,
        ('B', 'Z') => 3,

        ('C', 'X') => 2,
        ('C', 'Y') => 3,
        ('C', 'Z') => 1,

        _ => panic!()
    };
    score
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