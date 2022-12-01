use helpers::read_lines_panicky;

fn main() {
    let lines = get_lines("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn get_lines(path: &str) -> Vec<String> {
    read_lines_panicky(path).collect()
}

fn part1(lines: &[String]) -> usize {
    parse_group_sums(lines).max().unwrap()
}

fn part2(lines: &[String]) -> usize {
    let mut lines = parse_group_sums(lines).collect::<Vec<_>>();
    lines.sort();
    lines.iter().rev().take(3).sum()
}

fn parse_group_sums(lines: &[String]) -> impl Iterator<Item = usize> + '_ {
    lines
        .split(|l| l.is_empty())
        .map(|group| group.into_iter().map(|l| l.parse::<usize>().unwrap()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let lines = get_lines("test_input.txt");
        assert_eq!(24000, part1(&lines));
    }

    #[test]
    fn part1_final() {
        let lines = get_lines("input.txt");
        assert_eq!(67016, part1(&lines));
    }

    #[test]
    fn part2_sample() {
        let lines = get_lines("test_input.txt");
        assert_eq!(45000, part2(&lines));
    }

    #[test]
    fn part2_final() {
        let lines = get_lines("input.txt");
        assert_eq!(200116, part2(&lines));
    }
}
