use helpers::read_lines_panicky;

fn main() {
    let input = read_lines_panicky("input.txt").next().unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .skip_while(|(_, window)| {
            window
                .iter()
                .any(|c| window.iter().filter(|b| b == &c).count() > 1)
        })
        .map(|(i, _)| i)
        .next()
        .unwrap()
        + 4
}

fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .skip_while(|(_, window)| {
            window
                .iter()
                .any(|c| window.iter().filter(|b| b == &c).count() > 1)
        })
        .map(|(i, _)| i)
        .next()
        .unwrap()
        + 14
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
