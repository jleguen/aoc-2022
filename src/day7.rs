use parse_display::{Display, FromStr};
use std::collections::HashMap;
//use std::string::ParseError;

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Using manual hand-waving
// ---------------------------------------------------------------------------
#[derive(Display, FromStr, Debug, PartialEq)]
enum Command {
    #[display("$ cd /")]
    CdTop,
    #[display("$ cd {0}")]
    CdIn(String),
    #[display("$ cd ..")]
    CdOut,
    #[display("$ ls")]
    List,
}
// ---------------------------------------------------------------------------
fn full_name(path: &Vec<String>) -> String {
    path.join("/")
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, usize> {
    let mut dirs: HashMap<String, usize> = HashMap::new();

    // Current exploration path, to be able to go up
    let mut path: Vec<String> = Vec::new();

    for line in input.lines() {
        // Command or listing?
        if line.starts_with("$ ") {
            match line.parse().unwrap() {
                Command::CdTop => {
                    path.clear();
                    let name = String::from("/");
                    path.push(name.clone());
                    let _top = dirs.entry(full_name(&path)).or_insert_with(|| 0);
                }
                Command::CdIn(name) => {
                    path.push(name.clone());
                    let _cur = dirs.entry(full_name(&path)).or_insert_with(|| 0);
                }
                Command::CdOut => {
                    path.pop();
                }
                Command::List => {
                    //println!("$ ls");
                }
            }
            //println!("{:?}", path);
        } else {
            if line.starts_with("dir") {
            } else {
                let size: usize = line.split_once(' ').unwrap().0.parse().unwrap();
                for dir in 0..path.len() {
                    dirs.entry(full_name(&path[..=dir].to_vec()))
                        .and_modify(|val| *val += size);
                }
            }
        }
    }

    println!("{:#?}", dirs);
    dirs
}

// ---------------------------------------------------------------------------
#[aoc(day7, part1, manual)]
pub fn part1_manual(input: &HashMap<String, usize>) -> usize {
    let dirs: Vec<(&String, &usize)> = input.iter().filter(|&(_, v)| *v <= 100000).collect();
    println!("{:?}", dirs);

    dirs.iter().map(|(_, &v)| v).fold(0, |sum, val| sum + val)
}

const TOTAL: usize = 70_000_000;
const MIN_FREE: usize = 30_000_000;

#[aoc(day7, part2, manual)]
pub fn part2_manual(input: &HashMap<String, usize>) -> usize {
    let req = MIN_FREE - (TOTAL - input["/"]);
    println!(
        "Used {} Free {} Req {}",
        input["/"],
        TOTAL - input["/"],
        MIN_FREE - (TOTAL - input["/"])
    );
    let mut dirs: Vec<usize> = input
        .iter()
        .filter(|&(_, v)| *v >= req)
        .map(|(_, &v)| v)
        .collect();
    println!("{:?}", dirs);

    dirs.sort();
    dirs[0]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_generator() {
        let dirs = input_generator(INPUT);
        assert_eq!(584, dirs["//a/e"]);
        assert_eq!(94853, dirs["//a"]);
        assert_eq!(24933642, dirs["//d"]);
        assert_eq!(48381165, dirs["/"]);
    }

    #[test]
    fn test_command() {
        assert_eq!(Command::CdTop, "$ cd /".parse().unwrap());
        assert_eq!(Command::CdOut, "$ cd ..".parse().unwrap());
        assert_eq!(
            Command::CdIn(String::from("test")),
            "$ cd test".parse().unwrap()
        );
    }
    #[test]
    fn test_part1_manual() {
        let dirs = input_generator(INPUT);
        assert_eq!(95437, part1_manual(&dirs));
    }
    #[test]
    fn test_part2_manual() {
        let dirs = input_generator(INPUT);
        assert_eq!(24933642, part2_manual(&dirs));
    }
}
