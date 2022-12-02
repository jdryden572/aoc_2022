use helpers::read_lines_panicky;

fn main() {
    println!("Part 1: {}", part1("input.txt"));
    println!("Part 2: {}", part2("input.txt"));
}

fn part1(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|l| Game::parse_part1(&l))
        .map(|g| g.play())
        .map(|o| o.score())
        .sum()
}

fn part2(path: &str) -> usize {
    read_lines_panicky(path)
        .map(|l| Game::parse_part2(&l))
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

    fn parse_part1(s: &str) -> Game {
        let mut split = s.split(" ");
        let opponent = Shoot::parse_part1(&split.next().unwrap());
        let me = Shoot::parse_part1(&split.next().unwrap());
        Game { opponent, me }
    }

    fn parse_part2(s: &str) -> Game {
        let mut split = s.split(" ");
        let opponent = Shoot::parse_part1(&split.next().unwrap());
        let outcome = PlannedOutcome::parse(&split.next().unwrap());
        let me = required_for_outcome(&opponent, &outcome);
        Game { opponent, me }
    }
}

enum PlannedOutcome {
    Win,
    Draw,
    Lose,
}

impl PlannedOutcome {
    fn parse(s: &str) -> PlannedOutcome {
        match s {
            "X" => PlannedOutcome::Lose,
            "Y" => PlannedOutcome::Draw,
            "Z" => PlannedOutcome::Win,
            _ => panic!("Unrecognized planned outcome"),
        }
    }
}

fn required_for_outcome(opponent: &Shoot, outcome: &PlannedOutcome) -> Shoot {
    match opponent {
        Shoot::Rock => match outcome {
            PlannedOutcome::Win => Shoot::Paper,
            PlannedOutcome::Draw => Shoot::Rock,
            PlannedOutcome::Lose => Shoot::Scissors,
        },
        Shoot::Paper => match outcome {
            PlannedOutcome::Win => Shoot::Scissors,
            PlannedOutcome::Draw => Shoot::Paper,
            PlannedOutcome::Lose => Shoot::Rock,
        },
        Shoot::Scissors => match outcome {
            PlannedOutcome::Win => Shoot::Rock,
            PlannedOutcome::Draw => Shoot::Scissors,
            PlannedOutcome::Lose => Shoot::Paper,
        },
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

    fn parse_part1(s: &str) -> Shoot {
        match s {
            "A" | "X" => Shoot::Rock,
            "B" | "Y" => Shoot::Paper,
            "C" | "Z" => Shoot::Scissors,
            _ => panic!("Unrecognized shoot"),
        }
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

    #[test]
    fn part2_sample() {
        assert_eq!(12, part2("test_input.txt"));
    }

    #[test]
    fn part2_final() {
        assert_eq!(12429, part2("input.txt"));
    }
}
