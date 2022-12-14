use parse_display::Display;
//use std::collections::HashMap;
//use std::fmt;
use std::str::FromStr;
use std::string::ParseError;
//use std::sync::{Arc, Mutex};
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};
//use num_bigint::{u64, Tou64};
//use std::ops::{Rem, Div};
//use serde_json::{Number, Value};
//use std::cmp::Ordering;
//use std::iter::zip;

type Coord = (usize, usize);

// ---------------------------------------------------------------------------
#[derive(Display, Copy, Clone, Debug, PartialEq)]
pub enum Element {
    #[display(".")]
    Air,
    #[display("#")]
    Rock,
    #[display("o")]
    Sand,
    #[display("+")]
    Origin,
}

impl Default for Element {
    fn default() -> Self {
        Element::Air
    }
}

#[derive(Debug, Clone)]
pub struct Cave {
    map: Box<[[Element; 1000]; 1000]>,
    topleft: Coord,
    botright: Coord,
}

impl Cave {
    const ORIGIN: Coord = (500, 0);

    // Drop one grain of sand, returns true if it settles
    fn drop(&mut self) -> bool {
        let mut p = Self::ORIGIN;
        loop {
            if p.1 >= self.botright.1 {
                return false;
            }
            // Find next position
            let down = (p.0, p.1 + 1);
            if self.get(down) == Element::Air {
                p = down;
                continue;
            }
            let left = (p.0 - 1, p.1 + 1);
            if self.get(left) == Element::Air {
                p = left;
                continue;
            }
            let right = (p.0 + 1, p.1 + 1);
            if self.get(right) == Element::Air {
                p = right;
                continue;
            } else {
                break;
            }
        }
        if p == Self::ORIGIN {
            return false;
        }
        self.set(p, Element::Sand);
        return true;
    }

    fn set(&mut self, pos: Coord, elem: Element) {
        self.map[pos.1][pos.0] = elem;
        if self.topleft.0 > pos.0 {
            self.topleft.0 = pos.0;
        }
        if self.topleft.1 > pos.1 {
            self.topleft.1 = pos.1;
        }
        if self.botright.0 < pos.0 {
            self.botright.0 = pos.0;
        }
        if self.botright.1 < pos.1 {
            self.botright.1 = pos.1;
        }
    }

    fn get(&self, pos: Coord) -> Element {
        self.map[pos.1][pos.0]
    }

    fn add_floor(&mut self) {
        let floor = self.botright.1 + 2;
        self.add_rock_segment(&(0, floor), &(999, floor));
    }

    fn add_rock_segment(&mut self, from: &Coord, to: &Coord) {
        //println!("+ Rock segment from {:?} to {:?}", from, to);
        let dx = to.0 as i32 - from.0 as i32;
        let dy = to.1 as i32 - from.1 as i32;
        let amt = dx.abs().max(dy.abs());
        let step_x = dx / amt;
        let step_y = dy / amt;
        //println!("  X {dx} {step_x} | Y {dy} {step_y}");
        for i in 0..=amt {
            let p = (
                (from.0 as i32 + (i * step_x)) as usize,
                (from.1 as i32 + (i * step_y)) as usize,
            );
            //println!("  {:?}", p);
            self.set(p, Element::Rock);
        }
    }

    fn add_rock_line(&mut self, points: &Vec<Coord>) {
        let mut it = points.iter();
        let mut from = it.next().expect("Points");
        while let Some(to) = it.next() {
            self.add_rock_segment(&from, &to);
            from = to;
        }
    }

    fn area(&self) -> (Coord, Coord) {
        (self.topleft, self.botright)
    }

    fn display_area(&self, topleft: Coord, botright: Coord) {
        println!("Cave {:?} to {:?}", topleft, botright);
        for y in topleft.1..=botright.1 {
            print!("{y:>4} ");
            for x in topleft.0..=botright.0 {
                print!("{}", self.get((x, y)));
            }
            print!("\n");
        }
    }

    fn display(&self) {
        let (topleft, botright) = self.area();
        self.display_area(topleft, botright);
    }
}

impl FromStr for Cave {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cave = Cave {
            map: Box::new([[Element::default(); 1000]; 1000]),
            topleft: (999, 999),
            botright: (0, 0),
        };

        // Origin
        cave.set(Self::ORIGIN, Element::Origin);

        for line in input.lines() {
            let mut points: Vec<Coord> = Vec::new();
            for segment in line.split(" -> ") {
                if let Some((x, y)) = segment.split_once(",") {
                    points.push((x.parse().unwrap(), y.parse().unwrap()));
                }
            }
            cave.add_rock_line(&points);
        }

        Ok(cave)
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Cave {
    input.parse().expect("Input")
}

// ---------------------------------------------------------------------------
#[aoc(day14, part1)]
pub fn part1(input: &Cave) -> usize {
    let mut count = 0;
    let mut cave = input.clone();
    while cave.drop() {
        count += 1;
    }
    cave.display();
    count
}

#[aoc(day14, part2)]
pub fn part2(input: &Cave) -> usize {
    let mut count = 0;
    let mut cave = input.clone();
    cave.add_floor();
    while cave.drop() {
        count += 1;
    }
    cave.display();
    count + 1
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_generator() {
        let cave = input_generator(INPUT);
        cave.display();
    }
    #[test]
    fn test_part1() {
        let cave = input_generator(INPUT);
        assert_eq!(24, part1(&cave));
    }
    #[test]
    fn test_part2() {
        let cave = input_generator(INPUT);
        assert_eq!(93, part2(&cave));
    }
}
