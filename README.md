# Advent of Code 2021 in Rust

This repo contains solutions to Advent of Code 2021 puzzles in Rust.

My goal is to become more familliar with the language and learn new features
along the way.

So I have imposed some restrictions on my solutions:
1. Use only stable rust.
2. Use no external dependencies with 2 exceptions:
	- `rayon` crate which provides an easy way to parallelize code. Like really, it is such a pleasure to use compared to `std::thread`.
	- `criterion` crate which is the right way to do benchmarks
to parallelize my code. 
3. Try to use as much of stdlib as I can. This is just to learn rust. Because of this my solutions are not always as simple as they could be.
4. Be able to handle inputs from https://the-tk.com/project/aoc2021-bigboys.html .
This is not always possible without dependencies (mostly for working with big integers). I started to do this from day 7.

## Structure

Solutions to day N are in `dayN` directory. Each `src/main.rs` contains:
-  `main` function to output solutions for any inputs and `part1`
- `parse` to parse input when needed
- `part1` and `part2` which provide result for respective parts of an exercise. Sometimes they provide more fine-grained result to facilitate better testing

To see the result for input from file `input`, run
```
cat input | cargo run
```

Sometimes (e.g. for day 8 when there are many simple puzzles) I don't follow this scheme because it is better to parallelize over the whole input.