use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::string::ParseError;
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};

// ---------------------------------------------------------------------------
#[derive(Display, FromStr, Debug, PartialEq)]
pub enum Dir {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("R")]
    Right,
    #[display("L")]
    Left,
}

#[derive(Display, FromStr, Debug, PartialEq)]
#[display("{dir} {amount}")]
pub struct Move {
    dir: Dir,
    amount: i32,
}

#[derive(Display, Default, Debug, Eq, PartialEq, Copy, Clone, std::hash::Hash)]
#[display("({col}, {row})")]
pub struct Coordinate {
    col: i32,
    row: i32,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Coordinate>,
    visited: Vec<Coordinate>,
}

impl Coordinate {
    // (col, row) or (x, y)
    fn new(col: i32, row: i32) -> Self {
        Coordinate { col, row }
    }
    // euclidian distance rounded down
    fn dist(&self, other: &Coordinate) -> i32 {
        //(self.col - other.col).abs() + (self.row - other.row).abs() // manhattan
        let dx = other.col - self.col;
        let dy = other.row - self.row;
        ((dx * dx + dy * dy) as f64).sqrt() as i32
    }

    // Move closer
    fn one_step_closer(&mut self, other: &Coordinate) -> bool {
        let dx = (other.col - self.col).signum();
        let dy = (other.row - self.row).signum();
        //eprintln!("  step {} {} | {} {}", self, other, dx, dy);
        if self.dist(other) > 1 {
            self.col += dx;
            self.row += dy;
            return true;
        }
        false
    }
}

impl Rope {
    fn new(num: usize) -> Self {
        let mut rope = Rope {
            knots: Vec::with_capacity(num),
            visited: Vec::new(),
        };
        for _ in 0..num {
            rope.knots.push(Coordinate::default());
        }
        rope
    }
    fn one_move(&mut self, dir: &Dir) {
        match dir {
            Dir::Up => {
                self.knots[0].row += 1;
            }
            Dir::Down => {
                self.knots[0].row -= 1;
            }
            Dir::Right => {
                self.knots[0].col += 1;
            }
            Dir::Left => {
                self.knots[0].col -= 1;
            }
        }
    }

    fn apply(&mut self, mov: &Move) {
        //eprintln!("\nApply {}", mov);
        for _ in 0..mov.amount {
            // Move head
            self.one_move(&mov.dir);
            // catch up
            for knot in 1..self.knots.len() {
                let prev = self.knots[knot - 1];
                self.knots[knot].one_step_closer(&prev);
            }
            self.visited.push(*self.knots.last().unwrap());
            //eprintln!("{}", self);
        }
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Move> {
    let mut res = Vec::new();
    for line in input.lines() {
        let mov = line.parse().unwrap();
        //eprintln!("{}", mov);
        res.push(mov);
    }
    res
}

// ---------------------------------------------------------------------------
#[aoc(day9, part1)]
pub fn part1(input: &Vec<Move>) -> usize {
    let mut rope = Rope::new(2);
    for mov in input {
        rope.apply(&mov);
    }
    rope.visited.into_iter().unique().count()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Move>) -> usize {
    let mut rope = Rope::new(10);
    for mov in input {
        rope.apply(&mov);
    }
    rope.visited.into_iter().unique().count()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_coordinate() {
        let a = Coordinate::new(4, 2);
        let b = Coordinate::new(2, 6);
        assert_eq!(4, a.dist(&b));
    }

    #[test]
    fn test_generator() {
        let mut moves = input_generator(INPUT);
        assert_eq!(
            &Move {
                dir: Dir::Right,
                amount: 2
            },
            moves.last().unwrap()
        );
    }

    #[test]
    fn test_rope() {
        let mut rope = Rope::new(2);
        rope.apply(&Move {
            dir: Dir::Right,
            amount: 1,
        });
        assert_eq!(Coordinate::new(1, 0), rope.knots[0]);
        assert_eq!(&Coordinate::new(0, 0), rope.knots.last().unwrap());
        rope.apply(&Move {
            dir: Dir::Up,
            amount: 2,
        });
        assert_eq!(Coordinate::new(1, 2), rope.knots[0]);
        assert_eq!(&Coordinate::new(1, 1), rope.knots.last().unwrap());
    }

    #[test]
    fn test_part1() {
        let mut moves = input_generator(INPUT);
        assert_eq!(13, part1(&moves));
    }
}
