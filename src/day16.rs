use parse_display::{Display, FromStr};
use std::collections::HashMap;
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
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use petgraph::prelude::*;
use petgraph::visit::{depth_first_search, Control, DfsEvent};

// ---------------------------------------------------------------------------
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Valve {
    flow: usize,
    index: NodeIndex,
}

#[derive(Debug)]
pub struct ParsedValve {
    name: String,
    flow: usize,
    neighbours: Vec<String>,
}

impl Valve {}

#[derive(Debug)]
pub struct Map {
    valves: HashMap<String, Valve>,
    graph: UnGraph<usize, usize>,
}

impl Map {
    fn new(input: &Vec<ParsedValve>) -> Self {
        let mut graph: UnGraph<usize, usize> = Graph::new_undirected();
        let mut valves: HashMap<String, Valve> = HashMap::new();
        // Nodes
        for parsed in input {
            let i = graph.add_node(parsed.flow);
            valves.insert(
                parsed.name.clone(),
                Valve {
                    flow: parsed.flow,
                    index: i,
                },
            );
        }
        // Edges
        for parsed in input {
            let index = valves.get(&parsed.name).expect("Valve").index;
            for n in parsed.neighbours.iter() {
                let i = valves.get(n).expect("Neighbour").index;
                graph.add_edge(index, i, 1);
            }
        }

        Map { valves, graph }
    }

    // Remove nodes with flow 0
    fn simplify(&mut self) {
        let mut graph: UnGraph<usize, usize> = Graph::new_undirected();
        let root = self.valves["AA"].index;
        let mut bfs = Bfs::new(&self.graph, root);
        let mut dist = 0;
        depth_first_search(&self.graph, Some(root), |event| {
            match event {
                DfsEvent::TreeEdge(u, v) => {
                    dist += 1;
                }
                DfsEvent::BackEdge
            }
        });
    }
}

// ---------------------------------------------------------------------------
fn parse_line(line: &str) -> ParsedValve {
    let (v, t) = line.split_once(';').expect("Line");
    //  "Valve BB has flow rate=13"
    let elems: Vec<&str> = v.split_ascii_whitespace().collect();
    let name = elems[1].to_string();
    let flow = elems[4]
        .split_once('=')
        .expect("Flow")
        .1
        .parse::<usize>()
        .unwrap();
    //  " tunnels lead to valves CC, AA"
    let neighbours = t
        .split_ascii_whitespace()
        .skip(4)
        .map(|s| String::from(s.trim_matches(',')))
        .collect();

    ParsedValve {
        name,
        flow,
        neighbours,
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Map {
    let mut valves: Vec<ParsedValve> = Vec::new();
    for line in input.lines() {
        valves.push(parse_line(&line));
    }

    let map = Map::new(&valves);

    println!("{:?}", map);
    map
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
    fn test_part2() {}
}
