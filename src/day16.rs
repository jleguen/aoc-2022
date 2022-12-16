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
use petgraph::{graphmap::UnGraphMap};

// ---------------------------------------------------------------------------
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Valve {
    name: &str,
    flow: usize,
}

#[derive(Debug)]
pub struct ParsedValve {
    name: &str,
    flow: usize,
    neighbours: Vec<String>,
}

impl Valve {
    fn from(v: &ParsedValve) -> Self {
        Valve { name: v.name.clone(), flow: v.flow }
    }
}


pub struct Map {
    valves: Vec<Valve>,
    graph: UnGraphMap<Valve, ()>,
}

impl Map {
    fn new(valves: &Vec<ParsedValve>) -> Self {
        let graph: UnGraphMap<Valve, ()> = UnGraphMap::new();
        Map { valves: valves.iter().map(|v| Valve::from(v)).collect(), graph }
    }
}

// ---------------------------------------------------------------------------
fn parse_line(line: &str) -> ParsedValve {
    let (v, t) = line.split_once(';').expect("Line");
    //  "Valve BB has flow rate=13"
    let elems: Vec<&str> = v.split_ascii_whitespace().collect();
    let name = elems[1];
    let flow = elems[4].split_once('=').expect("Flow").1.parse::<usize>().unwrap();
    //  " tunnels lead to valves CC, AA"
    let neighbours = t.split_ascii_whitespace().skip(4).map(|s| String::from(s.trim_matches(','))).collect();

    ParsedValve{ name, flow, neighbours }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Map {
    let mut valves: Vec<ParsedValve> = Vec::new();
    for line in input.lines() {
        valves.push(parse_line(&line));
    }

    println!("{:#?}", valves);
    Map::new(&valves)
}

// ---------------------------------------------------------------------------
#[aoc(day16, part1)]
pub fn part1(input: &Map) -> usize {
    0
}

#[aoc(day16, part2)]
pub fn part2(input: &Map) -> usize {
    0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_generator() {
        let input = input_generator(INPUT);
    }
    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(1651, part1(&input));
    }
    #[test]
    fn test_part2() {
    }
}
