use parse_display::{Display, FromStr};
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
pub struct RuckSack {
    one: HashSet<char>,
    two: HashSet<char>,
}

impl std::str::FromStr for RuckSack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("FromStr '{}'", s);
        let mut one = HashSet::new();
        let mut two = HashSet::new();
        let size = s.len() / 2;
        for i in 0..(size as usize) {
            one.insert(s.chars().nth(i).unwrap());
            two.insert(s.chars().nth(size + i).unwrap());
        }
        let res = RuckSack { one, two };
        //println!("{:?} {}", &res, &res.part1());
        Ok(res)
    }
}

impl RuckSack {
    fn duplicates(&self) -> &char {
        let i: HashSet<&char> = self.one.intersection(&self.two).collect();
        assert_eq!(1, i.len());
        i.iter().next().unwrap()
    }

    fn value(c: &char) -> u8 {
        match c {
            'a'..='z' => (*c as u8) - ('a' as u8) + 1,
            'A'..='Z' => (*c as u8) - ('A' as u8) + 27,
            i => panic!("Unknown {}", i),
        }
    }

    fn part1(&self) -> u64 {
        Self::value(self.duplicates()) as u64
    }

    fn part2(&self) -> HashSet<char> {
        let mut h = self.one.clone();
        h.extend(&self.two);
        h
    }
}

fn badge(group: &Vec<&RuckSack>) -> u64 {
    assert_eq!(3, group.len());
    let mut res: HashSet<char> = group[0].part2();

    println!("{:?}", res);
    for r in group {
        println!("New {:?}", &r.part2());
        res = res.intersection(&r.part2()).map(|&x| x).collect();
        println!("RES {:?}", res);
    }

    assert_eq!(1, res.len());

    RuckSack::value(&res.iter().next().unwrap().clone()).into()
}

// ---------------------------------------------------------------------------
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<RuckSack> {
    let mut res: Vec<RuckSack> = Vec::new();
    for line in input.lines() {
        res.push(RuckSack::from_str(&line).unwrap());
    }
    res
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day3, part1)]
pub fn part1(input: &Vec<RuckSack>) -> u64 {
    input.iter().fold(0, |sum, v| sum + v.part1())
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<RuckSack>) -> u64 {
    let mut res = 0;

    // Iterate over groups of 3
    for i in 0..input.len() / 3 {
        let group: Vec<_> = input[i * 3..=i * 3 + 2].iter().collect();
        res += badge(&group);
    }

    res
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_generator() {
        let sacks = input_generator(INPUT);
        assert_eq!(sacks.len(), 6);
    }

    #[test]
    fn test_part1() {
        let sacks = input_generator(INPUT);
        assert_eq!(157, part1(&sacks))
    }

    #[test]
    fn test_part2() {
        let sacks = input_generator(INPUT);
        assert_eq!(70, part2(&sacks))
    }
}
