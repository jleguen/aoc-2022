//use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::string::ParseError;
use toodee::{Coordinate, TooDee, TooDeeOps, TooDeeOpsMut};

// ---------------------------------------------------------------------------
#[derive(Debug)]
pub struct Forest {
    tree_height: TooDee<u32>,
    visible: TooDee<bool>,
}

impl Forest {
    fn from_vec(input: &Vec<Vec<u32>>) -> Self {
        let height = TooDee::from_vec(
            input[0].len(),
            input.len(),
            input.into_iter().map(|v| v.clone()).flatten().collect(),
        );
        let vis = TooDee::new(height.num_cols(), height.num_rows());
        let mut forest = Forest {
            tree_height: height,
            visible: vis,
        };
        forest
    }

    fn count_visible(&self) -> usize {
        self.visible
            .rows()
            .map(|r| r.iter().filter(|&v| *v == true).count())
            .sum()
    }

    fn print_visible(&self) {
        for row in self.visible.rows() {
            for elem in row.iter() {
                match elem {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!("");
        }
        println!("");
    }

    fn viewing_dist(&self, coord: Coordinate) -> usize {
        let cols = self.tree_height.num_cols();
        let rows = self.tree_height.num_rows();
        let val = self.tree_height[coord];

        let mut max = val;
        let mut min = 0;
        let mut num: [usize; 4] = [0, 0, 0, 0];
        // up
        println!("\nViewing from {} {}", coord.0, coord.1);
        for row in (0..coord.1).rev() {
            let cur = self.tree_height[(coord.0, row)];
            print!("  up    {} {} : {} ({})", coord.0, row, cur, max);
            if cur < max {
                num[0] += 1;
                println!(" num {}", num[0]);
            } else {
                num[0] += 1;
                println!(" STOP num {}", num[0]);
                break; // Stop at first big tree
            }
        }
        // down
        max = val;
        min = 0;
        for row in (coord.1 + 1)..rows {
            let cur = self.tree_height[(coord.0, row)];
            print!("  down  {} {} : {} ({})", coord.0, row, cur, max);
            if cur < max {
                num[2] += 1;
                println!(" num {}", num[2]);
            } else {
                num[2] += 1;
                println!(" STOP num {}", num[2]);
                break; // Stop at first big tree
            }
        }
        // left
        max = val;
        min = 0;
        for col in (0..coord.0).rev() {
            let cur = self.tree_height[(col, coord.1)];
            print!("  left  {} {} : {} ({})", col, coord.1, cur, max);
            if cur < max {
                num[1] += 1;
                println!(" num {}", num[1]);
            } else {
                num[1] += 1;
                println!(" STOP num {}", num[1]);
                break; // Stop at first big tree
            }
        }
        // right
        max = val;
        min = 0;
        for col in (coord.0 + 1)..cols {
            let cur = self.tree_height[(col, coord.1)];
            print!("  right {} {} : {} ({})", col, coord.1, cur, max);
            if cur < max {
                num[3] += 1;
                println!(" num {}", num[3]);
            } else {
                num[3] += 1;
                println!(" STOP num {}", num[3]);
                break; // Stop at first big tree
            }
        }
        println!("{} {}: {:?}", coord.0, coord.1, num);
        let res = num.iter().product();
        res
    }

    fn build_visible(&mut self) {
        let cols = self.tree_height.num_cols();
        let rows = self.tree_height.num_rows();

        // Corners
        self.visible[(0, 0)] = true;
        self.visible[(0, rows - 1)] = true;
        self.visible[(cols - 1, 0)] = true;
        self.visible[(cols - 1, rows - 1)] = true;

        // From the left and right
        for row in 1..(rows - 1) {
            let mut high_left = self.tree_height[(0, row)];
            self.visible[(0, row)] = true;
            let mut high_right = self.tree_height[(cols - 1, row)];
            self.visible[(cols - 1, row)] = true;

            for col in 1..(cols - 1) {
                let left = self.tree_height[(col, row)];
                if left > high_left {
                    self.visible[(col, row)] = true;
                    high_left = left;
                }
                let right = self.tree_height[(cols - col - 1, row)];
                if right > high_right {
                    self.visible[(cols - col - 1, row)] = true;
                    high_right = right;
                }
            }
            //self.print_visible();
        }

        // From top and bottom
        for col in 1..(cols - 1) {
            let mut high_top = self.tree_height[(col, 0)];
            self.visible[(col, 0)] = true;
            let mut high_bot = self.tree_height[(col, rows - 1)];
            self.visible[(col, rows - 1)] = true;

            for row in 1..(rows - 1) {
                let top = self.tree_height[(col, row)];
                if top > high_top {
                    self.visible[(col, row)] = true;
                    high_top = top;
                }
                let bot = self.tree_height[(col, rows - row - 1)];
                if bot > high_bot {
                    self.visible[(col, rows - row - 1)] = true;
                    high_bot = bot;
                }
            }
            //self.print_visible();
        }

        //println!("{:?}", self);
    }
}
// ---------------------------------------------------------------------------
#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Forest {
    let mut res: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let ch = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        res.push(ch);
    }
    let mut forest = Forest::from_vec(&res);
    forest.build_visible();
    forest
}

// ---------------------------------------------------------------------------
#[aoc(day8, part1)]
pub fn part1_manual(input: &Forest) -> usize {
    input.count_visible()
}

#[aoc(day8, part2)]
pub fn part2_manual(input: &Forest) -> usize {
    let mut max = 0;
    for row in 0..input.tree_height.num_rows() {
        for col in 0..input.tree_height.num_cols() {
            let v = input.viewing_dist((col, row));
            if v > max {
                max = v;
            }
        }
    }
    max
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_generator() {
        let mut forest = input_generator(INPUT);
    }

    #[test]
    fn test_part1_manual() {
        let dirs = input_generator(INPUT);
        assert_eq!(21, part1_manual(&dirs));
    }
    #[test]
    fn test_viewing() {
        let forest = input_generator(INPUT);
        assert_eq!(4, forest.viewing_dist((2, 1)));
        assert_eq!(8, forest.viewing_dist((2, 3)));
    }
    #[test]
    fn test_part2_manual() {
        let dirs = input_generator(INPUT);
        assert_eq!(8, part2_manual(&dirs));
    }
}
