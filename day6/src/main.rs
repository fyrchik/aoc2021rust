use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let fishes = parse(&input)?;

    let amount = part1(&fishes, 80);
    io::stdout().write_fmt(format_args!("Part 1: {}\n", amount))?;

    let amount = part1(&fishes, 256);
    io::stdout().write_fmt(format_args!("Part 2: {}\n", amount))?;

    Ok(())
}

fn parse(input: &str) -> Result<Vec<u8>> {
    let mut fishes = Vec::new();
    for s in input.trim().split(',') {
        fishes.push(s.parse()?)
    }
    Ok(fishes)
}

fn part1(fishes: &[u8], days: usize) -> usize {
    let mut hist = [0_usize; 9];
    for f in fishes.iter() {
        hist[*f as usize] += 1;
    }

    for _ in 0..days/7 {
        let baby0 = hist[7];
        let baby1 = hist[8];
        hist[7] = hist[5];
        hist[8] = hist[6];
        hist[5] += hist[3];
        hist[6] += hist[4];
        hist[3] += hist[1];
        hist[4] += hist[2];
        hist[2] += hist[0];
        hist[0] += baby0;
        hist[1] += baby1;
    }
    for _ in 0..days%7 {
        let first = hist[0];
        hist.copy_within(1..=8, 0);
        hist[6] += first;
        hist[8] = first;
    }
    hist.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn simple() {
        let input = "3,4,3,1,2";
        let lines = parse(input).expect("can't parse the input");

        assert_eq!(26, part1(&lines, 18));
        assert_eq!(5934, part1(&lines, 80));
        assert_eq!(26984457539, part1(&lines, 256));
    }
}
