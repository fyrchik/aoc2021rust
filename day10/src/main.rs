use rayon::prelude::*;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    io::stdout().write_fmt(format_args!("Part 1: {}\n", part1(&input)))?;
    io::stdout().write_fmt(format_args!("Part 2: {}\n", part2(&input)))?;

    Ok(())
}

pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    let mut stack = Vec::<u8>::new();

    'outer: for line in input.lines() {
        stack.truncate(0);
        for &c in line.as_bytes() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' => {
                    if !matches!(stack.pop(), Some(b'(')) {
                        sum += 3;
                        continue 'outer;
                    }
                }
                b']' => {
                    if !matches!(stack.pop(), Some(b'[')) {
                        sum += 57;
                        continue 'outer;
                    }
                }
                b'}' => {
                    if !matches!(stack.pop(), Some(b'{')) {
                        sum += 1197;
                        continue 'outer;
                    }
                }
                b'>' => {
                    if !matches!(stack.pop(), Some(b'<')) {
                        sum += 25137;
                        continue 'outer;
                    }
                }
                _ => unreachable!("unexpected input"),
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .par_lines()
        .filter_map(|line| {
            let mut stack = Vec::<u8>::new();

            for &c in line.as_bytes() {
                match c {
                    b'(' | b'[' | b'{' | b'<' => stack.push(c),
                    b')' => {
                        if !matches!(stack.pop(), Some(b'(')) {
                            return None;
                        }
                    }
                    b']' => {
                        if !matches!(stack.pop(), Some(b'[')) {
                            return None;
                        }
                    }
                    b'}' => {
                        if !matches!(stack.pop(), Some(b'{')) {
                            return None;
                        }
                    }
                    b'>' => {
                        if !matches!(stack.pop(), Some(b'<')) {
                            return None;
                        }
                    }
                    _ => unreachable!("unexpected input"),
                }
            }
            Some(stack.iter().rev().fold(0_usize, |acc, &c| {
                acc * 5
                    + match c {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => unreachable!("BUG: unexpected item on stack"),
                    }
            }))
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(26397, part1(&input));
        assert_eq!(288957, part2(&input));
    }
}
