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
//use petgraph::{graphmap::UnGraphMap};

// ---------------------------------------------------------------------------

#[derive(Debug, FromStr, Display)]
#[display("{x},{y},{z}")]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    // true if points are neighbours:
    // - two coordinates are identical
    // - one coordinate is +/- 1
    fn is_neighbour(&self, other: &Point) -> bool {
        (self.x == other.x && self.y == other.y && (self.z - other.z).abs() == 1) ||
        (self.x == other.x && (self.y - other.y).abs() == 1 && self.z == other.z) ||
        ((self.x - other.x).abs() == 1 && self.y == other.y && self.z == other.z)
    }

    fn neighbours(&self) -> Vec<Point> {
        let res = vec![
            Point {x: self.x+1, y: self.y, z: self.z},
            Point {x: self.x-1, y: self.y, z: self.z},
            Point {x: self.x, y: self.y+1, z: self.z},
            Point {x: self.x, y: self.y-1, z: self.z},
            Point {x: self.x, y: self.y, z: self.z+1},
            Point {x: self.x, y: self.y, z: self.z-1},
        ];
        res
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Point> {
    let mut res: Vec<Point> = Vec::new();
    for line in input.lines() {
        let p: Point = line.parse().unwrap();
        res.push(p);
    }
    //println!("{:?}", res);
    res
}

// ---------------------------------------------------------------------------
#[aoc(day18, part1)]
pub fn part1(input: &Vec<Point>) -> usize {
    let mut faces = input.len() * 6;
    for v in input.iter() {
        let count = input.iter().map(|p| v.is_neighbour(p)).filter(|v| *v).count();
        faces -= count;
    }
    faces
}

#[aoc(day18, part2)]
pub fn part2(input: &Vec<Point>) -> usize {
    let visited = Vec::<Point>::new();
    0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_generator() {
        let input = input_generator(INPUT);
    }
    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(64, part1(&input));
    }
    #[test]
    fn test_part2() {
        let input = input_generator(INPUT);
        assert_eq!(58, part1(&input));
    }
}
