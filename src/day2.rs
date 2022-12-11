use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, PartialEq, Copy, Clone)]
pub enum RPS {
    #[from_str(regex = "[AX]")]
    Rock,
    #[from_str(regex = "[BY]")]
    Paper,
    #[from_str(regex = "[CZ]")]
    Scissors,
}

#[derive(Display, FromStr, Debug, PartialEq, Copy, Clone)]
pub enum RoundResult {
    #[from_str(regex = "Z")]
    Win,
    #[from_str(regex = "X")]
    Lose,
    #[from_str(regex = "Y")]
    Draw,
}

impl RoundResult {
    fn value(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

impl RPS {
    fn value(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn eval(&self, other: &RPS) -> RoundResult {
        match self {
            Self::Rock => match other {
                Self::Rock => RoundResult::Draw,
                Self::Paper => RoundResult::Lose,
                Self::Scissors => RoundResult::Win,
            },
            Self::Paper => match other {
                Self::Rock => RoundResult::Win,
                Self::Paper => RoundResult::Draw,
                Self::Scissors => RoundResult::Lose,
            },
            Self::Scissors => match other {
                Self::Rock => RoundResult::Lose,
                Self::Paper => RoundResult::Win,
                Self::Scissors => RoundResult::Draw,
            },
        }
    }

    fn eval2(&self, result: &RoundResult) -> RPS {
        match (self, result) {
            (x, RoundResult::Draw) => *x,
            (Self::Rock, RoundResult::Win) => Self::Paper,
            (Self::Rock, RoundResult::Lose) => Self::Scissors,
            (Self::Paper, RoundResult::Win) => Self::Scissors,
            (Self::Paper, RoundResult::Lose) => Self::Rock,
            (Self::Scissors, RoundResult::Win) => Self::Rock,
            (Self::Scissors, RoundResult::Lose) => Self::Paper,
        }
    }

    fn score(&self, other: &RPS) -> u64 {
        self.value() + self.eval(other).value()
    }
}

pub type Round = (RPS, RPS);
pub type Round2 = (RPS, RoundResult);

// ---------------------------------------------------------------------------
#[aoc_generator(day2, part1)]
pub fn input_generator(input: &str) -> Vec<Round> {
    let mut strat: Vec<Round> = Vec::new();
    for line in input.lines() {
        let round: Vec<RPS> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        strat.push((round[0], round[1]));
    }
    strat
}

#[aoc_generator(day2, part2)]
pub fn input_generator2(input: &str) -> Vec<Round2> {
    let mut strat: Vec<Round2> = Vec::new();
    for line in input.lines() {
        let mut text = line.splitn(2, ' ');
        let round = (
            text.next().unwrap().to_string().parse().unwrap(),
            text.next().unwrap().to_string().parse().unwrap(),
        );
        strat.push(round);
    }
    strat
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day2, part1)]
pub fn part1(input: &Vec<Round>) -> u64 {
    let mut res = 0;
    for round in input {
        res += round.1.score(&round.0)
    }
    res
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Round2>) -> u64 {
    let mut res = 0;
    for round in input {
        let p = round.0.eval2(&round.1);
        res += p.score(&round.0)
    }
    res
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!((RPS::Rock, RPS::Paper), i[0])
    }

    #[test]
    fn test_from_str() {
        assert_eq!(RPS::Rock, "A".parse().unwrap());
        assert_eq!(RPS::Paper, "Y".parse().unwrap());
    }

    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        let score = part1(&i);
        assert_eq!(15, score);
    }
    #[test]
    fn test_part2() {
        let i = input_generator2(INPUT);
        let score = part2(&i);
        assert_eq!(12, score);
    }
}
