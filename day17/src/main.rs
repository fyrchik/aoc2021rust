use std::error::Error;
use std::io::{self, Read};
use std::result::Result;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn parse(input: &str) -> Option<((i32, i32), (i32, i32))> {
    input
        .trim_end()
        .strip_prefix("target area: ")
        .and_then(|s| s.split_once(", "))
        .and_then(|(xs, ys)| xs[2..].split_once("..").zip(ys[2..].split_once("..")))
        .and_then(|((x1, x2), (y1, y2))| {
            x1.parse()
                .ok()
                .zip(x2.parse().ok())
                .zip(y1.parse().ok().zip(y2.parse().ok()))
        })
}

fn calculate(x1: i32, x2: i32, y1: i32, y2: i32) -> (i32, u32) {
    let mut y_max = 0;
    let mut count = 0_u32;

    for y in y1..y1.abs() {
        let mut vy = y;
        let mut cy = 0;
        let mut n = 0;
        while y2 < cy {
            n += 1;
            cy += vy;
            vy -= 1;
        }

        let mut last_x = x2;
        while y1 <= cy {
            let sum = n * (n - 1) / 2;

            let mut c = 0_u32;
            for x in (0..=last_x).rev() {
                let dist = if n <= x { x * n - sum } else { x * (x + 1) / 2 };
                c += (x1 <= dist && dist <= x2) as u32;
                if dist < x1 {
                    last_x = x;
                    break;
                }
            }
            if c != 0 {
                y_max = y_max.max(y * (y + 1) / 2);
            }

            count += c;
            n += 1;
            cy += vy;
            vy -= 1;
        }
    }
    (y_max, count)
}

pub fn part1(input: &str) -> i32 {
    let ((x1, x2), (y1, y2)) = parse(input).unwrap();
    let (y_max, _) = calculate(x1, x2, y1, y2);
    y_max
}

pub fn part2(input: &str) -> u32 {
    let ((x1, x2), (y1, y2)) = parse(input).unwrap();
    let (_, count) = calculate(x1, x2, y1, y2);
    count
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(45, part1(&input));
        assert_eq!(112, part2(&input));

        let input = include_str!("../input");
        assert_eq!(7503, part1(&input));
        assert_eq!(3229, part2(&input));
    }
}
