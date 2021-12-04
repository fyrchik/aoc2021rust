use std::error::Error;
use std::io::{self, Read, Write};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Table {
    x: [usize; 25],
    sums: [usize; 10],
    won: bool,
}

impl Table {
    fn mark(&mut self, n: usize) -> bool {
        if let Some(i) = self.x.iter().position(|&num| num == n) {
            self.sums[i / 5] -= n;
            self.sums[5 + i % 5] -= n;
            self.won = self.sums.contains(&0);
            return self.won;
        }
        false
    }
}

pub fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (numbers, tables) = parse(&input)?;

    let (last_number, sum) = part1(&numbers, tables.clone().as_mut());
    io::stdout().write_fmt(format_args!("Part 1: {}\n", last_number * sum))?;

    let (last_number, sum) = part2(&numbers, tables.clone().as_mut());
    io::stdout().write_fmt(format_args!("Part 2: {}\n", last_number * sum))?;

    Ok(())
}

pub fn parse(input: &str) -> Result<(Vec<usize>, Vec<Table>)> {
    let mut iter = input.lines();
    let mut numbers = Vec::<usize>::new();
    let mut tables = Vec::<Table>::new();

    let line = iter.next().ok_or("expected numbers")?;
    for s in line.split(',') {
        numbers.push(s.parse()?)
    }

    while let Some(_) = iter.next() {
        let mut t = Table {
            x: [0; 25],
            sums: [0; 10],
            won: false,
        };
        for i in 0..5 {
            let line = iter.next().ok_or("can't read next line")?;
            let ss = line.split_whitespace();
            for (j, num) in ss.enumerate() {
                let num = num.parse()?;
                t.x[i * 5 + j] = num;
                t.sums[i] += num;
                t.sums[5 + j] += num;
            }
        }
        tables.push(t);
    }
    Ok((numbers, tables))
}

pub fn part1(numbers: &[usize], tables: &mut [Table]) -> (usize, usize) {
    let mut last = 0_usize;
    let mut max_sum = usize::MIN;
    for n in numbers {
        let mut is_last = false;
        for t in tables.iter_mut() {
            if t.mark(*n) {
                is_last = true;
                max_sum = std::cmp::max(max_sum, t.sums.iter().take(5).sum());
            }
        }
        if is_last {
            last = *n;
            break;
        }
    }
    (last, max_sum)
}

pub fn part2(numbers: &[usize], tables: &mut [Table]) -> (usize, usize) {
    let mut last = 0_usize;
    let mut last_sum = usize::MIN;
    for n in numbers {
        let mut has_some = false;
        for t in tables.iter_mut().filter(|t| !t.won) {
            has_some = true;
            if t.mark(*n) {
                last = *n;
                last_sum = t.sums.iter().take(5).sum();
            }
        }
        if !has_some {
            break;
        }
    }
    (last, last_sum)
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn simple() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";
        let (size, params) = parse(input).unwrap();

        let (last_number, sum) = part1(&size, params.clone().as_mut());
        assert_eq!(24, last_number);
        assert_eq!(188, sum);

        let (last_number, sum) = part2(&size, params.clone().as_mut());
        assert_eq!(13, last_number);
        assert_eq!(148, sum);
    }
}
