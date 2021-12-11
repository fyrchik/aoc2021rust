use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    io::stdout().write_fmt(format_args!("Part 1: {}\n", part1(&input, 100)))?;
    io::stdout().write_fmt(format_args!("Part 2: {}\n", part2(&input)))?;

    Ok(())
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|&c| c - b'0').collect())
        .collect()
}

fn step(field: &mut [Vec<u8>]) -> usize {
    let value_mask = 0x3F;
    let flash_bit = 7;
    let used_bit = 6;

    for row in field.iter_mut() {
        for cell in row.iter_mut() {
            *cell = ((1 - (*cell >> 7)) * *cell) & value_mask
        }
    }

    let mut count = 0;
    let mut iter = 0;
    loop {
        let old_count = count;
        for i in 0..field.len() {
            for j in 0..field[i].len() {
                // 7-th bit is 1 if a cell was already flashed.
                // 6-th bit is 1 if a cell has been altered.
                let value = field[i][j] & value_mask;
                let overflow = if iter == 0 {
                    field[i][j] += 1;
                    value >= 9
                } else {
                    (field[i][j] & (1 << used_bit)) != 0 && value >= 10
                };

                let high_bit = field[i][j] >> flash_bit;
                if high_bit == 0 && overflow {
                    field[i][j] = 1 << flash_bit;
                    count += 1;
                    if 0 < i {
                        if 0 < j {
                            field[i - 1][j - 1] = (field[i - 1][j - 1] + 1) | (1 << used_bit);
                        }
                        field[i - 1][j] = (field[i - 1][j] + 1) | (1 << used_bit);
                        if j < field[i].len() - 1 {
                            field[i - 1][j + 1] = (field[i - 1][j + 1] + 1) | (1 << used_bit);
                        }
                    }
                    if 0 < j {
                        field[i][j - 1] = (field[i][j - 1] + 1) | (1 << used_bit);
                    }
                    if j < field[i].len() - 1 {
                        field[i][j + 1] = (field[i][j + 1] + 1) | (1 << used_bit);
                    }
                    if i < field.len() - 1 {
                        if 0 < j {
                            field[i + 1][j - 1] = (field[i + 1][j - 1] + 1) | (1 << used_bit);
                        }
                        field[i + 1][j] = (field[i + 1][j] + 1) | (1 << used_bit);
                        if j < field[i].len() - 1 {
                            field[i + 1][j + 1] = (field[i + 1][j + 1] + 1) | (1 << used_bit);
                        }
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
    let mut field = parse(input);
    let mut count = 0;

    for _ in 0..steps {
        count += step(&mut field);
    }
    count
}

pub fn part2(input: &str) -> usize {
    let mut field = parse(input);
    let mut count = 1;
    let size = field.len() * field[0].len();

    while step(&mut field) != size {
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
