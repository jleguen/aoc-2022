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
}

impl Rock {
    fn shift(&mut self, dir: &Jet) {
        match dir {
            Jet::Left => {
                if self.sprite & 0x80808080 == 0 {
                    self.sprite <<= 1;
                }
            }
            Jet::Right => {
                if self.sprite & 0x02020202 == 0 {
                    self.sprite >>= 1;
                }
            }
        }
    }

    fn print(&self) {
        println!("{:08b}", self.sprite & 0xff);
        println!("{:08b}", self.sprite >> 8 & 0xff);
        println!("{:08b}", self.sprite >> 16 & 0xff);
        println!("{:08b}", self.sprite >> 24 & 0xff);
        println!("");
    }
}

const dash: u32 = 0x3c_00_00_00; // 0b00111100, 0, 0, 0
const plus: u32 = 0x10_38_10_00; // 0b00010000, 0b00111000, 0b00010000, 0
const corner: u32 = 0x38_08_08_00; // 0b00111000, 0b00001000, 0b00001000, 0
const pipe: u32 = 0x20_20_20_20; // 0b00100000, 0b00100000, 0b00100000, 0b00100000
const square: u32 = 0x30_30_00_00; // 0b00110000, 0b00110000, 0, 0


#[derive(FromStr, Display, Debug)]
pub enum Jet {
    #[display("<")]
    Left,
    #[display(">")]
    Right,
}


// ---------------------------------------------------------------------------
struct Chamber {
    map: Vec<u32>,
}

impl Chamber {

}

// ---------------------------------------------------------------------------
#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Jet> {
    input.chars().map(|c| if c == '<' { Jet::Left } else { Jet::Right }).collect()
}

// ---------------------------------------------------------------------------
#[aoc(day17, part1)]
pub fn part1(input: &Vec<Jet>) -> usize {
    0
}

#[aoc(day17, part2)]
pub fn part2(input: &Vec<Jet>) -> usize {
    0
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
        let mut r = Rock { sprite: plus};
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
        assert_eq!(58, part1(&input));
    }
}
