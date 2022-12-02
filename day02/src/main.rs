use std::{str::FromStr, convert::Infallible};

use helpers::read_lines_panicky;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
}

fn part1(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|l| l.parse::<Game>().unwrap())
        .map(|g| g.play())
        .map(|o| o.score())
        .sum()
}

struct Game {
    opponent: Shoot,
    me: Shoot,
}

impl Game {
    fn play(&self) -> Outcome {
        match self.me {
            Shoot::Rock => match self.opponent {
                Shoot::Rock => Outcome::Draw(Shoot::Rock),
                Shoot::Paper => Outcome::Lose(Shoot::Rock),
                Shoot::Scissors => Outcome::Win(Shoot::Rock),
            },
            Shoot::Paper => match self.opponent {
                Shoot::Rock => Outcome::Win(Shoot::Paper),
                Shoot::Paper => Outcome::Draw(Shoot::Paper),
                Shoot::Scissors => Outcome::Lose(Shoot::Paper),
            },
            Shoot::Scissors => match self.opponent {
                Shoot::Rock => Outcome::Lose(Shoot::Scissors),
                Shoot::Paper => Outcome::Win(Shoot::Scissors),
                Shoot::Scissors => Outcome::Draw(Shoot::Scissors),
            },
        }
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let opponent = split.next().unwrap().parse().unwrap();
        let me = split.next().unwrap().parse().unwrap();
        let game = Game { opponent, me };
        Ok(game)
    }
}

enum Shoot {
    Rock,
    Paper,
    Scissors,
}

impl Shoot {
    fn score(&self) -> usize {
        match self {
            Shoot::Rock => 1,
            Shoot::Paper => 2,
            Shoot::Scissors => 3,
        }
    }
}

impl FromStr for Shoot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shoot = match s {
            "A" | "X" => Shoot::Rock,
            "B" | "Y" => Shoot::Paper,
            "C" | "Z" => Shoot::Scissors,
            _ => panic!("Unrecognized letter"),
        };
        Ok(shoot)
    }
}

enum Outcome {
    Win(Shoot),
    Draw(Shoot),
    Lose(Shoot),
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win(shoot) => 6 + shoot.score(),
            Outcome::Draw(shoot) => 3 + shoot.score(),
            Outcome::Lose(shoot) => 0 + shoot.score(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(15, part1("test_input.txt"));
    }

    #[test]
    fn part1_final() {
        assert_eq!(9759, part1("input.txt"));
    }
}
