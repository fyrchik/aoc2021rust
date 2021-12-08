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

fn convert(number: &str) -> usize {
    number
        .as_bytes()
        .iter()
        .fold(0, |acc, b| acc | (1 << (b - b'a')))
}

fn part2(input: &str) -> usize {
    let mut sum = 0_usize;
    let mut numbers = [0_usize; 10];
    let mut five = [0_usize; 3];
    let mut six = [0_usize; 3];

    for line in input.lines() {
        let (patterns, out) = line.split_once('|').unwrap();

        let mut i5: usize = 0;
        let mut i6: usize = 0;
        for pat in patterns.split_whitespace() {
            let x = convert(pat);
            match pat.len() {
                2 => numbers[1] = x,
                3 => numbers[7] = x,
                4 => numbers[4] = x,
                7 => numbers[8] = x,
                5 => {
                    five[i5] = x;
                    i5 += 1
                }
                6 => {
                    six[i6] = x;
                    i6 += 1
                }
                _ => panic!("unexpected pattern"),
            }
        }

        numbers[3] = *five.iter().find(|&n| n & numbers[1] == numbers[1]).unwrap();
        numbers[9] = *six.iter().find(|&n| n & numbers[4] == numbers[4]).unwrap();
        for &n in six.iter() {
            if n == numbers[9] {
                continue;
            }
            if n & numbers[1] == numbers[1] {
                numbers[0] = n;
            } else {
                numbers[6] = n;
            }
        }
        for &n in five.iter() {
            if n == numbers[3] {
                continue;
            }
            if n & numbers[6] == n {
                numbers[5] = n
            } else {
                numbers[2] = n
            }
        }

        sum += out
            .split_whitespace()
            .map(|s| convert(s))
            .fold(0, |acc, d| {
                (acc * 10) + numbers.iter().position(|&n| n == d).unwrap()
            });
    }

    sum
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
