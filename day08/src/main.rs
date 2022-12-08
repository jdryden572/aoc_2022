use std::collections::HashSet;

use helpers::read_lines_panicky;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
}

fn part1(path: &str) -> usize {
    let forest = Forest::parse(path);
    let mut visible = HashSet::new();
    for i in 0..forest.grid_size {
        visible.extend(select_visible(forest.horizontal(i)).map(|t| t.to_xy()));
        visible.extend(select_visible(forest.horizontal(i).rev()).map(|t| t.to_xy()));
        visible.extend(select_visible(forest.vertical(i)).map(|t| t.to_xy()));
        visible.extend(select_visible(forest.vertical(i).rev()).map(|t| t.to_xy()));
    }

    visible.len()
}

fn select_visible(iter: impl Iterator<Item = Tree>) -> impl Iterator<Item = Tree> {
    let mut tallest: i8 = -1;
    let visible: Vec<_> = iter
        .filter(|t| {
            if t.2 as i8 > tallest {
                tallest = t.2 as i8;
                true
            } else {
                false
            }
        })
        .collect();
    visible.into_iter()
}

#[derive(PartialEq, Eq, Debug)]
struct Tree(usize, usize, u8);

impl Tree {
    fn to_xy(self) -> (usize, usize) {
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

    fn horizontal(&self, idx: usize) -> impl DoubleEndedIterator<Item = Tree> + '_ {
        let start = idx * self.grid_size;
        let mut trees = Vec::new();
        for i in 0..self.grid_size {
            let x = i;
            let y = idx;
            let cur = start + i;
            let height = self.trees[cur];
            trees.push(Tree(x, y, height));
        }
        trees.into_iter()
    }

    fn vertical(&self, idx: usize) -> impl DoubleEndedIterator<Item = Tree> + '_ {
        let start = idx;
        let mut vec = Vec::new();
        for i in 0..self.grid_size {
            let x = idx;
            let y = i;
            let cur = start + (i * self.grid_size);
            let height = self.trees[cur];
            vec.push(Tree(x, y, height))
        }
        vec.into_iter()
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
            forest.horizontal(0).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![
                Tree(0, 2, 6),
                Tree(1, 2, 5),
                Tree(2, 2, 3),
                Tree(3, 2, 3),
                Tree(4, 2, 2)
            ],
            forest.horizontal(2).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![
                Tree(0, 4, 3),
                Tree(1, 4, 5),
                Tree(2, 4, 3),
                Tree(3, 4, 9),
                Tree(4, 4, 0)
            ],
            forest.horizontal(4).collect::<Vec<_>>()
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
            forest.vertical(0).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![
                Tree(2, 0, 3),
                Tree(2, 1, 5),
                Tree(2, 2, 3),
                Tree(2, 3, 5),
                Tree(2, 4, 3)
            ],
            forest.vertical(2).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![
                Tree(4, 0, 3),
                Tree(4, 1, 2),
                Tree(4, 2, 2),
                Tree(4, 3, 9),
                Tree(4, 4, 0)
            ],
            forest.vertical(4).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_select_visible() {
        assert_eq!(
            vec![Tree(0, 4, 3), Tree(1, 4, 5), Tree(3, 4, 9)],
            select_visible(
                vec![
                    Tree(0, 4, 3),
                    Tree(1, 4, 5),
                    Tree(2, 4, 3),
                    Tree(3, 4, 9),
                    Tree(4, 4, 0)
                ]
                .into_iter()
            )
            .collect::<Vec<_>>()
        )
    }

    #[test]
    fn part1_final() {
        assert_eq!(1803, part1("input.txt"));
    }

    // #[test]
    // fn part2_sample() {
    //     assert_eq!(70, part2("test_input.txt"));
    // }

    // #[test]
    // fn part2_final() {
    //     assert_eq!(2497, part2("input.txt"));
    // }
}
