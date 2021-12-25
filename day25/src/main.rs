use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cucumber {
    Empty,
    East,
    South,
}

fn parse(input: &str) -> Vec<Vec<Cucumber>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'.' => Cucumber::Empty,
                    b'>' => Cucumber::East,
                    b'v' => Cucumber::South,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn print_field(f: &[Vec<Cucumber>]) {
    for row in f {
        for x in row {
            print!(
                "{}",
                match x {
                    Cucumber::Empty => '.',
                    Cucumber::East => '>',
                    Cucumber::South => 'v',
                }
            );
        }
        println!();
    }
}

fn part1(input: &str) -> u32 {
    let mut field = parse(input);

    let mut step = 0;
    let mut moved = true;
    while moved {
        moved = false;

        for row in field.iter_mut() {
            let move_zero = row[0] == Cucumber::East && row[1] == Cucumber::Empty;
            let last = row.len() - 1;

            let mut j = 0;
            while j < last {
                if row[j] == Cucumber::East && row[j + 1] == Cucumber::Empty {
                    moved = true;
                    row[j] = Cucumber::Empty;
                    row[j + 1] = Cucumber::East;
                    j += 2;
                } else {
                    j += 1;
                }
            }

            if j == last && row[last] == Cucumber::East && row[0] == Cucumber::Empty && !move_zero {
                moved = true;
                row[last] = Cucumber::Empty;
                row[0] = Cucumber::East;
            }
        }

        for i in 0..field[0].len() {
            let move_zero = field[0][i] == Cucumber::South && field[1][i] == Cucumber::Empty;
            let last = field.len() - 1;

            let mut j = 0;
            while j < last {
                if field[j][i] == Cucumber::South && field[j + 1][i] == Cucumber::Empty {
                    moved = true;
                    field[j][i] = Cucumber::Empty;
                    field[j + 1][i] = Cucumber::South;
                    j += 2;
                } else {
                    j += 1;
                }
            }

            if j == last
                && field[last][i] == Cucumber::South
                && field[0][i] == Cucumber::Empty
                && !move_zero
            {
                moved = true;
                field[last][i] = Cucumber::Empty;
                field[0][i] = Cucumber::South;
            }
        }

        step += 1;
    }

    step
}

fn part2(_: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        assert_eq!(58, part1(&input));
    }
}
