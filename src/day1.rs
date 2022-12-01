
/// Split up input into Vec<Elf> each Elf is a Vec<u32>
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut tmp: Vec<u32> = Vec::new();
    for line in input.lines() {
        if line.len() > 0 {
            tmp.push(line.parse::<u32>().unwrap())
        } else {
            elves.push(tmp);
            tmp = Vec::new();
        }
    }
    elves.push(tmp);
    elves
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day1, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut food : Vec<u32> = input.iter().map(|x| x.iter().fold(0, |sum, y| sum + y)).collect();
    food.sort();
    food.reverse();
    food[0]
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut food : Vec<u32> = input.iter().map(|x| x.iter().fold(0, |sum, y| sum + y)).collect();
    food.sort();
    food.reverse();
    food[0]+food[1] + food[2]
}


// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 5);
    }

    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(24000, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(45000, part2(&i));
    }
}
