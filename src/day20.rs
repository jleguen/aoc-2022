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
use std::io::{Write, stdout};

// ---------------------------------------------------------------------------
#[derive(Display, Debug, Clone, Copy)]
#[display("{value} ({moved}) [{orig} {pos}]")]
pub struct Number {
    value: isize,
    orig: usize,
    pos: usize,
    moved: bool,
}

#[derive(Display, Debug, Clone, Copy)]
#[display("{value} ({moved})")]
pub struct Num {
    value: isize,
    moved: bool,
}

impl Num {
    fn from(value: isize) -> Self {
        Num {
            value,
            moved: false,
        }
    }
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
        //println!("Index {index} is {i} (len {size})");
        &self.orig[i as usize]
    }
}

impl IndexMut<isize> for Ring {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let size = self.orig.len() as isize;
        let i = index.rem_euclid(size);
        //println!("Index {index} is {i} (len {size})");
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
// uses rem_euclid
fn pos_rem(vec: &VecDeque<Num>, pos: isize) -> usize {
    let res = pos.rem_euclid(vec.len() as isize).try_into().unwrap();
    //println!("+ REM Pos {pos} len {} {res}", vec.len());
    res
}

fn move_num(vec: &mut VecDeque<Num>, pos: isize) {
    let from: usize = pos_rem(vec, pos);
    let mut amount = vec[from].value;
    if amount < 0 {
        amount -= 1;
    }
    let mut to: usize = pos_rem(vec, pos + amount);
    
    //println!("Move elem pos {pos} from {from} to {to}");
        stdout().flush();
    let mut elem = vec.remove(from).unwrap();
    elem.moved = true;
    vec.insert(to, elem);
}

// ---------------------------------------------------------------------------
#[aoc_generator(day20, part1, vecdeque)]
pub fn input_generator_vecdeque(input: &str) -> VecDeque<Num> {
    let mut vec = VecDeque::new();
    for line in input.lines() {
        vec.push_back(Num::from(line.parse().unwrap()));
    }
    vec
}

#[aoc_generator(day20, part1, ring)]
pub fn input_generator_ring(input: &str) -> Ring {
    let mut vec = Vec::new();
    for line in input.lines() {
        vec.push(line.parse().unwrap());
    }
    Ring::from(&vec)
}

// ---------------------------------------------------------------------------
#[aoc(day20, part1, ring)]
pub fn part1_ring(input: &Ring) -> usize {
    0
}

#[aoc(day20, part1, vecdeque)]
pub fn part1_vecdeque(input: &VecDeque<Num>) -> isize {
    let mut vec = input.clone();
    let mut index = 0;
    loop {
        //stdout().flush();
        //println!("{:?}", vec.iter().map(|e| e.value).collect::<Vec<isize>>());
        let value = vec[index].value;
        let moved = vec[index].moved;
        //println!("  Index {index} value {value} moved {moved}");
        if moved == true {
            //println!("  ! Skip");
            index += 1;
            if index >= vec.len() {
                //println!("  !! the end");
                break;
            }
        } else {
            move_num(&mut vec, index as isize);
        }
    }

    let zero = vec
        .iter()
        .enumerate()
        .filter(|(_, v)| v.value == 0)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()[0] as isize;
    //println!("{:?}", zero);

    let one = vec[pos_rem(&vec, 1000 + zero)].value;
    let two = vec[pos_rem(&vec, 2000 + zero)].value;
    let three = vec[pos_rem(&vec, 3000 + zero)].value;
    println!("{one} {two} {three} => {}", one + two + three);
    one + two + three
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
    fn test_generator_vecdeque() {
        let input = input_generator_vecdeque(INPUT);
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
        assert_eq!(3, part1_vecdeque(&input));
    }
}
