# Rustvent of Code 2024

This repo holds my implementation of the [Advent of Code 2024](https://adventofcode.com/2024) in Rust.

## Structure

This repo has the following structure:

```
|-inputs
| |-01.txt              MY challenge input for day 1
| |-01-example.txt      Example input for day 1 (both parts)
| |-01-example-1.txt    Example input for day 1 (only part 1)
| |-02.txt
| |-...
|-src
| |-days
| | |-day00.rs          Template for a day's solution
| | |-dayXX.rs          Solution for day XX
| |-main.rs             Binary to run to execute one or all days
| |-*.rs                Some more utils and traits
|-Cargo.toml            Dependency management
|-README.md             This file
```

## Adding a new solution

1. Clone **src/days/day00.rs** to **src/days/dayXX.rs**
2. Add `pub mod dayXX;` and the case `XX => Some(Box::new(dayXX::Day)),` to **src/days.rs**
3. Implement `Solution::part1` and `Solution::part2` in **src/days/dayXX.rs** (don't forget to update testcases at the end of the file)

## Running

### Arguments

| Argument  | Default | Description                                  |
| :-------- | :-----: | :------------------------------------------- |
| --help    |         | Print command help                           |
| --day     |         | Select day to run (runs all when not set)    |
| --example |  false  | Use example input instead of challenge input |

### Examples

#### Single Day

`cargo run --release -- --day XX`

#### All days

`cargo run --release`

#### All days with examples

`cargo run --release -- --examples`

## Testing

All days have unittests attached for example and **MY** challenge input. These can be run by executing `cargo test`.

Some tests are ignored by default, because they take too long for active development (multiple seconds). You can enable them by running `cargo test -- --include-ignored`.
