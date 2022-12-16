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

// ---------------------------------------------------------------------------

#[derive(FromStr, Display, Debug, PartialEq)]
#[display("x={0}, y={1}")]
pub struct Coordinate(i64, i64);

impl Coordinate {
    fn dist(&self, other: &Coordinate) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(FromStr, Display, Debug)]
#[display("Sensor at {position}: closest beacon is at {beacon}")]
pub struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
}

enum Position {
    Covered,
    Sensor,
    Beacon,
    Uncovered,
}

impl Sensor {
    // Manhattan distance between sensor and beacon
    fn coverage(&self) -> i64 {
        self.position.dist(&self.beacon)
    }

    // True if both coverages intersect
    fn intersect(&self, other: &Sensor) -> bool {
        let dist = self.position.dist(&other.position);
        self.coverage() + other.coverage() > dist
    }

    // Points one unit outside of the coverage
    fn limits(&self) -> Vec<Coordinate> {
        let cov = self.coverage();
        let mut res: Vec<Coordinate> = Vec::new();
        for i in 0..=cov+1 {
            res.push(Coordinate(self.position.0 + i, self.position.1 + (cov+1 - i)));
            res.push(Coordinate(self.position.0 + i, self.position.1 - (cov+1) + i));
            res.push(Coordinate(self.position.0 - i, self.position.1 + (cov+1 - i)));
            res.push(Coordinate(self.position.0 - i, self.position.1 - (cov+1) + i));
        }
        //println!("{:?}", res);
        res
    }
}

pub struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn display(&self) {
        println!("========= Map =========");
        for sensor in self.sensors.iter() {
            println!("{} Coverage {}", sensor, sensor.coverage());
        }
        println!("\nArea {:?}", self.area());
        println!("=======================");
    }

    fn area(&self) -> (Coordinate, Coordinate) {
        let mut topleft = Coordinate(i64::MAX, i64::MAX);
        let mut botright = Coordinate(i64::MIN, i64::MIN);
        for sensor in self.sensors.iter() {
            if sensor.position.0 < topleft.0 {
                topleft.0 = sensor.position.0;
            }
            if sensor.position.0 > botright.0 {
                botright.0 = sensor.position.0;
            }
            if sensor.position.1 < topleft.1 {
                topleft.1 = sensor.position.1;
            }
            if sensor.position.1 > botright.1 {
                botright.1 = sensor.position.1;
            }

            if sensor.beacon.0 < topleft.0 {
                topleft.0 = sensor.beacon.0;
            }
            if sensor.beacon.0 > botright.0 {
                botright.0 = sensor.beacon.0;
            }
            if sensor.beacon.1 < topleft.1 {
                topleft.1 = sensor.beacon.1;
            }
            if sensor.beacon.1 > botright.1 {
                botright.1 = sensor.beacon.1;
            }
        }
        (topleft, botright)
    }

    fn is_covered_by_sensor(point: &Coordinate, sensor: &Sensor) -> bool {
        let dist = point.dist(&sensor.position);
        let cov = sensor.coverage();
        //println!("  Sensor {:?} Cov {cov} | dist {dist}", sensor.position);
        if point == &sensor.beacon {
            //print!("B");
            //return false;
        }
        if dist <= cov {
            return true;
        }
        return false;
    }

    fn is_covered(&self, point: &Coordinate) -> bool {
        //println!("is_covered {:?}", point);
        for sensor in self.sensors.iter() {
            if Self::is_covered_by_sensor(point, &sensor) {
                //println!("Covered by sensor {sensor}");
                return true;
            }
        }
        false
    }

    // Coverage inside an area (limits included)
    fn coverage_area(&self, left: i64, right: i64, line: i64) -> usize {
        let mut num = 0;
        'line_loop: for x in left..=right {
            let p = Coordinate(x, line);
            //println!("Point {:?}", p);
            if self.is_covered(&p) {
                num += 1;
            } else {
                println!("{x} {line} uncovered");
            }
        }
        num
    }

    // Number of positions on line covered by current sensors and beacons
    fn coverage(&self, line: i64) -> usize {
        let mut num = 0;
        let (topleft, botright) = self.area();
        // Search in the area
        num += self.coverage_area(topleft.0, botright.0, line);
        println!("{num} covered inside the area");
        // Search outside until we lose coverage
        // left
        println!("Looking to the left...");
        let mut p = Coordinate(topleft.0 - 1, line);
        loop {
            if self.is_covered(&p) {
                num += 1;
                p.0 -= 1;
            } else {
                break;
            }
        }
        // right
        println!("Looking to the right...");
        p.0 = botright.0 + 1;
        loop {
            if self.is_covered(&p) {
                num += 1;
                p.0 += 1;
            } else {
                break;
            }
        }
        num
    }

    fn inside(topleft: &Coordinate, botright: &Coordinate, point: &Coordinate) -> bool {
        point.0 >= topleft.0
            && point.0 <= botright.0
            && point.1 >= topleft.1
            && point.1 <= botright.1
    }

    fn find_uncovered(&self, topleft: &Coordinate, botright: &Coordinate) -> Coordinate {
        for sensor in self.sensors.iter() {
            //println!("{} dist {}", sensor, sensor.coverage());
            let points = sensor.limits();
            for point in points {
                // Inside area?
                if !Self::inside(topleft, botright, &point) {
                    continue;
                }
                //println!("  {:?}", point);
                if !self.is_covered(&point) {
                    //println!("Point {:?} is not covered", point);
                    return point;
                }
            }
        }
        unreachable!();
    }
}

// ---------------------------------------------------------------------------
#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Map {
    Map {
        sensors: input.lines().map(|l| l.parse().unwrap()).collect(),
    }
}

// ---------------------------------------------------------------------------
#[aoc(day15, part1)]
pub fn part1(input: &Map) -> usize {
    input.display();
    input.coverage(2000000)
}

#[aoc(day15, part2)]
pub fn part2(input: &Map) -> i64 {
    let p = input.find_uncovered(&Coordinate(0, 0), &Coordinate(4000000, 4000000));
    println!("{:?}", p);
    p.0 * 4000000 + p.1
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_generator() {
        let input = input_generator(INPUT);
        assert_eq!(14, input.sensors.len());
        assert_eq!(Coordinate(2, 18), input.sensors[0].position);
        assert_eq!(7, input.sensors[0].coverage());
        assert_eq!(Coordinate(-2, 0), input.area().0);
        assert_eq!(Coordinate(25, 22), input.area().1);
    }
    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        input.display();
        //assert_eq!(25, input.coverage(9));
        //assert_eq!(26, input.coverage(10));
    }
    #[test]
    fn test_part2() {
        let input = input_generator(INPUT);
        let p = input.find_uncovered(&Coordinate(0, 0), &Coordinate(20, 20));
        assert_eq!(Coordinate(14, 11), p);
    }
}
