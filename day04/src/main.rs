use helpers::read_lines_panicky;

type Range = (usize, usize);
type Pair = (Range, Range);

fn main() {
    println!("Part 1: {}", part1("input.txt"));
    println!("Part 2: {}", part2("input.txt"));
}

fn part1(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|line| parse_pair(&line))
        .filter(|&pair| is_fully_contained(pair))
        .count()
}

fn part2(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|line| parse_pair(&line))
        .filter(|&pair| is_overlapping(pair))
        .count()
}

fn parse_pair(line: &str) -> Pair {
    let mut split = line.split(",");
    let first = split.next().unwrap();
    let second = split.next().unwrap();
    (parse_range(first), parse_range(second))
}

fn parse_range(input: &str) -> Range {
    let mut split = input.split("-");
    let start = split.next().unwrap();
    let end = split.next().unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

fn is_fully_contained((first, second): Pair) -> bool {
    (is_in_range(first.0, second) && is_in_range(first.1, second))
        || (is_in_range(second.0, first) && is_in_range(second.1, first))
}

fn is_overlapping((first, second): Pair) -> bool {
    is_in_range(first.0, second)
        || is_in_range(first.1, second)
        || is_in_range(second.0, first)
        || is_in_range(second.1, first)
}

fn is_in_range(point: usize, range: Range) -> bool {
    range.0 <= point && point <= range.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(2, part1("test_input.txt"));
    }

    #[test]
    fn part1_final() {
        assert_eq!(503, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(4, part2("test_input.txt"));
    }

    #[test]
    fn part2_final() {
        assert_eq!(827, part2("input.txt"));
    }
}
