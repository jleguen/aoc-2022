use parse_display;
//use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};

// ---------------------------------------------------------------------------
#[derive(parse_display::FromStr, parse_display::Display, Debug)]
enum Operation {
    #[display("old + {0}")]
    Add(i32),
    #[display("old * {0}")]
    Mul(i32),
    #[display("old * old")]
    Sqr,
}

#[derive(Debug)]
pub struct Monkey {
    index: usize,
    items: Vec<i32>,
    op: Operation,
}

impl FromStr for Monkey {
    type Err = ParseError;

    /*
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
    */
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let index: usize = lines
            .next()
            .expect("Monkey")
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let items: Vec<i32> = lines
            .next()
            .expect("Items")
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        let op: Operation = lines
            .next()
            .expect("Operation")
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        Ok(Monkey { index, items, op })
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    let res = Vec::new();

    // Iterate over monkeys
    for m in input.split("\n\n") {}

    res
}

// ---------------------------------------------------------------------------
/*
#[aoc(day11, part1)]
pub fn part1(input: &Vec<Inst>) -> i64 {
}
*/

/*
#[aoc(day11, part2)]
pub fn part2(input: &Vec<Inst>) -> i64 {
}
*/

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_generator() {
        let mut inst = input_generator(INPUT);
    }
}
