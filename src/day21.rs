use parse_display::{Display, FromStr};
use std::collections::HashMap;
//use std::fmt;
//use std::str::FromStr;
//use std::string::ParseError;
//use std::sync::{Arc, Mutex};
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};
//use num_bigint::{u64, Tou64};
//use std::ops::{Rem, Div};
//use serde_json::{Number, Value};
//use std::cmp::Ordering;
//use std::iter::zip;
//use petgraph::graph::{Graph, NodeIndex, UnGraph};
//use petgraph::prelude::*;
//use petgraph::visit::DfsPostOrder;
//use std::collections::VecDeque;
//use std::ops::{Index, IndexMut};

// ---------------------------------------------------------------------------
#[derive(Debug, FromStr, Display)]
enum Operation {
    #[display("{0} + {1}")]
    Add(String, String),
    #[display("{0} - {1}")]
    Sub(String, String),
    #[display("{0} * {1}")]
    Mul(String, String),
    #[display("{0} / {1}")]
    Div(String, String),
    #[display("{0} = {1}")]
    Equ(String, String),
    #[display("{0}")]
    Say(i64),
}

// ---------------------------------------------------------------------------
#[derive(Debug)]
pub struct Monkeys(HashMap<String, Operation>);

impl Monkeys {
    // We rely on the stack to do its magic
    fn value(&self, monkey: &String) -> i64 {
        match self.0.get(monkey).unwrap() {
            Operation::Say(value) => *value,
            Operation::Add(one, two) => self.value(one) + self.value(two),
            Operation::Sub(one, two) => self.value(one) - self.value(two),
            Operation::Mul(one, two) => self.value(one) * self.value(two),
            Operation::Div(one, two) => self.value(one) / self.value(two),
            Operation::Equ(one, two) => (self.value(one) == self.value(two)) as i64,
        }
    }
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
#[aoc_generator(day21, part1)]
pub fn input_generator(input: &str) -> Monkeys {
    let mut map: HashMap<String, Operation> = HashMap::new();
    for line in input.lines() {
        let (name, op) = line.split_once(": ").unwrap();
        map.insert(String::from(name), op.parse().unwrap());
    }
    Monkeys(map)
}

#[aoc_generator(day21, part2)]
pub fn input_generator2(input: &str) -> Monkeys {
    let mut map: HashMap<String, Operation> = HashMap::new();
    for line in input.lines() {
        let (name, mut op) = line.split_once(": ").unwrap();
        if name == "root" {
            map.insert(String::from(name), op.replace("+", "=").parse().unwrap());
        } else {
            map.insert(String::from(name), op.parse().unwrap());
        }
    }
    Monkeys(map)
}

// ---------------------------------------------------------------------------
#[aoc(day21, part1)]
pub fn part1(input: &Monkeys) -> i64 {
    input.value(&String::from("root"))
}

#[aoc(day21, part2)]
pub fn part2(input: &Monkeys) -> i64 {
    0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_generator() {
        let input = input_generator(INPUT);
        println!("{:#?}", input);
        assert_eq!(15, input.0.len());
    }

    #[test]
    fn test_generator2() {
        let input = input_generator2(INPUT);
        println!("{:#?}", input);
        println!("{:?}", input.0.get(&String::from("root")));
    }

    #[test]
    fn test_value() {
        let input = input_generator(INPUT);
        assert_eq!(5, input.value(&String::from("dbpl")));
        assert_eq!(30, input.value(&String::from("drzm")));
    }

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(152, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = input_generator2(INPUT);
        //assert_eq!(301, part2(&input));
    }
}
