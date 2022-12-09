use std::collections::HashSet;

use helpers::read_lines_panicky;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
    println!("Part 2: {}", part2("input.txt"));
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

fn part2(path: &str) -> usize {
    let mut knots = [Pt::new(0,0); 10];

    let mut tail_spots = HashSet::new();
    tail_spots.insert(knots[9]);

    let moves = read_lines_panicky(path).map(|l| Move::parse(&l));
    for Move { dir, steps } in moves {
        for _ in 0..steps {
            let head = knots.get_mut(0).unwrap();
            match dir {
                Dir::Up => head.y += 1,
                Dir::Down => head.y -= 1,
                Dir::Right => head.x += 1,
                Dir::Left => head.x -= 1,
            }

            for i in 1..10 {
                let first = knots[i - 1];
                let second = knots[i];
                knots[i] = follow(first, second);
            }

            tail_spots.insert(knots[9]);
        }
    }
    
    tail_spots.len()
}

fn print(knots: &[Pt]) {
    let x_iter = knots.iter().map(|k| k.x);
    let x_min = x_iter.clone().min().unwrap();
    let x_max = x_iter.max().unwrap();
    let y_iter = knots.iter().map(|k| k.y);
    let y_min = y_iter.clone().min().unwrap();
    let y_max = y_iter.max().unwrap();

    let x_range = (x_max - x_min) as usize;
    let y_range = (y_max - y_min) as usize;

    let row = vec!['.'; x_range + 1];
    let mut grid = vec![row; y_range + 1];

    for (i, &Pt { x, y }) in knots.iter().enumerate().rev() {
        let x = (x - x_min) as usize;
        let y = (y - y_min) as usize;
        let c = if i == 0 {
            'H'
        } else {
            (i as u8 + '0' as u8) as char
        };
        grid[y][x] = c;
    }

    for row in grid.into_iter().rev() {
        println!("{}", row.into_iter().collect::<String>());
    }
    println!();
}

fn follow(head: Pt, tail: Pt) -> Pt {
    let mut x = tail.x;
    let mut y = tail.y;

    let move_x = head.x.abs_diff(tail.x) > 1;
    let move_y = head.y.abs_diff(tail.y) > 1;

    if move_x || move_y {
        x = tail.x + (head.x - tail.x).signum();
        y = tail.y + (head.y - tail.y).signum();
    }

    Pt::new(x, y)
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

        assert_eq!(Pt::new(1,1), follow(Pt::new(2,2), Pt::new(0,0)));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(13, part1("test_input.txt"));
    }

    #[test]
    fn part1_final() {
        assert_eq!(6391, part1("input.txt"));
    }

    #[test]
    fn part2_sample_1() {
        assert_eq!(1, part2("test_input.txt"));
    }

    #[test]
    fn part2_sample_2() {
        assert_eq!(36, part2("test_input_2.txt"));
    }

    #[test]
    fn part2_final() {
        assert_eq!(2593, part2("input.txt"));
    }
}