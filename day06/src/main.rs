use helpers::read_lines_panicky;

fn main() {
    let input = read_lines_panicky("input.txt").next().unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    find_marker_character(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker_character(input, 14)
}

fn find_marker_character(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .skip_while(|(_, window)| has_duplicates(window))
        .map(|(i, _)| i)
        .next()
        .unwrap()
        + window_size
}

fn has_duplicates(window: &[u8]) -> bool {
    (1..window.len()).any(|i| window[i..].contains(&window[i - 1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part1_final() {
        let input = read_lines_panicky("input.txt").next().unwrap();
        assert_eq!(1578, part1(&input));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part2_final() {
        let input = read_lines_panicky("input.txt").next().unwrap();
        assert_eq!(2178, part2(&input));
    }
}
