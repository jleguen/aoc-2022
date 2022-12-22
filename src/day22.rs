use parse_display::{Display, FromStr};
//use std::collections::HashMap;
//use std::fmt;
//use std::str::FromStr;
//use std::string::ParseError;
//use std::sync::{Arc, Mutex};
use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};
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
#[derive(FromStr, Display, Debug, Copy, Clone, PartialEq)]
enum Tile {
    #[display(".")]
    Open,
    #[display("#")]
    Wall,
    #[display(" ")]
    Off,
    #[from_str(ignore)]
    #[display("H")]
    Edge,
}

#[derive(FromStr, Display, Debug, Clone, Copy)]
enum Instruction {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
    #[display("{0}")]
    Forward(usize),
}

#[derive(Debug, Display)]
enum Direction {
    #[display("^")]
    Up,
    #[display(">")]
    Right,
    #[display("v")]
    Down,
    #[display("<")]
    Left,
}

// ---------------------------------------------------------------------------
#[derive(Debug, Display)]
#[display("{face}")]
pub struct Explorer {
    pos: Coordinate,
    face: Direction,
}

impl Explorer {
    fn turn_left(&mut self) {
        let new = match self.face {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        };
        self.face = new;
    }
    fn turn_right(&mut self) {
        let new = match self.face {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self.face = new;
    }
}
// ---------------------------------------------------------------------------
#[derive(Debug)]
pub struct Map {
    map: TooDee<Tile>,
    path: Vec<Instruction>,
    visited: Vec<Coordinate>,
    explorer: Explorer,
}

impl Map {
    fn print(&self) {
        for row in 0..self.map.num_rows() {
            print!("{:3} | ", row);
            for col in 0..self.map.num_cols() {
                if self.explorer.pos == (col, row) {
                    print!("{}", self.explorer);
                } else if self.visited.contains(&(col, row)) {
                    print!("@");
                } else {
                    print!("{}", self.map[(col, row)]);
                }
            }
            print!("\n");
        }
    }

    // Iterate on rows and cols to identify edges
    fn compute_edges(&mut self) {
        // rows
        for i in 1..self.map.num_rows() - 1 {
            let start = self.map[i].iter().position(|x| x != &Tile::Off).unwrap() - 1;
            let end = self.map[i].iter().rposition(|x| x != &Tile::Off).unwrap() + 1;
            //println!("{i} {start} {end}");
            self.map[i][start] = Tile::Edge;
            self.map[i][end] = Tile::Edge;
        }
        // cols
        for i in 1..self.map.num_cols() - 1 {
            let first = self.map.col(i).position(|x| x != &Tile::Off);
            if first == None {
                break;
            }
            let start = first.unwrap() - 1;
            let end = self.map.col(i).rposition(|x| x != &Tile::Off).unwrap() + 1;
            //println!("{i} {start} {end}");
            self.map[start][i] = Tile::Edge;
            self.map[end][i] = Tile::Edge;
        }
    }

    fn teleport(&self, from: &Coordinate, face: &Direction) -> Option<Coordinate> {
        //println!("map r {} c {}", self.map.num_rows(), self.map.num_cols());
        let new = match face {
            Direction::Right => {
                // First edge to the left
                let row = from.1;
                let mut col = usize::MAX;
                for i in (0..from.0).rev() {
                    if self.map[row][i] == Tile::Edge {
                        col = i + 1;
                        break;
                    } else {
                        continue;
                    }
                }
                (col, row)
            }
            Direction::Left => {
                // First edge to the right
                let row = from.1;
                let mut col = usize::MAX;
                for i in from.0..self.map.num_cols() {
                    if self.map[row][i] == Tile::Edge {
                        col = i - 1;
                        break;
                    } else {
                        continue;
                    }
                }
                (col, row)
            }
            Direction::Up => {
                // First edge to the bottom
                //println!("Looking up");
                let col = from.0;
                let mut row = usize::MAX;
                for i in from.1..self.map.num_rows() {
                    //println!("{}", self.map[i][col]);
                    if self.map[i][col] == Tile::Edge {
                        row = i - 1;
                        break;
                    } else {
                        continue;
                    }
                }
                (col, row)
            }
            Direction::Down => {
                // First edge to the top
                // First edge to the bottom
                //println!("Looking down");
                let col = from.0;
                let mut row = usize::MAX;
                for i in (0..from.1).rev() {
                    //println!("{}", self.map[i][col]);
                    if self.map[i][col] == Tile::Edge {
                        row = i + 1;
                        break;
                    } else {
                        continue;
                    }
                }
                (col, row)
            }
        };
        //println!("New pos {:?}", new);
        if self.map[new] == Tile::Wall {
            return None;
        }
        Some(new)
    }

