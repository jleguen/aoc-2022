use parse_display;
//use std::collections::HashMap;
//use std::fmt;
use std::str::FromStr;
use std::string::ParseError;
//use std::sync::{Arc, Mutex};
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};
//use num_bigint::{u64, Tou64};
//use std::ops::{Rem, Div};
use std::iter::zip;

#[derive(Debug)]
pub enum Item {
    Int(usize),
    List(Vec<Item>),
}

impl Item {
    // returns true if left <= right
    fn compare(&self, other: &Item) -> bool {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => left <= right,
            (Self::Int(left), Self::List(right)) => {
                Self::List(vec![Self::Int(*left)]).compare(other)
            }
            (Self::List(left), Self::Int(right)) => {
                self.compare(&Self::List(vec![Self::Int(*right)]))
            }
            (Self::List(left), Self::List(right)) => {
                for (one, two) in zip(left, right) {
                    if one.compare(two) == false {
                        return false;
                    }
                }
                true
            }
        }
    }
}

/* TODO
impl FromStr for Item {
    type Err = ParseErr;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with("[") {
            let mut res = Vec::new();
        }
  }
}
*/
// ---------------------------------------------------------------------------



// ---------------------------------------------------------------------------
#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<(Item, Item)> {
    let res = Vec::new();
    for pair in input.split("\n\n") {
        for line in pair.lines() {

        }
    }
    res
}

// ---------------------------------------------------------------------------
#[aoc(day13, part1)]
pub fn part1(input: &Vec<(Item, Item)>) -> usize {
    0
}

#[aoc(day13, part2)]
pub fn part2(input: &Vec<(Item, Item)>) -> usize {
    0
}


// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_generator() {
    }
    #[test]
    fn test_item_int() {
        assert!(Item::Int(0).compare(&Item::Int(0)));
        assert!(Item::Int(0).compare(&Item::Int(1)));
        assert_eq!(false, Item::Int(1).compare(&Item::Int(0)));
    }
    fn test_item_list() {
        let one = Item::List(vec![Item::Int(0)]);
        let two = Item::List(vec![Item::Int(1)]);
        assert!(one.compare(&two));
    }
}
