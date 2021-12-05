use std::cmp::{min, Ordering};
use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(ax: isize, ay: isize, bx: isize, by: isize) -> Line {
        let a = Point { x: ax, y: ay };
        let b = Point { x: bx, y: by };
        if a.cmp(&b).is_le() {
            Line { a, b }
        } else {
            Line { a: b, b: a }
        }
    }

    #[inline]
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    #[inline]
    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    #[inline]
    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    #[inline]
    fn get_equation(&self) -> (isize, isize, isize) {
        (
            self.b.y - self.a.y,
            self.a.x - self.b.x,
            self.b.x * self.a.y - self.a.x * self.b.y,
        )
    }

    fn intersect(&self, other: &Line) -> Option<(Point, Option<Point>)> {
        let (a1, b1, c1) = self.get_equation();

        debug_assert_eq!(0, a1 * self.a.x + b1 * self.a.y + c1);
        debug_assert_eq!(0, a1 * self.b.x + b1 * self.b.y + c1);

        let r3 = a1 * other.a.x + b1 * other.a.y + c1;
        let r4 = a1 * other.b.x + b1 * other.b.y + c1;
        if r3 * r4 > 0 {
            return None;
        }

        let (a2, b2, c2) = other.get_equation();

        debug_assert_eq!(0, a2 * other.a.x + b2 * other.a.y + c2);
        debug_assert_eq!(0, a2 * other.b.x + b2 * other.b.y + c2);

        let r1 = a2 * self.a.x + b2 * self.a.y + c2;
        let r2 = a2 * self.b.x + b2 * self.b.y + c2;
        if r1 * r2 > 0 {
            return None;
        }

        let d = a1 * b2 - a2 * b1;
        if d == 0 {
            return if self.a.cmp(&other.a).is_le() {
                match self.b.cmp(&other.a) {
                    Ordering::Less => None,
                    Ordering::Equal => Some((self.b, None)),
                    Ordering::Greater => Some((other.a, Some(min(self.b, other.b)))),
                }
            } else {
                match self.a.cmp(&other.b) {
                    Ordering::Less => None,
                    Ordering::Equal => Some((self.a, None)),
                    Ordering::Greater => Some((self.a, Some(min(self.b, other.b)))),
                }
            };
        }

        let xd = b1 * c2 - b2 * c1;
        let yd = a2 * c1 - a1 * c2;
        (xd % d == 0 && yd % d == 0).then(|| {
            (
                Point {
                    x: xd / d,
                    y: yd / d,
                },
                None,
            )
        })
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines = parse(&input)?;

    let amount = part1(&lines);
    io::stdout().write_fmt(format_args!("Part 1: {}\n", amount))?;

    let amount = part2(&lines);
    io::stdout().write_fmt(format_args!("Part 2: {}\n", amount))?;

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Line>> {
    let mut lines = Vec::new();
    for s in input.lines() {
        let (pa, pb) = s.split_once("->").ok_or("invalid line format")?;

        let (sx, sy) = pa.split_once(',').ok_or("invalid line format")?;
        let ax = sx.parse()?;
        let ay = sy.parse()?;

        let (sx, sy) = pb.split_once(',').ok_or("invalid line format")?;
        let bx = sx.parse()?;
        let by = sy.parse()?;

        lines.push(Line::new(ax, ay, bx, by));
    }
    lines.sort_by(|p1, p2| -> Ordering {
        match p1.a.cmp(&p2.a) {
            Ordering::Equal => p1.b.cmp(&p2.b),
            r => r,
        }
    });
    Ok(lines)
}

fn part1(lines: &[Line]) -> usize {
    let mut points = HashSet::new();
    for (i, l1) in lines.iter().enumerate().filter(|(_, l)| !l.is_diagonal()) {
        for (p1, r) in lines
            .iter()
            .skip(i + 1)
            .filter(|l2| !l2.is_diagonal())
            .map(|l2| l1.intersect(l2))
            .flatten()
        {
            points.insert(p1);
            if let Some(p2) = r {
                let sx = (p2.x - p1.x).signum();
                let sy = (p2.y - p1.y).signum();
                let mut x = p1.x;
                let mut y = p1.y;
                while sx != 0 && x != p2.x || sy != 0 && y != p2.y {
                    x += sx;
                    y += sy;
                    points.insert(Point { x, y });
                }
            }
        }
    }

    points.len()
}

fn part2(lines: &[Line]) -> usize {
    let mut points = HashSet::new();
    for (i, l1) in lines.iter().enumerate() {
        for (p1, r) in lines.iter().skip(i + 1).filter_map(|l2| l1.intersect(l2)) {
            points.insert(p1);
            if let Some(p2) = r {
                let sx = (p2.x - p1.x).signum();
                let sy = (p2.y - p1.y).signum();
                let mut x = p1.x;
                let mut y = p1.y;
                while sx != 0 && x != p2.x || sy != 0 && y != p2.y {
                    x += sx;
                    y += sy;
                    points.insert(Point { x, y });
                }
            }
        }
    }

    points.len()
}

#[cfg(test)]
mod test {
    use crate::{parse, part1, part2};

    #[test]
    fn simple() {
        let input = "0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2";

        let lines = parse(input).expect("can't parse the input");

        let amount = part1(&lines);
        assert_eq!(5, amount);

        let amount = part2(&lines);
        assert_eq!(12, amount);
    }
}
