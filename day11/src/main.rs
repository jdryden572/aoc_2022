use std::{collections::VecDeque, fmt::Debug, time::Instant};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let monkeys = parse_monkeys(&input);
    let now = Instant::now();
    println!("Part 1: {} ({:?})", part1(monkeys), now.elapsed());
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        simulate_round(&mut monkeys, |w| w / 3);
    }

    product_of_top_two(monkeys)
}

fn product_of_top_two(monkeys: Vec<Monkey>) -> usize {
    let mut counts: Vec<_> = monkeys.iter().map(|m| m.inspection_count).collect();
    counts.sort();
    counts.iter().rev().take(2).product()
}

fn simulate_round<R>(monkeys: &mut [Monkey], reduce_worry: R) 
where
    R: Fn(usize) -> usize,
{
    for i in 0..monkeys.len() {
        while let Some(val) = monkeys.get_mut(i).unwrap().items.pop_front() {
            monkeys.get_mut(i).unwrap().inspection_count += 1;
            let monkey = &monkeys[i];
            let worry_level = reduce_worry(monkey.operation.run(val));
            let give_to = if worry_level % monkey.divisible_by == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };

            monkeys
                .get_mut(give_to)
                .unwrap()
                .items
                .push_back(worry_level);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Plus(usize),
    Times(usize),
    Squared,
}

impl Operation {
    fn run(&self, val: usize) -> usize {
        match self {
            Operation::Plus(v) => v + val,
            Operation::Times(v) => v * val,
            Operation::Squared => val * val,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
    inspection_count: usize,
}

// impl Monkey {
//     fn print(&self) -> String {
//         self.items.iter().join(", ")
//     }
// }

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\r\n\r\n").map(parse_monkey).collect()
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines().skip(1).map(|l| l.trim());
    let items = lines
        .next()
        .unwrap()
        .trim_start_matches("Starting items: ")
        .split(", ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let (op, val) = lines
        .next()
        .unwrap()
        .trim_start_matches("Operation: new = old ")
        .split_once(' ')
        .unwrap();
    let operation = match (op, val) {
        ("+", val) => Operation::Plus(val.parse().unwrap()),
        ("*", "old") => Operation::Squared,
        ("*", val) => Operation::Times(val.parse().unwrap()),
        _ => panic!(),
    };

    let divisible_by = lines
        .next()
        .unwrap()
        .trim_start_matches("Test: divisible by ")
        .parse()
        .unwrap();
    let if_true = lines
        .next()
        .unwrap()
        .trim_start_matches("If true: throw to monkey ")
        .parse()
        .unwrap();
    let if_false = lines
        .next()
        .unwrap()
        .trim_start_matches("If false: throw to monkey ")
        .parse()
        .unwrap();

    Monkey {
        items,
        operation,
        divisible_by,
        if_true,
        if_false,
        inspection_count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        assert_eq!(
            vec![
                Monkey {
                    items: vec![79, 98].into(),
                    operation: Operation::Times(19),
                    divisible_by: 23,
                    if_true: 2,
                    if_false: 3,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![54, 65, 75, 74].into(),
                    operation: Operation::Plus(6),
                    divisible_by: 19,
                    if_true: 2,
                    if_false: 0,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![79, 60, 97].into(),
                    operation: Operation::Squared,
                    divisible_by: 13,
                    if_true: 1,
                    if_false: 3,
                    inspection_count: 0,
                },
                Monkey {
                    items: vec![74].into(),
                    operation: Operation::Plus(3),
                    divisible_by: 17,
                    if_true: 0,
                    if_false: 1,
                    inspection_count: 0,
                },
            ],
            parse_monkeys(&input)
        )
    }

    #[test]
    fn part1_sample() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let monkeys = parse_monkeys(&input);
        assert_eq!(10605, part1(monkeys));
    }

    #[test]
    fn part1_final() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let monkeys = parse_monkeys(&input);
        assert_eq!(88208, part1(monkeys));
    }
}
