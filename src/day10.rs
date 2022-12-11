use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::string::ParseError;
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};

// ---------------------------------------------------------------------------
#[derive(FromStr, Display)]
pub enum Inst {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Add(i64),
}

impl Inst {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Add(_) => 2,
        }
    }
}

struct CRT {
    pixels: [[bool; 40]; 6],
}

impl CRT {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    fn print(&self) {
        for line in 0..Self::HEIGHT {
            for col in 0..Self::WIDTH {
                if self.pixels[line][col] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    fn display(&mut self, xs: &Vec<i64>) {
        assert!(xs.len() > 240);
        for i in 1..=240 {
            let line = (i - 1) / 40;
            let col = (i - 1) % 40;
            let x = xs[i];
            self.pixels[line][col] = (col as i64) >= x - 1 && (col as i64) <= x + 1;
        }
    }
}

impl Default for CRT {
    fn default() -> Self {
        CRT {
            pixels: [[false; 40]; 6],
        }
    }
}

#[derive(Display)]
#[display("{cycle}: x {x}")]
struct Cpu {
    x: i64,
    cycle: i64,
    sig: Vec<i64>,
}

impl Cpu {
    fn signal_strength(&self, cycle: usize) -> i64 {
        let res = self.sig[cycle] * cycle as i64;
        println!("Cycle {} X {} Strength {}", cycle, self.sig[cycle], res);
        res
    }

    fn exec(&mut self, inst: &Inst) {
        match inst {
            Inst::Noop => {
                self.cycle += 1;
                self.sig.push(self.x);
            }
            Inst::Add(v) => {
                // first cycle
                self.cycle += 1;
                self.sig.push(self.x);
                // second cycle
                self.cycle += 1;
                self.sig.push(self.x);
                self.x += v; // XXX Careful, only effective at the end of the cycle
            }
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            x: 1,
            cycle: 0,
            sig: Vec::from([0]),
        }
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Inst> {
    let mut res: Vec<Inst> = Vec::new();
    for line in input.lines() {
        res.push(line.parse().unwrap());
    }
    res
}

// ---------------------------------------------------------------------------
#[aoc(day10, part1)]
pub fn part1(input: &Vec<Inst>) -> i64 {
    let mut cpu = Cpu::default();
    println!("{}", cpu);
    for inst in input {
        cpu.exec(inst);
        println!("{}", cpu);
    }

    println!("{:?}", cpu.sig);

    let mut res = 0;
    for i in 0..6 {
        let cycle = 20 + 40 * i;
        let s = cpu.signal_strength(cycle);
        println!("{} {}", cycle, s);
        res += s;
    }
    res
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<Inst>) -> i64 {
    let mut cpu = Cpu::default();
    for inst in input {
        cpu.exec(inst);
    }

    let mut crt = CRT::default();

    crt.display(&cpu.sig);

    crt.print();
    0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "noop
addx 3
addx -5";

    const INPUT2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_generator() {
        let mut inst = input_generator(INPUT);
    }

    #[test]
    fn test_part_small() {
        let mut cpu = Cpu::default();
        let mut inst = input_generator(INPUT);
        for i in inst.iter() {
            cpu.exec(&i);
            println!("{}", cpu);
        }
        println!("{:?}", cpu.sig);

        assert_eq!(1, cpu.signal_strength(1));
        assert_eq!(2, cpu.signal_strength(2));
        assert_eq!(16, cpu.signal_strength(4));
    }
    #[test]
    fn test_part1() {
        let mut cpu = Cpu::default();
        let mut inst = input_generator(INPUT2);

        println!("{}", cpu);
        for i in inst.iter() {
            cpu.exec(&i);
            println!("{} => {}", i, cpu);
        }
        // Manual
        assert_eq!(420, cpu.signal_strength(20));
        assert_eq!(1140, cpu.signal_strength(60));
        assert_eq!(1800, cpu.signal_strength(100));
        assert_eq!(2940, cpu.signal_strength(140));
        assert_eq!(2880, cpu.signal_strength(180));
        assert_eq!(3960, cpu.signal_strength(220));

        // Part1
        assert_eq!(13140, part1(&inst));
    }
}
