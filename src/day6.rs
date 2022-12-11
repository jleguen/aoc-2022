//use parse_display::{Display, FromStr};
use std::collections::HashMap;
//use std::string::ParseError;

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
fn find_distinct(input: &String, num: usize) -> usize {
    // Current number of consecutive new chars
    let mut cur: usize = 0;
    // HashMap<char, index>
    let mut map: HashMap<char, usize> = HashMap::new();

    /* mjqjpqmgbljsphdztnvjfqwrcgsmlb
     *
     * CHAR     NUM     INDEX   AGE
     * m        1       1       -
     * mj       2       2       -
     * mjq      3       3       -
     * mjqj     2       4       2
     * --qjp    3       5       -
     * --qjpq   3       6       3
     * ---jpqm  4       7       -
     */
    for (i, c) in input.chars().enumerate() {
        let index = i + 1;
        print!("{}: {}", index, c);
        let prev = map.insert(c, index);
        match prev {
            None => {
                cur += 1;
            }
            Some(p) => {
                let age = index - p;
                print!("    prev {} (age {}) cur {}", p, age, cur);
                // If instance older than NUM -> count it as new
                // Else, remove instance age from num and continue

                print!("    cur++");
                cur += 1;
                if age <= cur {
                    print!("    cur -= {}", cur - age);
                    assert!(cur >= cur - age);
                    cur -= cur - age;
                }
            }
        }
        println!("");

        if num == cur {
            println!("FOUND index {}\n", index);
            return index;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    String::from(input)
}

#[aoc(day6, part1)]
pub fn part1(input: &String) -> usize {
    find_distinct(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &String) -> usize {
    find_distinct(input, 14)
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];
    const INPUT2: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];
    #[test]
    fn test_generator() {}

    #[test]
    fn test_part1() {
        for (s, res) in INPUT {
            let input = input_generator(&s);
            assert_eq!(res, part1(&input));
        }
    }
    #[test]
    fn test_part2() {
        for (s, res) in INPUT2 {
            let input = input_generator(&s);
            assert_eq!(res, part2(&input));
        }
    }
}
