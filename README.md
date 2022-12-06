# [Advent of Code 2022](https://adventofcode.com/2022/)

View [Kaizen LeaderBoard](https://adventofcode.com/2022/leaderboard/private/view/796831).


## Cargo AoC
This project uses [`cargo-aoc`](https://github.com/gobanos/cargo-aoc) to simplify the interactions with the server.

```
cargo install cargo-aoc
```

### Run
```
cargo aoc
```

## Resolution

### [Day 1](https://adventofcode.com/2022/day/1)
Split entries by empty lines, sum number groups using [`Iterator::fold()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

### [Day 2](https://adventofcode.com/2022/day/2)

### [Day 3](https://adventofcode.com/2022/day/3)
Rust ownership is hard.  
Complicated play with `HashSet` intersections, `&char`. Don't use `union` but `extend` instead to save your sanity.

Nice discovery of `RangeInclusive`
```rust
for i in 'a' ..= 'z' { }
```
### [Day 4](https://adventofcode.com/2022/day/4)
Manually parse ranges and compute inclusion and overlap.

### [Day 5](https://adventofcode.com/2022/day/5)

Move crates around, and return the topmost ones.

```
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

#### Common
Crate `parse_display` facilitates parsing strings into structs:
```rust
// Input is "move 1 from 2 to 1"
#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("move {num} from {from} to {to}")]
pub struct Move {
    num: usize,
    from: usize,
    to: usize,
}
```

This allows the following:
```rust
let mov: Move = "move 1 from 2 to 3".parse().unwrap();
```


#### Part1 - crate by crate
Easily implemented with [`Vec::push()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) 
and [`Vec::pop()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop), to simulate a stack.

#### Part2 - many crates at once
Implemented with [`Vec::split_off()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off) 
and [`Vec::extend()`](https://doc.rust-lang.org/std/iter/trait.Extend.html#tymethod.extend)

### [Day 6](https://adventofcode.com/2022/day/6)
Use a [`HashMap<char, usize>`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) to store char positions 
while iterating (once) over the string. Maintain the count of current consecutive distinct chars and return index if count == required.
