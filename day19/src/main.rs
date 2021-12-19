use rayon::prelude::*;
use std::{
    collections::HashSet,
    error::Error,
    io::{self, Read},
    ops::{Add, Sub},
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i16, i16, i16);

impl Add for Point {
    type Output = Point;
    fn add(self, p: Point) -> Self {
        Point(self.0 + p.0, self.1 + p.1, self.2 + p.2)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, p: Point) -> Self {
        Point(self.0 - p.0, self.1 - p.1, self.2 - p.2)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct Rotation(u8);

impl Rotation {
    fn inverse(self) -> Rotation {
        if self.0 < 10 {
            self
        } else {
            Rotation(self.0 + 1 - 2 * (self.0 % 2))
        }
    }

    fn combine(self, other: Rotation) -> Rotation {
        let x = Point(1, 0, 0);
        let y = Point(0, 1, 0);
        let z = Point(0, 0, 1);

        for r in 0..24 {
            if x.rotate(self).rotate(other) == x.rotate(Rotation(r))
                && y.rotate(self).rotate(other) == y.rotate(Rotation(r))
                && z.rotate(self).rotate(other) == z.rotate(Rotation(r))
            {
                return Rotation(r);
            }
        }
        unreachable!()
    }
}

impl Point {
    fn manhattan(&self, other: Point) -> i16 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }

    fn rotate(&self, kind: Rotation) -> Self {
        match kind.0 {
            // Rotations of order 2 (which are inverse of themselves).
            0 => Point(self.0, self.1, self.2),
            1 => Point(self.0, -self.1, -self.2),
            2 => Point(-self.0, self.1, -self.2),
            3 => Point(-self.0, -self.1, self.2),
            4 => Point(self.2, -self.1, self.0),
            5 => Point(-self.2, -self.1, -self.0),
            6 => Point(-self.0, self.2, self.1),
            7 => Point(-self.0, -self.2, -self.1),
            8 => Point(self.1, self.0, -self.2),
            9 => Point(-self.1, -self.0, -self.2),

            // Other rotations group by mutually inverse pairs.
            10 => Point(self.0, self.2, -self.1),
            11 => Point(self.0, -self.2, self.1),

            12 => Point(self.1, -self.0, self.2),
            13 => Point(-self.1, self.0, self.2),

            14 => Point(self.1, self.2, self.0),
            15 => Point(self.2, self.0, self.1),

            16 => Point(self.1, -self.2, -self.0),
            17 => Point(-self.2, self.0, -self.1),

            18 => Point(-self.1, self.2, -self.0),
            19 => Point(-self.2, -self.0, self.1),

            20 => Point(-self.1, -self.2, self.0),
            21 => Point(self.2, -self.0, -self.1),

            22 => Point(self.2, self.1, -self.0),
            23 => Point(-self.2, self.1, self.0),

            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<HashSet<Point>> {
    let mut scanners: Vec<HashSet<Point>> = vec![];
    for line in input.lines().map(|line| line.trim_start()) {
        if line.is_empty() {
            continue;
        } else if line.starts_with("---") {
            scanners.push(HashSet::new());
        } else {
            let mut ps = line.split(',');
            let x = ps.next().unwrap().parse().unwrap();
            let y = ps.next().unwrap().parse().unwrap();
            let z = ps.next().unwrap().parse().unwrap();
            scanners.last_mut().unwrap().insert(Point(x, y, z));
        }
    }

    scanners
}

fn union(scanners: &[HashSet<Point>]) -> (HashSet<Point>, Vec<(Point, Rotation)>) {
    let mut points = HashSet::<Point>::new();
    for p in scanners[0].iter() {
        points.insert(*p);
    }

    let mut count = 1_usize;
    let mut visited: Vec<bool> = vec![false; scanners.len()];
    // Stores scanner position and rotation relative to scanner 0.
    // If visited[j] is false, store zero value.
    let mut positions: Vec<(Point, Rotation)> = vec![(Point(0, 0, 0), Rotation(0)); scanners.len()];
    visited[0] = true;

    while count != scanners.len() {
        for i in 0..scanners.len() {
            if !visited[i] {
                continue;
            }

            for j in 0..scanners.len() {
                if i == j || visited[j] {
                    continue;
                }
                if let Some(r) = intersect(&scanners[i], &scanners[j]) {
                    count += 1;

                    // Determine rotation relative to 0 scanner.
                    let v = positions[i].0 + r.0.rotate(positions[i].1);
                    let rot = r.1.combine(positions[i].1);
                    positions[j] = (v, rot);
                    visited[j] = true;

                    points.extend(scanners[j].iter().map(|p| v + p.rotate(rot)));
                }
            }
        }
    }

    (points, positions)
}

fn part1(input: &str) -> usize {
    let scanners = parse(input);
    let (points, _) = union(&scanners);

    points.len()
}

fn part2(input: &str) -> i16 {
    let scanners = parse(input);
    let (_, ss) = union(&scanners);

    let mut max = i16::MIN;
    for i in 0..ss.len() {
        max = max.max(
            ss[i + 1..]
                .iter()
                .map(|s| ss[i].0.manhattan(s.0))
                .max()
                .unwrap_or(i16::MIN),
        );
    }
    max
}

fn intersect(a: &HashSet<Point>, b: &HashSet<Point>) -> Option<(Point, Rotation)> {
    (0..24).into_par_iter().find_map_any(|rot| {
        let rot = Rotation(rot);
        let inv = rot.inverse();

        for anchor_a in a.iter() {
            for anchor_b in b {
                let mut count = 0;
                let mut rem = a.len();
                let b_zero = *anchor_a - anchor_b.rotate(rot);

                for p in a.iter().map(|p| (*p - b_zero).rotate(inv)) {
                    if count + rem < 12 {
                        break;
                    }

                    count += b.get(&p).is_some() as usize;
                    if count >= 12 {
                        return Some((b_zero, rot));
                    }

                    rem -= 1;
                }
            }
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn inverse() {
        let x = Point(1, 0, 0);
        let y = Point(0, 1, 0);
        let z = Point(0, 0, 1);

        for rot in (0..24).map(|r| Rotation(r)) {
            assert_eq!(x, x.rotate(rot).rotate(rot.inverse()));
            assert_eq!(y, y.rotate(rot).rotate(rot.inverse()));
            assert_eq!(z, z.rotate(rot).rotate(rot.inverse()));
        }
    }

    #[test]
    fn example() {
        let input = "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401
        
        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390
        
        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562
        
        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596
        
        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14";
        assert_eq!(79, part1(&input));
        assert_eq!(3621, part2(&input));

        let input = include_str!("../input");
        assert_eq!(449, part1(&input));
        assert_eq!(13128, part2(&input));
    }
}
