use std::collections::HashSet;

use helpers::read_lines_panicky;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
}

fn part1(path: &str) -> usize {
    let mut head = Pt { x: 0, y: 0 };
    let mut tail = Pt { x: 0, y: 0 };

    let mut tail_spots = HashSet::new();
    tail_spots.insert(tail);

    let moves = read_lines_panicky(path).map(|l| Move::parse(&l));
    for Move { dir, steps } in moves {
        for _ in 0..steps {
            match dir {
                Dir::Up => head.y += 1,
                Dir::Down => head.y -= 1,
                Dir::Right => head.x += 1,
                Dir::Left => head.x -= 1,
            }

            tail = follow(head, tail);
            tail_spots.insert(tail);
        }
    }
    
    tail_spots.len()
}

fn follow(head: Pt, tail: Pt) -> Pt {
    let x_diff = head.x.abs_diff(tail.x);
    if x_diff > 1 {
        let x = tail.x + (head.x - tail.x).signum();
        return Pt::new(x, head.y)
    }

    let y_diff = head.y.abs_diff(tail.y);
    if y_diff > 1 {
        let y = tail.y + (head.y - tail.y).signum();
        return Pt::new(head.x, y)
    }

    tail
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

struct Move {
    dir: Dir,
    steps: usize,
}

enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Move {
    fn parse(input: &str) -> Self {
        let (dir, steps) = input.split_once(" ").unwrap();
        let steps = steps.parse().unwrap();
        let dir = match dir {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => panic!(),
        };
        Self { dir, steps }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow() {
        assert_eq!(Pt::new(0,0), follow(Pt::new(1,0), Pt::new(0,0)));
        assert_eq!(Pt::new(1,0), follow(Pt::new(2,0), Pt::new(0,0)));
        assert_eq!(Pt::new(1,0), follow(Pt::new(2,1), Pt::new(1,0)));
        assert_eq!(Pt::new(2,1), follow(Pt::new(2,2), Pt::new(1,0)));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(13, part1("test_input.txt"));
    }

    #[test]
    fn part1_final() {
        assert_eq!(6391, part1("input.txt"));
    }
}