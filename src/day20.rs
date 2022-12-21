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
//use petgraph::graph::{Graph, NodeIndex, UnGraph};
//use petgraph::prelude::*;
//use petgraph::visit::{depth_first_search, Control, DfsEvent};
use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

// ---------------------------------------------------------------------------
#[derive(Display, Debug, Clone, Copy)]
#[display("{value} ({moved}) [{orig} {pos}]")]
pub struct Number {
    value: isize,
    orig: usize,
    pos: usize,
    moved: bool,
}
// ---------------------------------------------------------------------------
#[derive(Debug)]
pub struct Ring {
    orig: Vec<Number>,
}

impl Index<isize> for Ring {
    type Output = Number;
    fn index(&self, index: isize) -> &Self::Output {
        let size = self.orig.len() as isize;
        let i = index.rem_euclid(size);
        println!("Index {index} is {i} (len {size})");
        &self.orig[i as usize]
    }
}

impl IndexMut<isize> for Ring {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let size = self.orig.len() as isize;
        let i = index.rem_euclid(size);
        println!("Index {index} is {i} (len {size})");
        &mut self.orig[i as usize]
    }
}

impl Ring {
    fn from(input: &Vec<isize>) -> Self {
        let mut vec = Vec::new();
        for (i, e) in input.iter().enumerate() {
            vec.push(Number {
                value: *e,
                pos: i,
                orig: i,
                moved: false,
            });
        }
        Ring { orig: vec }
    }
    fn len(&self) -> usize {
        self.orig.len()
    }
}
// ---------------------------------------------------------------------------

fn move_elem(vec: &mut VecDeque<Number>, pos: isize) {
    let from: usize = pos.rem_euclid(vec.len() as isize) as usize;
    let to: usize = (pos + vec[from].value).rem_euclid(vec.len() as isize) as usize;
    let mut rem = vec.split_off(from);
    let mut elem = rem.pop_front().unwrap();
    elem.moved = true;
    if to < from {
        // Easy
        vec.insert(to, elem);
        vec.append(&mut rem);
    } else {
        // Harder
        rem.insert(to - vec.len() - 1, elem);
        vec.append(&mut rem);
    }
}
// ---------------------------------------------------------------------------
#[aoc_generator(day16, part1, vecdeque)]
pub fn input_generator_vecdeque(input: &str) -> VecDeque<isize> {
    let mut vec = VecDeque::new();
    for line in input.lines() {
        vec.push_back(line.parse().unwrap());
    }
    vec
}

#[aoc_generator(day16, part1, ring)]
pub fn input_generator_ring(input: &str) -> Ring {
    let mut vec = Vec::new();
    for line in input.lines() {
        vec.push(line.parse().unwrap());
    }
    Ring::from(&vec)
}

// ---------------------------------------------------------------------------
#[aoc(day16, part1, ring)]
pub fn part1_ring(input: &Ring) -> usize {
    0
}

#[aoc(day16, part1, vecdeque)]
pub fn part1_vecdeque(input: &VecDeque<isize>) -> usize {
    0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_generator() {
        let input = input_generator_ring(INPUT);
        assert_eq!(7, input.len());
    }

    #[test]
    fn test_ring() {
        let input = input_generator_ring(INPUT);
        assert_eq!(1, input[0].value);
        assert_eq!(4, input[-1].value);
        assert_eq!(-3, input[3000 * 7 + 2].value);
        assert_eq!(-3, input[-3000 * 7 + 2].value);
    }

    #[test]
    fn test_part1_vecdeque() {
        let input = input_generator_vecdeque(INPUT);
        assert_eq!(1651, part1_vecdeque(&input));
    }
}
