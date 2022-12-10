use std::time::Instant;

use helpers::read_lines_panicky;

fn main() {
    let instructions = instructions("input.txt");
    let now = Instant::now();
    println!("Part 1: {} ({:?})", part1(&instructions), now.elapsed());
}

fn instructions(path: &str) -> Vec<String> {
    read_lines_panicky(path).collect()
}

fn part1(instructions: &[String]) -> i64 {
    let mut cpu = Cpu::new();

    for instruction in instructions {
        if instruction.starts_with("addx") {
            let val = instruction[5..].parse().unwrap();
            cpu.addx(val);
        } else {
            cpu.noop();
        }
    }

    cpu.signal_strength
}

struct Cpu {
    cycle: i64,
    x: i64,
    signal_strength: i64,
}

impl Cpu {
    fn new() -> Self {
        Self { 
            cycle: 0,
            x: 1,
            signal_strength: 0,
        }
    }

    fn noop(&mut self) {
        self.cycle += 1;
        self.update_strength();
    }

    fn addx(&mut self, val: i64) {
        self.cycle += 1;
        self.update_strength();
        self.cycle += 1;
        self.update_strength();
        self.x += val;
    }

    fn update_strength(&mut self) {
        if (self.cycle + 20) % 40 == 0 {
            self.signal_strength += self.cycle * self.x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let instructions = instructions("test_input.txt");
        assert_eq!(13140, part1(&instructions));
    }

    #[test]
    fn part1_final() {
        let instructions = instructions("input.txt");
        assert_eq!(17840, part1(&instructions));
    }
}