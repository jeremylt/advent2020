# Advent of Code 2020

[![Rust](https://github.com/jeremylt/advent2020/workflows/Rust/badge.svg?branch=main)](https://github.com/jeremylt/advent2020/actions)
[![License](https://img.shields.io/badge/License-BSD%202--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

[Advent of Code 2020](https://adventofcode.com/2020) in Rust. I'm just messing around in some spare time here. Feel free to comment in an issue if you see some possible improvements.

## Installing

To build

    cargo build

To run

    cargo run

To run with optimization

    RUSTFLAGS='-Ctarget-cpu=native -Copt-level=3' cargo run --release

## Try It

You can try this repo live on [repl.it](https://repl.it/@jeremylt/advent2020#README.md).
Note that this may be quite slow to download and install all of the packages.

## Observations

* Trust Rust's compiler. The high level functions give quite good performance.

* File IO is very expensive. String parsing is even more so.

* The cost of File IO and string parsing can be somewhat mitigated by using high level functions to only pass over the data once. Chained high order functions can create accidental performance bombs and can be hard to read, so care is needed. The readability can be mitigated by separating the closures for filter/map functions and naming them well.

* Comparison with chars is faster than with strings, when you can trust the input.

* `splitn` is faster than `split`, and indexing into slices of chars is faster than using `split`, when you know about the input.

## Caveats

* My timing is rather crude right now. I plan to add proper benchmarking if I get some time.
