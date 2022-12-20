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
use std::ops::Index;

// ---------------------------------------------------------------------------
#[derive(Display, Debug, Clone)]
enum Robot {
    #[display("{0} ore-collecting robot")]
    OreBot(usize),
    #[display("{0} clay-collecting robot")]
    ClayBot(usize),
    #[display("{0} obsidian-collecting robot")]
    ObsidianBot(usize),
    #[display("{0} geode-cracking robot")]
    GeodeBot(usize),
}

#[derive(Display, FromStr, Debug, Clone)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Index<Material> for Cost {
    type Output = usize;
    fn index(&self, material: Material) -> &Self::Output {
        match material {
            Material::Ore => &self.ore,
            Material::Clay => &self.clay,
            Material::Obsidian => &self.obsidian,
            Material::Geode => &0,
        }
    }
}

impl Default for Cost {
    fn default() -> Self {
        Cost { ore: 0, clay: 0, obsidian: 0 }
    }
}

#[derive(Debug, Clone)]
struct Node {
    // Number of active robots
    robots: [Robot; 4],
    // amount of each Material
    material: [Material; 4],
}
// ---------------------------------------------------------------------------
// Blueprint with cost of a construction
#[derive(Debug)]
pub struct Blueprint {
    id: usize,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

impl Blueprint {
    // Parse material cost
    // "Each obsidian robot costs 3 ore and 14 clay."
    fn cost(line: &str) -> Cost {
        let mut res = Cost::default();
        let costs = line.split(" costs ").last().unwrap()
            .trim_end_matches('.')
            .split(" and ").map(|s| s.parse::<Material>()).map(|m| res[]);
        println!("{:?}", costs);
        res
    }
    fn from(line: &str) -> Self {
        let id: usize = line
            .split_once(':').unwrap()
            .0.split_ascii_whitespace().last().unwrap()
            .parse().unwrap();
        let mut it = line.split_once(':').unwrap().1.split(".");
        let ore_robot = Self::cost(it.next().unwrap());
        let clay_robot = Self::cost(it.next().unwrap());
        let obsidian_robot = Self::cost(it.next().unwrap());
        let geode_robot = Self::cost(it.next().unwrap());
        Blueprint { id, ore_robot, clay_robot, obsidian_robot, geode_robot }
    }
}

// ---------------------------------------------------------------------------

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Blueprint> {
    let mut res: Vec<Blueprint> = Vec::new();
    for line in input.lines() {
        res.push(Blueprint::from(line));
    }
    println!("{:#?}", res);
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
