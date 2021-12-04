use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (size, mut params) = parse(&input)?;

    let (epsilon_rate, gamma_rate) = part1(size, &params);
    io::stdout().write_fmt(format_args!("Part 1: {}\n", epsilon_rate * gamma_rate))?;

    let (oxygen_rate, co2_rate) = part2(size, &mut params);
    io::stdout().write_fmt(format_args!("Part 2: {}\n", oxygen_rate * co2_rate))?;

    Ok(())
}

fn parse(input: &str) -> Result<(usize, Vec<usize>)> {
    let mut params = Vec::<usize>::new();
    let mut size = 0_usize;

    for line in input.lines() {
        if size == 0 {
            size = line.len();
        } else if line.len() != size {
            return Err("inconsistent parameter sizes".into());
        }
        let num = usize::from_str_radix(line, 2)?;
        params.push(num);
    }
    Ok((size, params))
}

fn part1(size: usize, params: &[usize]) -> (usize, usize) {
    let mut freq = vec![0; size];

    for v in params {
        for (x, f) in freq.iter_mut().enumerate() {
            *f += (v >> x) & 1;
        }
    }

    let mut gamma_rate = 0;
    for (x, f) in freq.iter().enumerate() {
        if f * 2 >= params.len() {
            gamma_rate |= 1 << x
        }
    }

    (gamma_rate, !gamma_rate & ((1 << size) - 1))
}

fn part2(size: usize, params: &mut [usize]) -> (usize, usize) {
    let oxygen_rate: usize;
    let co2_rate: usize;

    params.sort_unstable();

    let mut current = 0_usize;
    let (mut left, mut right) = (0_usize, params.len());

    for x in (0..size).rev() {
        current |= 1 << x;

        let index: usize = left
            + params[left..right]
                .binary_search(&current)
                .unwrap_or_else(|ind| ind);
        if index * 2 <= left + right {
            left = index;
        } else {
            right = index;
            current &= !(1 << x);
        }
    }
    oxygen_rate = current;

    let mut current = 0;
    let (mut left, mut right) = (0, params.len());

    for x in (0..size).rev() {
        current |= 1 << x;

        let index: usize = left
            + params[left..right]
                .binary_search(&current)
                .unwrap_or_else(|ind| ind);
        if index != left && (index * 2 <= left + right || index == right) {
            right = index;
            current &= !(1 << x);
        } else {
            left = index;
        }
    }
    co2_rate = current;

    (oxygen_rate, co2_rate)
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn simple() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let (size, mut params) = parse(input).unwrap();

        let (gamma_rate, epsilon_rate) = part1(size, &params);
        assert_eq!(22, gamma_rate);
        assert_eq!(9, epsilon_rate);

        let (oxygen_rate, co2_rate) = part2(size, &mut params);
        assert_eq!(23, oxygen_rate);
        assert_eq!(10, co2_rate);
    }
}
