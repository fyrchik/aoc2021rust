use std::error::Error;
use std::io::{self, Read, Write};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Table {
    x: [usize; 25],
}

#[derive(Clone)]
#[repr(transparent)]
struct Mask {
    mask: usize,
}

impl Mask {
    const WON: usize = 1 << 25;
    const ROW1: usize = (1 << 5) - 1;
    const ROW2: usize = Mask::ROW1 << 5;
    const ROW3: usize = Mask::ROW2 << 5;
    const ROW4: usize = Mask::ROW3 << 5;
    const ROW5: usize = Mask::ROW4 << 5;
    const COL1: usize = 1 << 20 | 1 << 15 | 1 << 10 | 1 << 5 | 1;
    const COL2: usize = Mask::COL1 << 1;
    const COL3: usize = Mask::COL2 << 1;
    const COL4: usize = Mask::COL3 << 1;
    const COL5: usize = Mask::COL4 << 1;

    #[inline]
    fn won(&self) -> bool {
        self.mask & Mask::WON != 0
    }

    fn mark(&mut self, n: usize) -> bool {
        self.mask |= 1 << n;
        let won = self.mask & Mask::ROW1 == Mask::ROW1
            || self.mask & Mask::ROW2 == Mask::ROW2
            || self.mask & Mask::ROW3 == Mask::ROW3
            || self.mask & Mask::ROW4 == Mask::ROW4
            || self.mask & Mask::ROW5 == Mask::ROW5
            || self.mask & Mask::COL1 == Mask::COL1
            || self.mask & Mask::COL2 == Mask::COL2
            || self.mask & Mask::COL3 == Mask::COL3
            || self.mask & Mask::COL4 == Mask::COL4
            || self.mask & Mask::COL5 == Mask::COL5;
        self.mask |= (won as usize) << 25;
        self.won()
    }
}

impl Table {
    fn mark(&self, n: usize, m: &mut Mask) -> bool {
        for (i, num) in self.x.iter().enumerate() {
            if *num == n {
                return m.mark(i);
            }
        }
        false
    }

    fn sum(&self, m: &Mask) -> usize {
        let mut sum = 0_usize;
        for (i, n) in self.x.iter().enumerate() {
            if m.mask & (1 << i) == 0 {
                sum += n;
            }
        }
        sum
    }
}

pub fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (numbers, tables) = parse(&input)?;

    let (last_number, sum) = part1(&numbers, &tables);
    io::stdout().write_fmt(format_args!("Part 1: {}\n", last_number * sum))?;

    let (last_number, sum) = part2(&numbers, &tables);
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
        let mut t = Table { x: [0; 25] };
        for i in 0..5 {
            let line = iter.next().ok_or("can't read next line")?;
            let ss = line.split_whitespace();
            for (j, num) in ss.enumerate() {
                let num = num.parse()?;
                t.x[i * 5 + j] = num;
            }
        }
        tables.push(t);
    }
    Ok((numbers, tables))
}

pub fn part1(numbers: &[usize], tables: &[Table]) -> (usize, usize) {
    let mut last = 0_usize;
    let mut max_sum = usize::MIN;
    let mut masks = vec![Mask { mask: 0 }; tables.len()];
    for &n in numbers {
        let mut is_last = false;
        for (t, m) in tables.iter().zip(masks.iter_mut()) {
            if t.mark(n, m) {
                is_last = true;
                max_sum = std::cmp::max(max_sum, t.sum(m));
            }
        }
        if is_last {
            last = n;
            break;
        }
    }
    (last, max_sum)
}

pub fn part2(numbers: &[usize], tables: &[Table]) -> (usize, usize) {
    let mut last = 0_usize;
    let mut last_sum = usize::MIN;
    let mut masks = vec![Mask { mask: 0 }; tables.len()];
    for &n in numbers {
        let mut has_some = false;
        for (i, m) in masks.iter_mut().enumerate().filter(|(_, w)| !w.won()) {
            has_some = true;
            if tables[i].mark(n, m) {
                last = n;
                last_sum = tables[i].sum(m);
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

        let (last_number, sum) = part1(&size, &params);
        assert_eq!(24, last_number);
        assert_eq!(188, sum);

        let (last_number, sum) = part2(&size, &params);
        assert_eq!(13, last_number);
        assert_eq!(148, sum);
    }
}
