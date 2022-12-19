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

// ---------------------------------------------------------------------------
// Cost of a construction
#[derive(Debug, Display, FromStr)]
pub struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

pub struct Blueprint {
    id: usize,
    ore_robot: Cost,
}
// ---------------------------------------------------------------------------
enum Robot {
    OreBot,
    ClayBot,
    ObsidianBot,
    GeodeBot,
}

struct Node {
    robots: [usize; 4],
    amount: [usize; 4],
}
// ---------------------------------------------------------------------------

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Blueprint> {
    let mut res: Vec<Blueprint> = Vec::new();
    for line in input.lines() {
        res.push()
    }
    res
}

// ---------------------------------------------------------------------------
#[aoc(day16, part1)]
pub fn part1(input: &Vec<Blueprint>) -> usize {
    0
}

#[aoc(day16, part2)]
pub fn part2(input: &Vec<Blueprint>) -> usize {
    0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.";
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
    fn test_part2() {}
}
