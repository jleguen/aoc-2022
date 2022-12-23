use parse_display::{Display, FromStr};
//use std::collections::HashMap;
//use std::fmt;
//use std::str::FromStr;
use std::string::ParseError;
//use std::sync::{Arc, Mutex};
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};
//use num_bigint::{u64, Tou64};
//use std::ops::{Rem, Div};
//use serde_json::{Number, Value};
//use std::cmp::Ordering;
//use std::iter::zip;
//use petgraph::{graphmap::UnGraphMap};

// ---------------------------------------------------------------------------
enum Shape {
    Dash,
    Plus,
    Corner,
    Pipe,
    Square,
}

// Binary representation of lines, from bottom to top.
// Origin is bottom left
// Since the map is 7 units wide, every rock line fits a u8
// Rock are at most 4 lines high, so everything fits a u32.
// Left and Right jets are left- and right-shift.
// Keep a usize for the baseline
#[derive(Debug)]
struct Rock {
    sprite: u32,
    prev: u32,
}

impl Rock {
    fn shift(&mut self, dir: &Jet) {
        match dir {
            Jet::Left => {
                if self.sprite & 0x80808080 == 0 {
                    self.prev = self.sprite;
                    self.sprite <<= 1;
                }
            }
            Jet::Right => {
                if self.sprite & 0x02020202 == 0 {
                    self.prev = self.sprite;
                    self.sprite >>= 1;
                }
            }
        }
        //print!("\n{dir} {:08x}", self.sprite);
    }

    fn undo(&mut self) {
        self.sprite = self.prev;
    }

    fn line(&self, line: usize) -> u8 {
        match line {
            3 => (self.sprite & 0xff) as u8,
            2 => (self.sprite >> 8 & 0xff) as u8,
            1 => (self.sprite >> 16 & 0xff) as u8,
            0 => (self.sprite >> 24 & 0xff) as u8,
            _ => panic!(),
        }
    }

    fn print(&self) {
        println!("{:08b}", self.line(0));
        println!("{:08b}", self.line(1));
        println!("{:08b}", self.line(2));
        println!("{:08b}", self.line(3));
        println!("");
    }
}

const dash: u32 = 0x3c_00_00_00; // 0b00111100, 0, 0, 0
const plus: u32 = 0x10_38_10_00; // 0b00010000, 0b00111000, 0b00010000, 0
const corner: u32 = 0x38_08_08_00; // 0b00111000, 0b00001000, 0b00001000, 0
const pipe: u32 = 0x20_20_20_20; // 0b00100000, 0b00100000, 0b00100000, 0b00100000
const square: u32 = 0x30_30_00_00; // 0b00110000, 0b00110000, 0, 0

#[derive(FromStr, Display, Debug, Clone)]
pub enum Jet {
    #[display("<")]
    Left,
    #[display(">")]
    Right,
}

// ---------------------------------------------------------------------------
struct Chamber {
    rocks: [u32; 5],   // The five rocks
    rock_index: usize, // Current rock
    jets: Vec<Jet>,    // Pattern of jets
    jet_index: usize,  // Current jet
    map: Vec<u8>,      // one element per line
    height: usize,
}

impl Chamber {
    fn print(&self) {
        for line in self.map.iter().rev() {
            println!("{:08b}", line);
        }
    }

    fn next_jet(&mut self) -> &Jet {
        let res = &self.jets[self.jet_index];
        self.jet_index = (self.jet_index + 1) % self.jets.len();
        res
    }

    // true if rock can be there in the map
    fn check_pos(&self, rock: &Rock, z: &usize) -> bool {
        // Iterate all possible rock lines
        for i in 0..4 {
            if (rock.line(i) & self.map[z + i]) != 0 {
                // Intersection
                /*
                print!(
                    "\nz {} map {:08b} rock line {:08b}",
                    z + i,
                    self.map[z + i],
                    rock.line(i)
                );
                */
                return false;
            }
        }
        true
    }

    fn place(&mut self, rock: &Rock, z: &usize) {
        //println!(" -> Place rock {:08x} at height {z}", rock.sprite);
        for i in 0..4 {
            self.map[z + i] |= rock.line(i);
            if self.map[z + i] == 0 {
                self.height = z + i - 1;
                break;
            }
            //println!("{} {:08b}", z + i, self.map[z + i]);
        }
    }

    fn step(&mut self) {
        self.map.extend([0; 4]);
        // spawn new rock
        let mut z = self.height + 3; // TODO update height to avoid computation
        let mut rock = Rock {
            sprite: self.rocks[self.rock_index],
            prev: 0,
        };
        //print!("  Spawn rock {:08x} at height {z}", rock.sprite);
        self.rock_index = (self.rock_index + 1) % 5;

        loop {
            // Get pushed by a jet
            rock.shift(self.next_jet());
            if !self.check_pos(&rock, &z) {
                rock.undo();
            }
            // floor
            if z == 0 {
                //println!("Floor");
                break;
            }
            // Fall one line
            z -= 1;
            if !self.check_pos(&rock, &z) {
                z += 1;
                //println!("  Rock");
                break;
            }
        }
        // Here the rock is stopped at height z.
        self.place(&rock, &z);
    }

    fn play(&mut self, num: usize) {
        println!(
            "Play {num}. Rock cycle {}, jet cycle {}",
            self.rocks.len(),
            self.jets.len()
        );

        for i in 0..num {
            if i % 1000000000 == 0 {
                println!("{i}");
            }
            self.step();
        }
    }
}

impl Default for Chamber {
    fn default() -> Self {
        Self {
            rocks: [dash, plus, corner, pipe, square],
            rock_index: 0,
            jets: Vec::new(),
            jet_index: 0,
            //map: vec![0; 7], // 3 empty line + 4 available for biggest rock
            map: vec![0; 1000000000],
            height: 0,
        }
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Jet> {
    input
        .chars()
        .map(|c| if c == '<' { Jet::Left } else { Jet::Right })
        .collect()
}

// ---------------------------------------------------------------------------
#[aoc(day17, part1)]
pub fn part1(input: &Vec<Jet>) -> usize {
    let mut chamber = Chamber::default();
    chamber.jets = input.clone();
    chamber.play(2022);
    chamber.height
}

#[aoc(day17, part2)]
pub fn part2(input: &Vec<Jet>) -> usize {
    let mut chamber = Chamber::default();
    chamber.jets = input.clone();
    // This will not work
    //chamber.play(1000000000000);
    chamber.height
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_rock() {
        let mut r = Rock {
            sprite: plus,
            prev: 0,
        };
        r.print();
        r.shift(&Jet::Left);
        r.print();
        r.shift(&Jet::Left);
        r.print();
        // Unable to shift Left more
        let left = r.sprite;
        r.shift(&Jet::Left);
        r.print();
        assert_eq!(left, r.sprite);
        r.shift(&Jet::Right);
        r.print();
        r.shift(&Jet::Right);
        r.print();
        r.shift(&Jet::Right);
        r.print();
        r.shift(&Jet::Right);
        r.print();
        // Unable to shift right more
        let end = r.sprite;
        r.shift(&Jet::Right);
        assert_eq!(end, r.sprite);
    }

    #[test]
    fn test_generator() {
        let input = input_generator(INPUT);
    }
    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(3068, part1(&input));
    }
    #[test]
    fn test_part2() {
        let input = input_generator(INPUT);
        assert_eq!(1514285714288, part2(&input));
    }
    #[test]
    fn test_chamber() {
        let mut chamber = Chamber::default();
        assert_eq!(0, chamber.height);
        chamber.map[0] = 0xf0;
        assert_eq!(1, chamber.height);
    }
}
