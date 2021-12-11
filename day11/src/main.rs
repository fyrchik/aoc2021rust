use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    io::stdout().write_fmt(format_args!("Part 1: {}\n", part1(&input, 100)))?;
    io::stdout().write_fmt(format_args!("Part 2: {}\n", part2(&input)))?;

    Ok(())
}

fn parse(input: &str) -> (usize, Vec<u8>) {
    let res: Vec<u8> = input
        .lines()
        .flat_map(|line| line.as_bytes().iter().map(|&c| c - b'0'))
        .collect();
    ((res.len() as f64).sqrt() as usize, res)
}

// VALUE_MASK is a mask for retrieving an actual charge of an octopus.
const VALUE_MASK: u8 = 0x3F;
// FLASH_BIT is 1 if a cell was already flashed.
const FLASH_BIT: u8 = 7;
// USED_BIT is 1 if a cell has been altered by the flash of neighbour.
const USED_BIT: u8 = 6;
// USED_MASK is a mask for checking if cell has been altered.
const USED_MASK: u8 = 1 << USED_BIT;

fn mark(i: usize, field: &mut [u8]) {
    if i < field.len() {
        field[i] += 1;
        field[i] |= USED_MASK;
    }
}

fn step(width: usize, field: &mut [u8]) -> usize {
    assert!(width > 0);
    assert!(field.len() == width * width);

    for cell in field.iter_mut() {
        *cell = ((((*cell >> FLASH_BIT == 0) as u8) * *cell) & VALUE_MASK) + 1
    }

    let mut count = 0;
    let mut iter = 0;
    loop {
        let old_count = count;
        for i in 0..field.len() {
            let overflow =
                field[i] & VALUE_MASK >= 10 && (iter == 0 || (field[i] & USED_MASK) != 0);
            let high_bit = field[i] & (1 << FLASH_BIT);
            if high_bit == 0 && overflow {
                field[i] = 1 << FLASH_BIT;
                count += 1;

                // Previous row.
                if width <= i {
                    if i % width != 0 {
                        mark(i - width - 1, field);
                    }
                    mark(i - width, field);
                    if i % width != width - 1 {
                        mark(i - width + 1, field);
                    }
                }

                // Current row.
                if i % width != 0 {
                    mark(i - 1, field);
                }
                if i % width != width - 1 {
                    mark(i + 1, field);
                }

                // Next row.
                if i + width < field.len() {
                    if i % width != 0 {
                        mark(i + width - 1, field);
                    }
                    mark(i + width, field);
                    if i % width != width - 1 {
                        mark(i + width + 1, field);
                    }
                }
            }
        }

        if old_count == count {
            break;
        }
        iter += 1;
    }
    count
}

pub fn part1(input: &str, steps: usize) -> usize {
    let (width, mut field) = parse(input);
    let mut count = 0;

    for _ in 0..steps {
        count += step(width, &mut field);
    }
    count
}

pub fn part2(input: &str) -> usize {
    let (width, mut field) = parse(input);
    let mut count = 1;

    while step(width, &mut field) != width * width {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example() {
        let input = "000\n090\n000";
        assert_eq!(1, part1(&input, 1));
        assert_eq!(1, part1(&input, 2));
        assert_eq!(1, part1(&input, 7));
        assert_eq!(1, part1(&input, 8));
        assert_eq!(10, part1(&input, 9));

        let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";

        assert_eq!(0, part1(&input, 1));
        assert_eq!(35, part1(&input, 2));
        assert_eq!(35 + 45, part1(&input, 3));
        assert_eq!(1656, part1(&input, 100));

        assert_eq!(195, part2(&input));
    }
}
