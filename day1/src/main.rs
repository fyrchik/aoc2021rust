use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let heights = parse(&input)?;

    let res1 = part1(heights.as_slice());
    io::stdout().write_fmt(format_args!("Part 1: {}\n", res1))?;

    let res2 = part2(heights.as_slice());
    io::stdout().write_fmt(format_args!("Part 2: {}\n", res2))?;

    Ok(())
}

fn parse(input: &str) -> Result<Vec<usize>> {
    let mut heights: Vec<usize> = vec![];

    for line in input.lines() {
        heights.push(line.parse::<usize>()?)
    }
    Ok(heights)
}

fn part1(heights: &[usize]) -> usize {
    let mut prev = usize::MAX;
    let mut count = 0_usize;

    for &h in heights {
        if h > prev {
            count += 1
        }
        prev = h
    }
    count
}

fn part2(heights: &[usize]) -> usize {
    let window_size = 3_usize;
    let mut prev = 0;
    let mut count = 0;

    for h in heights.iter().take(window_size) {
        prev += h;
    }
    for i in window_size..heights.len() {
        let sum = prev + heights[i] - heights[i - window_size];
        if sum > prev {
            count += 1
        }
        prev = sum
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};
    #[test]
    fn simple_sequence() {
        let input: String = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let heights = parse(&input).unwrap();
        assert_eq!(7, part1(heights.as_slice()));
        assert_eq!(5, part2(heights.as_slice()));
    }
}
