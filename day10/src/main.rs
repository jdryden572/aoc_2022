use std::time::Instant;

use helpers::read_lines_panicky;

fn main() {
    let instructions = instructions("input.txt");
    let now = Instant::now();
    let cpu = run(&instructions);
    println!("Elapsed: {:?}", now.elapsed());
    println!("Part 1: {}", cpu.signal_strength);
    println!("Part 2:\n{}", cpu.print());
}

fn instructions(path: &str) -> Vec<String> {
    read_lines_panicky(path).collect()
}

fn run(instructions: &[String]) -> Cpu {
    let mut cpu = Cpu::new();

    for instruction in instructions {
        if instruction.starts_with("addx") {
            let val = instruction[5..].parse().unwrap();
            cpu.addx(val);
        } else {
            cpu.noop();
        }
    }

    cpu
}

struct Cpu {
    cycle: i64,
    x: i64,
    signal_strength: i64,
    pixels: [[char; 40]; 6],
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycle: 0,
            x: 1,
            signal_strength: 0,
            pixels: [['.'; 40]; 6],
        }
    }

    fn noop(&mut self) {
        self.clock();
    }

    fn addx(&mut self, val: i64) {
        self.clock();
        self.clock();
        self.x += val;
    }

    fn clock(&mut self) {
        self.cycle += 1;
        self.update_strength();
        self.draw_pixel();
    }

    fn update_strength(&mut self) {
        if (self.cycle + 20) % 40 == 0 {
            self.signal_strength += self.cycle * self.x;
        }
    }

    fn draw_pixel(&mut self) {
        let cycle = self.cycle - 1;
        let row = cycle / 40;
        let pos = cycle % 40;

        if pos.abs_diff(self.x) <= 1 {
            self.pixels[row as usize][pos as usize] = '#';
        }
    }

    fn print(&self) -> String {
        let mut output = String::with_capacity(6 * 40);
        for row in self.pixels {
            for ch in row {
                output.push(ch);
            }
            output.push('\n');
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let cpu = run(&instructions("test_input.txt"));
        assert_eq!(13140, cpu.signal_strength);
    }

    #[test]
    fn part1_final() {
        let cpu = run(&instructions("input.txt"));
        assert_eq!(17840, cpu.signal_strength);
    }

    #[test]
    fn part2_sample() {
        let expected = 
"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        let cpu = run(&instructions("test_input.txt"));
        assert_eq!(expected, &format!("\n{}", cpu.print()));
    }

    #[test]
    fn part2_final() {
        let expected = 
"
####..##..#.....##..#..#.#....###...##..
#....#..#.#....#..#.#..#.#....#..#.#..#.
###..#..#.#....#....#..#.#....#..#.#....
#....####.#....#.##.#..#.#....###..#.##.
#....#..#.#....#..#.#..#.#....#....#..#.
####.#..#.####..###..##..####.#.....###.
";
        let cpu = run(&instructions("input.txt"));
        assert_eq!(expected, &format!("\n{}", cpu.print()));
    }
}
