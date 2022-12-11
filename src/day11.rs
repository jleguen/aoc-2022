use parse_display;
//use std::collections::HashMap;
//use std::fmt;
//use std::str::FromStr;
//use std::string::ParseError;
//use std::sync::{Arc, Mutex};
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};
//use num_bigint::{u64, Tou64};
//use std::ops::{Rem, Div};

type Items = Vec<u64>;



// ---------------------------------------------------------------------------

#[derive(parse_display::FromStr, parse_display::Display, Debug, Clone)]
enum Operation {
    #[display("old + {0}")]
    Add(u64),
    #[display("old * {0}")]
    Mul(u64),
    #[display("old * old")]
    Sqr,
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Self::Add(amt) => {
                //println!("    Worry level increases by {amt} to {}", value + amt);
                value + amt
            }
            Self::Mul(amt) => {
                //println!("    Worry level is multiplied by {amt} to {}", value * amt);
                value * amt
            }
            Self::Sqr => {
                //println!( "    Worry level is multiplied by itself to {}", value * value);
                value.pow(2)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    index: usize,
    inspected: usize,
    op: Operation,
    test: u64,
    to_true: usize,
    to_false: usize,
}


impl Monkey {
    // Test worry and throw to correct monkey
    fn test(&self, value: &u64) -> usize {
        if 0 == value % self.test {
            //println!("    Current worry level is divisible by {}.", self.test);
            let m = self.to_true;
            //println!("    Item with worry level {value} is thrown to monkey {m}.");
            m
        } else {
            //println!("    Current worry level is not divisible by {}.", self.test);
            let m = self.to_false;
            //println!("    Item with worry level {value} is thrown to monkey {m}.");
            m
        }
    }

}

// ---------------------------------------------------------------------------
#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> (Vec<Monkey>, Vec<Items>) {
    let mut res: Vec<Monkey> = Vec::new();
    let mut items: Vec<Items> = Vec::new();

    // Iterate over monkeys
    for m in input.split("\n\n") {
        let mut lines = m.lines();
    /*
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
    */
        let index: usize = lines
            .next()
            .expect("Monkey")
            .split_once(" ")
            .unwrap()
            .1
            .trim_end_matches(':')
            .parse()
            .unwrap();
        let item: Vec<u64> = lines
            .next()
            .expect("Items")
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|v| v.parse().unwrap())
            .collect();
        let op: Operation = lines
            .next()
            .expect("Operation")
            .split_once("= ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let test: u64 = lines
            .next()
            .expect("Test")
            .rsplit_once(" ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let to_true: usize = lines
            .next()
            .expect("True")
            .rsplit_once(" ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let to_false: usize = lines
            .next()
            .expect("False")
            .rsplit_once(" ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        res.push(Monkey {
            index,
            inspected: 0,
            op,
            test,
            to_true,
            to_false,
        });
        items.push(item);
    }

    (res, items)
}

// ---------------------------------------------------------------------------
#[aoc(day11, part1)]
pub fn part1(input: &(Vec<Monkey>, Vec<Items>)) -> usize {
    let (m, i) = input;
    let mut monkeys: Vec<Monkey> = m.clone();
    let mut items: Vec<Items> = i.clone();
    println!("=================================================================");

    for i in 1..=20 {
        for m in monkeys.iter_mut() {
            let mut transfert: Vec<(usize, u64)> = Vec::new();
            while let Some(item) = items.get_mut(m.index).expect("Items").pop() {
                //println!("  Monkey inspects an item with a worry level of {}", item);
                m.inspected += 1;
                // Worry level
                let mut new = m.op.apply(item);
                new = new/3;
                //println!("    Monkey gets bored with item. Worry level is divided by 3 to {new}");
                let to = m.test(&new);
                transfert.push((to, new));
            }
            for (to, item) in transfert {
                items.get_mut(to).expect("Monkey").push(item);
            }
        }
        println!("\nAfter round {i}, the monkeys are holding items with these worry levels:");
        for m in monkeys.iter() {
            println!("{} {:?}", m.index, items.get(m.index).unwrap());
        }
    }

    let mut ins: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    ins.sort();
    ins.reverse();
    ins[0] * ins[1]
}

#[aoc(day11, part2)]
pub fn part2(input: &(Vec<Monkey>, Vec<Items>)) -> usize {
    let (m, i) = input;
    let mut monkeys: Vec<Monkey> = m.clone();
    let mut items: Vec<Items> = i.clone();
    let divisor: u64 = monkeys.iter().map(|m| m.test).product();
    println!("Divisor {}", divisor);
    println!("=================================================================");

    for i in 1..=10000 {
        print!(".");
        for m in monkeys.iter_mut() {
            let mut transfert: Vec<(usize, u64)> = Vec::new();
            while let Some(item) = items.get_mut(m.index).expect("Items").pop() {
                //println!("  Monkey inspects an item with a worry level of {}", item);
                m.inspected += 1;
                // Worry level
                let mut new = m.op.apply(item);
                let to = m.test(&new);
                new = new % divisor;
                //println!("    Monkey gets bored with item. Worry level is divided to {new}");
                transfert.push((to, new));
            }
            for (to, item) in transfert {
                items.get_mut(to).expect("Monkey").push(item);
            }
        }
        if 0 == i % 1000 || i == 20 || i == 1 {
            println!("\nAfter round {i}, the monkeys are holding items with these worry levels:");
            for m in monkeys.iter() {
                println!("{} {:?}", m.index, items.get(m.index).unwrap());
            }
        }
    }

    let mut ins: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    ins.sort();
    ins.reverse();
    ins[0] * ins[1]
}


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
        let inst = input_generator(INPUT);
        println!("{:#?}", inst);
        assert_eq!(4, inst.0.len());
    }
    #[test]
    fn test_part1() {
        let m = input_generator(INPUT);
        let res = part1(&m);
        assert_eq!(10605, res);
    }
    #[test]
    fn test_part2() {
        let m = input_generator(INPUT);
        let res = part2(&m);
        assert_eq!(2713310158, res);
    }
}
