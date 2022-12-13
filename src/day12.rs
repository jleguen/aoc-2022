//use itertools::Itertools;
//use parse_display::{Display, FromStr};
//use std::collections::HashMap;
//use std::string::ParseError;
use toodee::{Coordinate, TooDee, TooDeeOps};

use pathfinding::prelude::dijkstra;

pub struct Map {
    height: TooDee<u32>,
    start: Coordinate,
    lows: Vec<Coordinate>,
    goal: Coordinate,
}

impl Map {
    // Return list of valid successors in a path
    fn valid_successors(&self, pos: Coordinate) -> Vec<(Coordinate, usize)> {
        let mut res = Vec::new();
        let cur = self.height[pos];
        if pos.0 > 0 {
            let new = (pos.0 - 1, pos.1);
            if self.height[new] <= cur + 1 {
                res.push((new, 1));
            }
        }
        if pos.0 < self.height.num_cols() - 1 {
            let new = (pos.0 + 1, pos.1);
            if self.height[new] <= cur + 1 {
                res.push((new, 1));
            }
        }
        if pos.1 > 0 {
            let new = (pos.0, pos.1 - 1);
            if self.height[new] <= cur + 1 {
                res.push((new, 1));
            }
        }
        if pos.1 < self.height.num_rows() - 1 {
            let new = (pos.0, pos.1 + 1);
            if self.height[new] <= cur + 1 {
                res.push((new, 1));
            }
        }
        res
    }

    fn path(&self, start: Coordinate) -> Option<(Vec<Coordinate>, usize)> {
        dijkstra(&start, |p| self.valid_successors(*p), |p| *p == self.goal)
    }
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Map {
    let mut height = TooDee::new(0, 0);
    let mut start: Coordinate = (0, 0);
    let mut goal: Coordinate = (0, 0);
    let mut lows: Vec<Coordinate> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut vals: Vec<u32> = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            vals.push(match ch {
                'S' => {
                    start = (col, row);
                    1
                }
                'E' => {
                    goal = (col, row);
                    26
                }
                'a' => {
                    lows.push((col, row));
                    1
                }
                _ => (ch as u32) - ('a' as u32) + 1,
            });
        }

        height.push_row(vals);
    }

    Map {
        height,
        start,
        lows,
        goal,
    }
}

// ---------------------------------------------------------------------------
#[aoc(day12, part1)]
pub fn part1(input: &Map) -> usize {
    input.path(input.start).expect("No path found").1
}

#[aoc(day12, part2)]
pub fn part2(input: &Map) -> usize {
    let mut res: Vec<usize> = Vec::new();
    for start in input.lows.iter() {
        if let Some((_, length)) = input.path(*start) {
            res.push(length);
        }
    }
    res.sort();
    println!("{:?}", res);
    res[0]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_generator() {
        let map = input_generator(INPUT);
        assert_eq!(5, map.height.num_rows());
        assert_eq!(8, map.height.num_cols());
        assert_eq!((0, 0), map.start);
        assert_eq!((5, 2), map.goal);
    }

    #[test]
    fn test_successors() {
        let map = input_generator(INPUT);
        assert_eq!(2, map.valid_successors((0, 0)).len());
    }

    #[test]
    fn test_path() {
        let map = input_generator(INPUT);
        let result = map.path((0, 0)).expect("No path found");
        println!("{:?}", result.1);
        println!("{:?}", result.0);
        assert_eq!(31, result.1);
    }
}
