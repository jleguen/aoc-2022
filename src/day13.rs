//use parse_display;
//use std::collections::HashMap;
//use std::fmt;
//use std::str::FromStr;
//use std::string::ParseError;
//use std::sync::{Arc, Mutex};
//use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};
//use num_bigint::{u64, Tou64};
//use std::ops::{Rem, Div};
use serde_json::{Number, Value};
use std::cmp::Ordering;
use std::iter::zip;

// returns true if left <= right
// XXX [2,3,4] <= 4
fn compare(left: &Value, right: &Value, _single: bool) -> Option<Ordering> {
    println!("Compare {} and {}", left, right);
    match (left, right) {
        (Value::Number(one), Value::Number(two)) => {
            one.as_u64().unwrap().partial_cmp(&two.as_u64().unwrap())
        }
        (Value::Number(_), Value::Array(_)) => {
            compare(&Value::Array(vec![left.clone()]), right, true)
        }
        (Value::Array(_), Value::Number(_)) => {
            compare(left, &Value::Array(vec![right.clone()]), true)
        }
        (Value::Array(one), Value::Array(two)) => {
            for (left, right) in zip(one, two) {
                let ord = compare(left, right, false);
                println!("      {left} {right}   {:?}", ord);
                if ord != Some(Ordering::Equal) {
                    return ord;
                }
            }
            // Find out which list is shorter
            println!("      Len one {} two {}", one.len(), two.len());
            one.len().partial_cmp(&two.len())
        }
        _ => panic!("Unknown Value"),
    }
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<(Value, Value)> {
    let mut res = Vec::new();
    for pair in input.split("\n\n") {
        let mut lines = pair.lines();
        let one: Value = serde_json::from_str(lines.next().expect("line")).expect("JSON");
        let two: Value = serde_json::from_str(lines.next().expect("line")).expect("JSON");
        res.push((one, two));
    }
    res
}

// ---------------------------------------------------------------------------
#[aoc(day13, part1)]
pub fn part1(input: &Vec<(Value, Value)>) -> usize {
    let mut res = 0;
    for (i, pair) in input.iter().enumerate() {
        println!("\n=========== Pair {} ==========", i + 1);
        if Some(Ordering::Less) == compare(&pair.0, &pair.1, false) {
            res += i + 1;
            println!("+ Comparison true => res = {res}\n");
        } else {
            println!("- Comparison false\n");
        }
    }
    res
}

#[aoc(day13, part2)]
pub fn part2(input: &Vec<(Value, Value)>) -> usize {
    let mut res: Vec<Value> = Vec::new();
    let mk1: Value = serde_json::from_str("[[2]]").unwrap();
    let mk2: Value = serde_json::from_str("[[6]]").unwrap();
    res.push(mk1.clone());
    res.push(mk2.clone());
    for (one, two) in input {
        res.push(one.clone()); // XXX This is ugly
        res.push(two.clone());
    }
    res.sort_by(|a, b| compare(&a, &b, false).unwrap());
    for elem in res.iter() {
        println!("{}", elem);
    }
    (1 + res.iter().position(|e| e == &mk1).unwrap())
        * (1 + res.iter().position(|e| e == &mk2).unwrap())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_generator() {}
    #[test]
    fn test_item_int() {
        assert_eq!(
            Some(Ordering::Equal),
            compare(
                &Value::Number(Number::from(0)),
                &Value::Number(Number::from(0)),
                false
            )
        );
        assert_eq!(
            Some(Ordering::Less),
            compare(
                &Value::Number(Number::from(0)),
                &Value::Number(Number::from(1)),
                false
            )
        );
        assert_eq!(
            Some(Ordering::Greater),
            compare(
                &Value::Number(Number::from(1)),
                &Value::Number(Number::from(0)),
                false
            )
        );
    }
    #[test]
    fn test_item_list() {
        let one = Value::Array(vec![Value::Number(Number::from(0))]);
        let two = Value::Array(vec![Value::Number(Number::from(1))]);
        assert_eq!(Some(Ordering::Less), compare(&one, &two, false));
    }
    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(13, part1(&input));
    }
    #[test]
    fn test_part2() {
        let input = input_generator(INPUT);
        assert_eq!(140, part2(&input));
    }
}
