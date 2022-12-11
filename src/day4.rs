//use parse_display::{Display, FromStr};
//use std::collections::HashSet;
//use std::ops::RangeInclusive;
//use std::string::ParseError;

type Range = (u32, u32);

trait AOC {
    fn build(s: &str) -> Range;
    fn contains(&self, other: &Range) -> bool;
    fn overlaps(&self, other: &Range) -> bool;
}

impl AOC for Range {
    fn build(s: &str) -> Self {
        if let Some((a, b)) = s.split_once('-') {
            return (a.parse().unwrap(), b.parse().unwrap());
        }
        panic!()
    }

    fn contains(&self, other: &Range) -> bool {
        &self.0 <= &other.0 && &self.1 >= &other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.0..=other.1).contains(&self.0)
            || (other.0..=other.1).contains(&self.1)
            || (self.0..=self.1).contains(&other.0)
            || (self.0..=self.1).contains(&other.1)
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Range, Range)> {
    let mut res = Vec::new();
    for line in input.lines() {
        if let Some((one, two)) = line.split_once(',') {
            res.push((Range::build(&one), Range::build(&two)));
        }
    }
    res
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day4, part1)]
pub fn part1(input: &Vec<(Range, Range)>) -> u64 {
    let mut res = 0;
    for pair in input {
        if pair.0.contains(&pair.1) || pair.1.contains(&pair.0) {
            res += 1;
        }
    }
    res
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<(Range, Range)>) -> u64 {
    let mut res = 0;
    for pair in input {
        if pair.0.overlaps(&pair.1) {
            res += 1;
        }
    }
    res
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_generator() {
        let pairs = input_generator(INPUT);
        assert_eq!(pairs.len(), 6);
    }

    #[test]
    fn test_part1() {
        let pairs = input_generator(INPUT);
        assert_eq!(2, part1(&pairs))
    }

    #[test]
    fn test_part2() {
        let pairs = input_generator(INPUT);
        assert_eq!(4, part2(&pairs))
    }
}
