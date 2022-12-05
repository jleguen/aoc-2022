use ascii;
use parse_display::{Display, FromStr};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::string::ParseError;

// number, from, to
// Input is "move 1 from 2 to 1"
#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("move {num} from {from} to {to}")]
pub struct Move {
    num: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn mov(&mut self, mov: &Move) {
        println!("{}", mov);
        for _ in 0..mov.num {
            let e = self.stacks[mov.from - 1].pop().unwrap();
            self.stacks[mov.to - 1].push(e);
        }
    }

    fn moves(&mut self, moves: &Vec<Move>) {
        println!("Stacks::moves");
        for m in moves {
            self.mov(&m);
        }
    }

    fn moves2(&mut self, moves: &Vec<Move>) {
        println!("Stacks::move2");
        for m in moves {
            let i = self.stacks[m.from - 1].len() - m.num;
            let e = self.stacks[m.from - 1].split_off(i);
            self.stacks[m.to - 1].extend(e);
        }
    }

    fn top(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }

    fn part1(&mut self, m: &Vec<Move>) -> String {
        println!("Stacks::part1");
        self.moves(m);
        self.top()
    }

    fn part2(&mut self, m: &Vec<Move>) -> String {
        println!("Stacks::part2");
        self.moves2(m);
        self.top()
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Stacks, Vec<Move>) {
    // Vec of stacks
    let mut res = Vec::new();
    // Separate input into stacks and moves
    if let Some((stacks, mov)) = input.split_once("\n\n") {
        // Number of stacks is the last number of the last line
        let mut stacks_iter = stacks.lines().rev();
        let num: usize = stacks_iter
            .next()
            .unwrap()
            .trim_end()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        println!("{:?}", num);
        for _ in 0..num {
            res.push(Vec::new());
        }

        // Parse all crates
        for line in stacks_iter {
            // Find crates in the form [A]
            let crates: Vec<(usize, char)> = line
                .char_indices()
                .filter(|(_, c)| c.is_ascii_alphabetic())
                .map(|(i, c)| ((i - 1) / 4, c))
                .collect();
            println!("{:?}", crates);
            for (i, c) in crates {
                res[i].push(c);
            }
        }

        // Parse all moves
        let moves: Vec<Move> = mov.lines().map(|s| s.parse().unwrap()).collect();

        return (Stacks { stacks: res }, moves);
    } else {
        panic!();
    }
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day5, part1)]
pub fn part1(input: &(Stacks, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();
    let moves = &input.1;
    stacks.part1(&moves)
}

#[aoc(day5, part2)]
pub fn part2(input: &(Stacks, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();
    let moves = &input.1;
    stacks.part2(&moves)
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_generator() {
        let (stacks, moves) = input_generator(INPUT);
        assert_eq!(4, moves.len());
    }

    #[test]
    fn test_part1() {
        let pairs = input_generator(INPUT);
        assert_eq!(String::from("CMZ"), part1(&pairs))
    }

    #[test]
    fn test_part2() {
        let pairs = input_generator(INPUT);
        assert_eq!(String::from("MCD"), part2(&pairs))
    }
}
