use rayon::prelude::*;
use std::{
    error::Error,
    io::{self, Read},
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

type Prog = Vec<bool>;
type Image = Vec<Vec<bool>>;

fn parse(input: &str) -> (Prog, Image) {
    let mut iter = input.lines();
    let prog = iter.next().unwrap().bytes().map(|b| b == b'#').collect();
    iter.next().unwrap();

    let mut field = vec![];
    for line in iter {
        field.push(line.bytes().map(|b| b == b'#').collect());
    }

    (prog, field)
}

fn enhance(prog: &[bool], mut image: Image, steps: usize) -> Image {
    let mut infinity = false;
    for _ in 0..steps {
        image = step(prog, infinity, &image);
        infinity ^= prog[0];
    }
    image
}

fn step(p: &[bool], infinity: bool, field: &[Vec<bool>]) -> Image {
    let bit = infinity as u16;
    let start_index = (bit << 7) | (bit << 6) | (bit << 4) | (bit << 3) | (bit << 1) | bit;

    let mut new_field = vec![vec![false; field[0].len() + 2]; field.len() + 2];
    new_field.par_iter_mut().enumerate().for_each(|(i, row)| {
        let prev = if i >= 2 { field.get(i - 2) } else { None };
        let curr = if i >= 1 { field.get(i - 1) } else { None };
        let next = field.get(i);

        let mut index = start_index;
        for (j, item) in row.iter_mut().enumerate() {
            let b3 = *prev.and_then(|p| p.get(j)).unwrap_or(&infinity) as u16;
            let b6 = *curr.and_then(|p| p.get(j)).unwrap_or(&infinity) as u16;
            let b9 = *next.and_then(|p| p.get(j)).unwrap_or(&infinity) as u16;

            index = ((index << 1) & 0b110110110) | (b3 << 6) | (b6 << 3) | b9;
            *item = p[index as usize];
        }
    });

    new_field
}

#[allow(dead_code)]
fn print(field: &[Vec<bool>]) {
    for row in field {
        for x in row {
            print!("{}", if *x { '#' } else { '.' })
        }
        println!();
    }
}

fn light_count(field: &[Vec<bool>]) -> u32 {
    field
        .iter()
        .map(|row| row.iter().filter(|b| **b).count())
        .sum::<usize>() as u32
}

fn part1(input: &str) -> u32 {
    let (prog, field) = parse(input);
    let field = enhance(&prog, field, 2);

    light_count(&field)
}

fn part2(input: &str) -> u32 {
    let (prog, field) = parse(input);
    let field = enhance(&prog, field, 50);

    light_count(&field)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(35, part1(&input));
        assert_eq!(3351, part2(&input));

        let input = include_str!("../input");
        assert_eq!(5419, part1(&input));
        assert_eq!(17325, part2(&input));
    }
}
