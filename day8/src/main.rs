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
                // 2 3 4 5  6  7
                // 1 7 4 i5 i6 8
                let l = pat.len();
                let is2 = (l == 2) as usize;
                let is3 = (l == 3) as usize;
                let is4 = (l == 4) as usize;
                let is5 = (l == 5) as usize;
                let is6 = (l == 6) as usize;
                let is7 = (l == 7) as usize;
                let index = 1 * is2 + 7 * is3 + 4 * is4 + 8 * is7 + i5 * is5 + i6 * is6;
                numbers[index] = convert(pat);
                i5 += is5 * (1 + ((i5 == 3) as usize));
                i6 += is6 * 3 * (((i6 == 0) as usize) + 1);
            }

            // ii is 0 if 9 is on its place.
            let ii = (numbers[9] & numbers[4] != numbers[4]) as usize;
            let index = (numbers[0] & numbers[4] != numbers[4]) as usize;
            numbers.swap(9 * ii, ii * index * 6);

            // index is 0 if 0 is on its place.
            let index = (numbers[0] & numbers[1] != numbers[1]) as usize;
            numbers.swap(0, index * 6);

            // ii is 0 if 3 is on its place.
            let ii = (numbers[3] & numbers[1] != numbers[1]) as usize;
            let index = (numbers[2] & numbers[1] != numbers[1]) as usize;
            numbers.swap(ii * 3, ii * (2 + index * 3));

            // index is 0 if 5 is on its place;
            let index = (numbers[5] & numbers[6] != numbers[5]) as usize;
            numbers.swap(2, 2 + index * 3);

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
