use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    io::stdout().write_fmt(format_args!("Part 1: {}\n", part1(&input)))?;
    io::stdout().write_fmt(format_args!("Part 2: {}\n", part2(&input)))?;

    Ok(())
}

#[inline]
fn is_pair(a: u8, b: u8) -> bool {
    b == match a {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => unreachable!("unexpected input"),
    }
}

#[inline]
fn score(c: u8) -> usize {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!("unexpected input"),
    }
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    let mut stack = Vec::<u8>::new();

    'outer: for line in input.lines() {
        stack.truncate(0);
        for &c in line.as_bytes() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' | b']' | b'}' | b'>' => match stack.pop() {
                    Some(p) if is_pair(p, c) => {}
                    _ => {
                        sum += score(c);
                        continue 'outer;
                    }
                },
                _ => unreachable!("unexpected input"),
            }
        }
    }

    sum
}

fn part2(input: &str) -> usize {
    let mut scores = Vec::<usize>::new();
    let mut stack = Vec::<u8>::new();

    'outer: for line in input.lines() {
        stack.truncate(0);
        for &c in line.as_bytes() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' | b']' | b'}' | b'>' => match stack.pop() {
                    Some(p) if is_pair(p, c) => {}
                    _ => continue 'outer,
                },
                _ => unreachable!("unexpected input"),
            }
        }
        scores.push(stack.iter().rev().fold(0_usize, |acc, &c| {
            acc * 5
                + match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => 0,
                }
        }))
    }

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