    fn move_explorer(&mut self, value: usize) {
        for i in 1..=value {
            //println!("  {:?}", self.explorer.pos);
            // New position
            let new = match self.explorer.face {
                // (col, row)
                Direction::Up => (self.explorer.pos.0, self.explorer.pos.1 - 1),
                Direction::Right => (self.explorer.pos.0 + 1, self.explorer.pos.1),
                Direction::Down => (self.explorer.pos.0, self.explorer.pos.1 + 1),
                Direction::Left => (self.explorer.pos.0 - 1, self.explorer.pos.1),
            };
            // Edge detection
            match self.map[new] {
                Tile::Wall => {
                    //println!("Hit a wall");
                    return;
                }
                Tile::Edge => {
                    //println!("Teleportation ?");
                    if let Some(to) = self.teleport(&self.explorer.pos, &self.explorer.face) {
                        self.explorer.pos = to;
                    } else {
                        //println!("Hit a wall");
                        return;
                    }
                }
                Tile::Open => {
                    self.explorer.pos = new;
                }
                Tile::Off => {
                    panic!("Not supposed to happen");
                }
            }
            self.visited.push(self.explorer.pos);
        }
        //println!("New explorer position {:?}\n", self.explorer.pos);
    }

    fn step(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Left => self.explorer.turn_left(),
            Instruction::Right => self.explorer.turn_right(),
            Instruction::Forward(value) => self.move_explorer(*value),
        }
    }

    fn execute(&mut self) {
        for inst in self.path.clone().iter() {
            self.step(&inst);
            //self.print();
        }
    }
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Map {
    const LEN: usize = 165;
    let (raw, code) = input.split_once("\n\n").unwrap();
    // Map
    let mut map: TooDee<Tile> = TooDee::init(LEN, 1, Tile::Off);
    for line in raw.lines() {
        let mut vec = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect::<Vec<Tile>>();
        assert!(vec.len() <= LEN);
        vec.resize(LEN, Tile::Off);
        map.push_row(vec);
    }
    // First and last cols
    let col: Vec<Tile> = [Tile::Off].repeat(map.num_rows());
    assert_eq!(col.len(), map.num_rows());
    map.insert_col(0, col.clone());
    map.push_col(col);
    // Last line
    map.push_row([Tile::Off].repeat(map.num_cols()));

    // Path
    let path = code
        .replace('R', " R ")
        .replace('L', " L ")
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<Instruction>>();

    // Explorer
    let start = (map[1].iter().position(|x| x == &Tile::Open).unwrap(), 1);
    let explorer = Explorer {
        pos: start,
        face: Direction::Right,
    };

    let mut res = Map {
        map,
        path,
        visited: vec![explorer.pos],
        explorer,
    };
    res.compute_edges();
    res.execute();

    res
}

// ---------------------------------------------------------------------------
#[aoc(day22, part1)]
pub fn part1(input: &Map) -> usize {
    println!("{:?}", input.explorer);
    input.explorer.pos.0 * 4
        + input.explorer.pos.1 * 1000
        + match input.explorer.face {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}
/*
#[aoc(day22, part2)]
pub fn part2(input: &Map) -> i64 {
    0
}
*/

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_generator() {
        let input = input_generator(INPUT);
        //println!("{:#?}", input);
        input.print();
        //assert!(false);
    }

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(6032, part1(&input));
    }

    #[test]
    fn test_teleport_up() {
        let input = input_generator(INPUT);
        let pos = (1, 5);
        assert_eq!(Some((1, 8)), input.teleport(&pos, &Direction::Up));
    }

    #[test]
    fn test_teleport_down() {
        let input = input_generator(INPUT);
        let pos = (16, 12);
        assert_eq!(Some((16, 9)), input.teleport(&pos, &Direction::Down));
    }

    #[test]
    fn test_teleport_left() {
        let input = input_generator(INPUT);
        println!("\n----------------------\nSTART LEFT");
        let pos = (9, 4);
        assert_eq!(Some((12, 4)), input.teleport(&pos, &Direction::Left));
    }
    #[test]
    fn test_teleport_right() {
        let input = input_generator(INPUT);
        println!("\n----------------------\nSTART LEFT");
        let pos = (12, 4);
        assert_eq!(Some((9, 4)), input.teleport(&pos, &Direction::Right));
    }
}
