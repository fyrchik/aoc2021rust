use std::cmp::min;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut positions = parse(&input)?;

    let number = part1(&mut positions);
    io::stdout().write_fmt(format_args!("Part 1: {}\n", number))?;

    let number = part2(&positions);
    io::stdout().write_fmt(format_args!("Part 2: {}\n", number))?;

    Ok(())
}

fn parse(input: &str) -> Result<Vec<usize>> {
    let mut positions = Vec::new();
    for s in input.trim().split(',') {
        positions.push(s.parse()?)
    }
    Ok(positions)
}

fn part1(positions: &mut [usize]) -> usize {
    positions.sort_unstable();

    let median = positions[positions.len() / 2];
    let mut sum = 0_usize;

    for p in positions.iter() {
        sum += if p < &median { median - p } else { p - median };
    }
    sum
}

#[inline]
fn sum1n(n: usize) -> usize {
    n * (n + 1) / 2
}

fn part2(positions: &[usize]) -> usize {
    let total: usize = positions.iter().sum();
    let mean = total / positions.len();
    let mut sum1 = 0_usize;
    let mut sum2 = 0_usize;

    for p in positions.iter() {
        if *p <= mean {
            sum1 += sum1n(mean - p);
            sum2 += sum1n(mean + 1 - p);
        } else {
            sum1 += sum1n(p - mean);
            sum2 += sum1n(p - mean - 1);
        }
    }
    min(sum1, sum2)
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn example() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let mut positions = parse(input).expect("can't parse input");

        let number = part1(&mut positions);
        assert_eq!(37, number);

        let number = part2(&positions);
        assert_eq!(168, number);
    }
}
