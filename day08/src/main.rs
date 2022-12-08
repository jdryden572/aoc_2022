use std::collections::HashSet;

use helpers::read_lines_panicky;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
    println!("Part 2: {}", part2("input.txt"));
}

fn part1(path: &str) -> usize {
    let forest = Forest::parse(path);
    let mut visible = HashSet::new();
    for i in 0..forest.grid_size {
        visible.extend(visible_from_outside(forest.horizontal(i).iter()).map(|t| t.to_xy()));
        visible.extend(visible_from_outside(forest.horizontal(i).iter().rev()).map(|t| t.to_xy()));
        visible.extend(visible_from_outside(forest.vertical(i).iter()).map(|t| t.to_xy()));
        visible.extend(visible_from_outside(forest.vertical(i).iter().rev()).map(|t| t.to_xy()));
    }

    visible.len()
}

fn part2(path: &str) -> usize {
    let forest = Forest::parse(path);

    let mut max_score = 0;
    for i in 0..forest.grid_size {
        for j in 0..forest.grid_size {
            let horizontal = forest.horizontal(i);
            let vertical = forest.vertical(j);
            let &Tree(x, y, height) = &horizontal[j];

            let (left, right) = horizontal.split_at(x);
            let right = &right[1..];
            let (up, down) = vertical.split_at(y);
            let down = &down[1..];

            let score = score_view(height, left.iter().rev())
                * score_view(height, right.iter())
                * score_view(height, up.iter().rev())
                * score_view(height, down.iter());

            max_score = max_score.max(score);
        }
    }

    max_score
}

fn score_view<'a>(base_height: u8, view: impl Iterator<Item = &'a Tree>) -> usize {
    let mut score = 0;
    for tree in view {
        score += 1;
        if tree.2 >= base_height {
            break;
        }
    }
    score
}

fn visible_from_outside<'a>(
    trees: impl Iterator<Item = &'a Tree>,
) -> impl Iterator<Item = &'a Tree> {
    let mut tallest: i8 = -1;
    trees.filter(move |t| {
        if t.2 as i8 > tallest {
            tallest = t.2 as i8;
            true
        } else {
            false
        }
    })
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Tree(usize, usize, u8);

impl Tree {
    fn to_xy(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Forest {
    pub grid_size: usize,
    trees: Vec<u8>,
}

impl Forest {
    fn parse(path: &str) -> Self {
        let lines: Vec<_> = read_lines_panicky(path).collect();
        let grid_size = lines.len();
        let mut trees = Vec::new();
        for line in lines {
            trees.extend(line.chars().map(|c| c.to_digit(10).unwrap() as u8));
        }
        Self { grid_size, trees }
    }

    fn horizontal(&self, idx: usize) -> Vec<Tree> {
        let start = idx * self.grid_size;
        let mut trees = Vec::new();
        for i in 0..self.grid_size {
            let x = i;
            let y = idx;
            let cur = start + i;
            let height = self.trees[cur];
            trees.push(Tree(x, y, height));
        }
        trees
    }

    fn vertical(&self, idx: usize) -> Vec<Tree> {
        let start = idx;
        let mut vec = Vec::new();
        for i in 0..self.grid_size {
            let x = idx;
            let y = i;
            let cur = start + (i * self.grid_size);
            let height = self.trees[cur];
            vec.push(Tree(x, y, height))
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(21, part1("test_input.txt"));
    }

    #[test]
    fn parse_forest() {
        let expected = Forest {
            grid_size: 5,
            trees: vec![
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ],
        };
        assert_eq!(expected, Forest::parse("test_input.txt"));
    }

    #[test]
    fn forest_horizontal() {
        let forest = Forest::parse("test_input.txt");
        assert_eq!(
            vec![
                Tree(0, 0, 3),
                Tree(1, 0, 0),
                Tree(2, 0, 3),
                Tree(3, 0, 7),
                Tree(4, 0, 3)
            ],
            forest.horizontal(0)
        );
        assert_eq!(
            vec![
                Tree(0, 2, 6),
                Tree(1, 2, 5),
                Tree(2, 2, 3),
                Tree(3, 2, 3),
                Tree(4, 2, 2)
            ],
            forest.horizontal(2)
        );
        assert_eq!(
            vec![
                Tree(0, 4, 3),
                Tree(1, 4, 5),
                Tree(2, 4, 3),
                Tree(3, 4, 9),
                Tree(4, 4, 0)
            ],
            forest.horizontal(4)
        );
    }

    #[test]
    fn forest_vertical() {
        let forest = Forest::parse("test_input.txt");
        assert_eq!(
            vec![
                Tree(0, 0, 3),
                Tree(0, 1, 2),
                Tree(0, 2, 6),
                Tree(0, 3, 3),
                Tree(0, 4, 3)
            ],
            forest.vertical(0)
        );
        assert_eq!(
            vec![
                Tree(2, 0, 3),
                Tree(2, 1, 5),
                Tree(2, 2, 3),
                Tree(2, 3, 5),
                Tree(2, 4, 3)
            ],
            forest.vertical(2)
        );
        assert_eq!(
            vec![
                Tree(4, 0, 3),
                Tree(4, 1, 2),
                Tree(4, 2, 2),
                Tree(4, 3, 9),
                Tree(4, 4, 0)
            ],
            forest.vertical(4)
        );
    }

    #[test]
    fn test_select_visible() {
        assert_eq!(
            vec![Tree(0, 4, 3), Tree(1, 4, 5), Tree(3, 4, 9)],
            visible_from_outside(
                vec![
                    Tree(0, 4, 3),
                    Tree(1, 4, 5),
                    Tree(2, 4, 3),
                    Tree(3, 4, 9),
                    Tree(4, 4, 0)
                ]
                .iter()
            )
            .copied()
            .collect::<Vec<_>>()
        )
    }

    #[test]
    fn part1_final() {
        assert_eq!(1803, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(8, part2("test_input.txt"));
    }

    #[test]
    fn part2_final() {
        assert_eq!(268912, part2("input.txt"));
    }
}
