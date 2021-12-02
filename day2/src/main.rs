use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

enum Movement {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let moves = parse(&input)?;

    let (dist, depth) = part1(&moves);
    io::stdout().write_fmt(format_args!("Part 1: {}\n", dist * depth))?;

    let (dist, depth) = part2(&moves);
    io::stdout().write_fmt(format_args!("Part 2: {}\n", dist * depth))?;

    Ok(())
}

fn parse<'a>(input: &str) -> Result<Vec<Movement>> {
    let mut moves = Vec::<Movement>::new();

    for line in input.lines() {
        match line.split_once(' ') {
            Some((op, v)) => {
                let value = v.parse()?;
                moves.push(match op {
                    "forward" => Movement::Forward(value),
                    "down" => Movement::Down(value),
                    "up" => Movement::Up(value),
                    _ => return Err("invalid direction".into()),
                })
            }
            None => return Err("invalid input".into()),
        }
    }
    Ok(moves)
}

fn part1(moves: &Vec<Movement>) -> (usize, usize) {
    let mut dist = 0_usize;
    let mut depth = 0_usize;

    for v in moves {
        match v {
            Movement::Forward(d) => dist += d,
            Movement::Down(d) => depth += d,
            Movement::Up(d) => depth -= d,
        }
    }
    (dist, depth)
}

fn part2(moves: &Vec<Movement>) -> (usize, usize) {
    let mut dist = 0_usize;
    let mut depth = 0_usize;
    let mut aim = 0_usize;

    for v in moves {
        match v {
            Movement::Forward(d) => {
                dist += d;
                depth += aim * d;
            }
            Movement::Down(d) => aim += d,
            Movement::Up(d) => aim -= d,
        }
    }
    (dist, depth)
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn simple() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let moves = parse(input).unwrap();

        let (dist, depth) = part1(&moves);
        assert_eq!(15, dist);
        assert_eq!(10, depth);

        let (dist, depth) = part2(&moves);
        assert_eq!(15, dist);
        assert_eq!(60, depth);
    }
}
