# Advent of Code 2017

Solutions to [Advent of Code 2017](https://adventofcode.com/2017) challenges in Rust.

## Project Structure

This repository uses a Cargo workspace with each day's solution in its own subdirectory:

- `day_01/` - Day 1 solution
- `day_02/` - Day 2 solution
- `day_03/` - Day 3 solution

## Building

Build all solutions:
```bash
cargo build --release
```

Build a specific day:
```bash
cargo build --release -p advent_2017_day_01
```

## Running

Each day's solution is a binary that can be run from the workspace root or from within the day's directory.

### Day 01 and Day 02
These days read from their `input.txt` file:
```bash
cd day_01 && cargo run --release
cd day_02 && cargo run --release
```

### Day 03
This day takes a command-line argument:
```bash
cd day_03 && cargo run --release -- 347991
```

## License

MIT License