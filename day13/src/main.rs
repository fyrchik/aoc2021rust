use std::{
    fmt::Write,
    io::{self, Read},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}\n", part1(&input));
    println!("Part 2:\n{}\n", part2(&input));

    Ok(())
}

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<bool>>) {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut instructions: Vec<usize> = Vec::new();
    let (mut x_max, mut y_max) = (0_usize, 0_usize);
    let mut met_empty = false;

    for line in input.lines() {
        if !met_empty {
            match line.split_once(',') {
                Some((sx, sy)) => {
                    let x = sx.parse().unwrap();
                    let y = sy.parse().unwrap();
                    points.push((x, y));
                    x_max = std::cmp::max(x_max, x);
                    y_max = std::cmp::max(y_max, y);
                }
                None => met_empty = true,
            }
        } else {
            let line = line.strip_prefix("fold along ").unwrap();
            let coord: usize = line[2..].parse().unwrap();
            instructions
                .push(coord | (((line.as_bytes()[0] == b'y') as usize) << (usize::BITS - 1)));
        }
    }

    let mut field: Vec<Vec<bool>> = vec![vec![false; x_max + 1]; y_max + 1];
    for p in points {
        field[p.1][p.0] = true;
    }
    (instructions, field)
}

fn dump(field: &[Vec<bool>], x_stop: usize, y_stop: usize) -> String {
    let mut result = String::new();

    for row in field.iter().take(y_stop) {
        for &x in row.iter().take(x_stop) {
            result.write_char(if x { '#' } else { '.' }).unwrap();
        }
        result.write_char('\n').unwrap();
    }
    result
}

fn fold(
    instruction: usize,
    x_stop: usize,
    y_stop: usize,
    field: &mut [Vec<bool>],
) -> (usize, usize) {
    assert!(y_stop <= field.len());

    let value = instruction & (usize::MAX >> 1);

    if instruction & (1 << (usize::BITS - 1)) != 0 {
        let (to, from) = field[..y_stop].split_at_mut(value);
        for (f, t) in from.iter().skip(1).zip(to.iter_mut().rev()) {
            for (x, y) in f.iter().zip(t.iter_mut()) {
                *y |= *x;
            }
        }
        (x_stop, value)
    } else {
        for row in field[..y_stop].iter_mut() {
            let (to, from) = row.split_at_mut(value);
            for (x, y) in from.iter().skip(1).zip(to.iter_mut().rev()) {
                *y |= *x;
            }
        }
        (value, y_stop)
    }
}

fn part1(input: &str) -> usize {
    let (instructions, mut field) = parse(input);

    let (x_stop, y_stop) = fold(instructions[0], field[0].len(), field.len(), &mut field);
    field
        .iter()
        .take(y_stop)
        .map(|row| row.iter().take(x_stop).filter(|&v| *v).count())
        .sum()
}

fn part2(input: &str) -> String {
    let (instructions, mut field) = parse(input);

    let (x_stop, y_stop) = instructions
        .iter()
        .fold((field[0].len(), field.len()), |stop, &i| {
            fold(i, stop.0, stop.1, &mut field)
        });

    dump(&field, x_stop, y_stop)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example() {
        let input = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0

fold along y=7
fold along x=5";
        assert_eq!(17, part1(&input));
        assert_eq!(
            "#####
#...#
#...#
#...#
#####
.....
.....\n",
            part2(&input)
        );
    }
}
