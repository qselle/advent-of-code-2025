[![Rust](https://github.com/qselle/advent-of-code-2025/actions/workflows/rust.yml/badge.svg)](https://github.com/qselle/advent-of-code-2025/actions/workflows/rust.yml) [![wakatime](https://wakatime.com/badge/github/qselle/advent-of-code-2025.svg)](https://wakatime.com/badge/github/qselle/advent-of-code-2025)

# Advent of code 2025 🎄🎅

1 year without coding Rust... Using https://github.com/gobanos/cargo-aoc 🦀.

Join the adventure: https://adventofcode.com/2025

## Build with `cargo-aoc`

Install [`cargo-aoc`](https://crates.io/crates/cargo-aoc) crate:

```console
advent-of-code-2025 $> cargo install cargo-aoc
```

Run:

```console
advent-of-code-2025 $> 7cf34de30383 :: ~/advent-of-code-2025 ‹main› » cargo-aoc 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
   Compiling aoc-autobuild v0.3.0 (/Users/qselle/advent-of-code-2025/target/aoc/aoc-autobuild)
    Finished `release` profile [optimized] target(s) in 0.88s
     Running `target/release/aoc-autobuild`
AOC 2025
Day 2 - Part 1 : 18700015741
        generator: 7.708µs,
        runner: 18.659ms

Day 2 - Part 2 : 20077272987
        generator: 8µs,
        runner: 60.883375ms
```

## Test

```console
advent-of-code-2025 $> cargo test
   Compiling advent-of-code-2025 v0.1.0 (/Users/qselle/advent-of-code-2025)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.05s
     Running unittests src/lib.rs (target/debug/deps/advent_of_code_2025-ff35487d7b35c543)

running 7 tests
test day1::tests::part1_example ... ok
test day1::tests::part2_example ... ok
test day1::tests::test_leaving_zero_does_not_count ... ok
test day1::tests::part2_example_bis ... ok
test day1::tests::test_arriving_at_zero_from_left_counts ... ok
test day2::tests::part1_example ... ok
test day2::tests::part2_example ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests advent_of_code_2025

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```