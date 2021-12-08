use rayon::{iter::ParallelIterator, str::ParallelString};
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let count = part1(&input);
    io::stdout().write_fmt(format_args!("Part 1: {}\n", count))?;

    let count = part2(&input);
    io::stdout().write_fmt(format_args!("Part 2: {}\n", count))?;

    Ok(())
}

fn part1(input: &str) -> usize {
    let mut count = 0_usize;

    for line in input.lines() {
        let (_, out) = line.split_once('|').unwrap();
        for num in out.split_whitespace() {
            count += [2_usize, 3, 4, 7].contains(&num.len()) as usize;
        }
    }

    count
}

#[inline]
fn convert(number: &str) -> u8 {
    number
        .as_bytes()
        .iter()
        .fold(0, |acc, b| acc | (1 << (b - b'a')))
}

pub fn part2(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| {
            let (patterns, out) = line.split_once('|').unwrap();

            let mut numbers = [0_u8; 10];

            // num 0 1 2 3 4 5 6 7 8 9
            // len 6 2 5 5 4 5 6 3 7 6
            let mut i5: usize = 2;
            let mut i6: usize = 0;
            for pat in patterns.split_whitespace() {
                let x = convert(pat);
                match pat.len() {
                    2 => numbers[1] = x,
                    3 => numbers[7] = x,
                    4 => numbers[4] = x,
                    7 => numbers[8] = x,
                    5 => {
                        numbers[i5] = x;
                        // i5 iterates over 2, 3, 5
                        i5 += (i5 == 3) as usize + 1;
                    }
                    6 => {
                        numbers[i6] = x;
                        // i6 iterates over 0, 6, 9
                        i6 += 3 * ((i6 == 0) as usize + 1);
                    }
                    _ => panic!("unexpected pattern"),
                }
            }

            if numbers[9] & numbers[4] != numbers[4] {
                if numbers[0] & numbers[4] != numbers[4] {
                    numbers.swap(9, 6);
                } else {
                    numbers.swap(9, 0)
                }
            }

            if numbers[0] & numbers[1] != numbers[1] {
                numbers.swap(0, 6);
            }

            if numbers[3] & numbers[1] != numbers[1] {
                if numbers[2] & numbers[1] != numbers[1] {
                    numbers.swap(3, 5);
                } else {
                    numbers.swap(3, 2);
                }
            }

            if numbers[5] & numbers[6] != numbers[5] {
                numbers.swap(2, 5);
            }

            out.split_whitespace()
                .map(|s| convert(s))
                .fold(0, |acc, d| {
                    (acc * 10) + numbers.iter().position(|&n| n == d).unwrap()
                })
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(26, part1(input));
        assert_eq!(61229, part2(input));
    }
}
