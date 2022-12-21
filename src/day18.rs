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
use bitmaps::Bitmap;

// ---------------------------------------------------------------------------
#[derive(Debug, Default)]
pub struct Droplet {
    xs: Bitmap<32>,
    ys: Bitmap<32>,
    zs: Bitmap<32>,
}

impl Droplet {
    fn add(&mut self, p: &Point) {
        self.xs.set(p.x, true);
        self.ys.set(p.y, true);
        self.zs.set(p.z, true);
    }

    fn bitmap_faces(bitmap: &Bitmap<32>) -> usize {
        let mut faces = 0;
        // For each index, if true and neighbour is false -> add one face
        let first = bitmap.first_index().unwrap();
        let last = bitmap.last_index().unwrap();
        let true_indices: Vec<usize> = bitmap.into_iter().collect();
        for i in true_indices {
            println!("{} L {} R {}", i, bitmap.get(i-1), bitmap.get(i+1));
            if i == first || bitmap.get(i-1) == false { faces += 1; }
            if i == last || bitmap.get(i+1) == false { faces += 1; }
        }
        faces
    }

    fn area(&self) -> usize {
        Self::bitmap_faces(&self.xs) + 
        Self::bitmap_faces(&self.ys) + 
        Self::bitmap_faces(&self.zs)
    }
}

#[derive(Debug, FromStr, Display)]
#[display("{x},{y},{z}")]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

// ---------------------------------------------------------------------------
#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Droplet {
    let mut drop: Droplet = Droplet::default();
    let mut maxx = 0;
    let mut maxy = 0;
    let mut maxz = 0;
    for line in input.lines() {
        let p: Point = line.parse().unwrap();
        drop.add(&p);
    }
    println!("{:?}", drop);
    drop
}

// ---------------------------------------------------------------------------
#[aoc(day18, part1)]
pub fn part1(input: &Droplet) -> usize {
    input.area()
}

#[aoc(day18, part2)]
pub fn part2(input: &Droplet) -> usize {
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
    }
}
