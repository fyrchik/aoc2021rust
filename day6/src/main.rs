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

fn prev(hist: &mut [usize]) {
    // 1 at 0 result in [x0, x1, x2, x3, x4, x5, x6,    x7, x8] after n days
    // 1 at 1 result in [x8, x0, x1, x2, x3, x4, x5, x6-x8, x7] after n days
    //dbg!(&hist);
    let x8 = hist[8];
    hist.copy_within(0..=7, 1);
    hist[0] = x8;
    hist[7] = hist[7].saturating_sub(x8);
}

fn part1(fishes: &[u8], days: usize) -> usize {
    let mut hist = [0_usize; 9];
    for f in fishes.iter() {
        hist[*f as usize] += 1;
    }

    // zero contains 1 fish at 0 day before spawning.
    let mut zero = [0_usize; 9];
    zero[0] = 1;

    for i in (0..=usize::BITS - days.leading_zeros()).rev() {
        let mut tmp = [0_usize; 9];

        // Calculate how much fish will spawn after 2*n days
        // given zero = how much fish will spawn after n days.
        let mut p = zero;
        for i in zero.iter() {
            for j in 0..9 {
                tmp[j] += i * p[j];
            }
            prev(&mut p);
        }

        if days & (1 << (i as usize)) != 0 {
            let first = tmp[0];
            tmp.copy_within(1..=8, 0);
            tmp[6] += first;
            tmp[8] = first;
        }
        zero = tmp;
    }

    let mut tmp = [0_usize; 9];
    for i in hist.iter() {
        for j in 0..9 {
            tmp[j] += i * zero[j];
        }
        prev(&mut zero);
    }
    tmp.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn zero_only() {
        let input = "0";
        let lines = parse(input).expect("can't parse the input");

        assert_eq!(1, part1(&lines, 0));
        assert_eq!(2, part1(&lines, 1));
        assert_eq!(2, part1(&lines, 7));
        assert_eq!(3, part1(&lines, 8));
        assert_eq!(4, part1(&lines, 10));
    }

    #[test]
    fn example() {
        let input = "3,4,3,1,2";
        let lines = parse(input).expect("can't parse the input");

        assert_eq!(26, part1(&lines, 18));
        assert_eq!(5934, part1(&lines, 80));
    }
}
