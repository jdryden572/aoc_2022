use std::{time::Instant, collections::{VecDeque, HashMap}};

use helpers::read_lines_panicky;

fn main() {
    let grid = parse_grid("input.txt");
    let now = Instant::now();
    println!("Part 1: {} ({:?})", part1(&grid), now.elapsed());
    let now = Instant::now();
    println!("Part 2: {} ({:?})", part2(&grid), now.elapsed());
}

fn parse_grid(path: &str) -> Vec<Vec<u8>> {
    read_lines_panicky(path)
        .map(|l| l.bytes().collect())
        .collect()
}

struct Visit {
    dist: usize,
    pos: (usize, usize),
}

fn part1(grid: &[Vec<u8>]) -> usize {
    let start = find_start(grid);
    shortest_distance(start, grid)
}

fn part2(grid: &[Vec<u8>]) -> usize {
    let mut min = usize::MAX;
    for start in find_potential_starts(grid) {
        min = min.min(shortest_distance(start, grid));
    }
    min
}

fn shortest_distance(start: (usize, usize), grid: &[Vec<u8>]) -> usize {
    let mut visited = HashMap::new();
    let mut to_visit = VecDeque::new();

    visited.insert(start, 0usize);
    for pos in neighbors(grid, start) {
        to_visit.push_back(Visit { dist: 1, pos });
    }

    while let Some(visit) = to_visit.pop_front() {
        let (x, y) = visit.pos;
        if grid[y][x] == b'E' {
            return visit.dist;
        }

        let dist = visit.dist + 1;
        for pos in neighbors(grid, visit.pos) {
            let prev = visited.get(&pos).unwrap_or(&usize::MAX);
            if &dist < prev {
                to_visit.push_back(Visit { pos, dist });
                visited.insert(pos, dist);
            }
        }
    }

    usize::MAX
}

fn neighbors(grid: &[Vec<u8>], (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut start = grid[y][x];
    if start < b'a' {
        start = b'z' + 1;
    }
    let len_x = grid[0].len() as i64;
    let len_y = grid.len() as i64;
    let (x, y) = (x as i64, y as i64);
    [
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y)
    ]
    .into_iter()
    .filter(move |(x, y)| &0 <= x && x < &len_x && &0 <= y && y < &len_y)
    .map(|(x, y)| (x as usize, y as usize))
    .filter(move |(x, y)| grid[*y][*x] <= start + 1)
}

fn find_start(grid: &[Vec<u8>]) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.iter().position(|b| b == &b'S') {
            return (x, y);
        }
    }

    unreachable!()
}

fn find_potential_starts(grid: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize)> {
    let mut starts = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &pos) in row.iter().enumerate() {
            if pos == b'S' || pos == b'a' {
                starts.push((x, y));
            }
        }
    }
    starts.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_sample() {
    //     let grid = parse_grid("test_input.txt");
    //     assert_eq!(31, part1(&grid));
    // }

    #[test]
    fn part1_final() {
        let grid = parse_grid("input.txt");
        assert_eq!(481, part1(&grid));
    }

    #[test]
    fn part2_final() {
        let grid = parse_grid("input.txt");
        assert_eq!(480, part2(&grid));
    }

    #[test]
    fn test_neighbors() {
        let grid = parse_grid("test_input.txt");
        assert_eq!(
            vec![(2, 0), (2, 2), (1, 1)],
            neighbors(&grid, (2, 1)).collect::<Vec<_>>()
        );
    }
}