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

#[derive(Debug, Clone)]
struct Number {
    depth: u32,
    value: u32,
}

#[allow(dead_code)]
fn print(a: &[Number], pos: usize, depth: u32) -> usize {
    if a[pos].depth == depth {
        print!("{}", a[pos].value);
        pos + 1
    } else {
        print!("[");
        let pos = print(a, pos, depth + 1);
        print!(",");
        let pos = print(a, pos, depth + 1);
        print!("]");
        if depth == 0 {
            println!();
        }
        pos
    }
}

fn reduce(a: &mut Vec<Number>) {
    let mut i = 0;
    while i < a.len() {
        if a[i].depth > 4 {
            if 0 < i {
                a[i - 1].value += a[i].value;
            }
            if i + 2 < a.len() {
                a[i + 2].value += a[i + 1].value;
            }
            a.remove(i + 1);
            a[i].value = 0;
            a[i].depth = 4;
            i = 0;
            continue;
        }
        i += 1;
    }

    i = 0;
    while i < a.len() {
        if a[i].value >= 10 {
            let Number { depth, value } = a[i];
            let (l, r) = (value / 2, value - value / 2);
            if depth == 4 {
                if i + 1 < a.len() {
                    a[i + 1].value += r;
                }
                a[i].value = 0;
                if 0 < i {
                    a[i - 1].value += l;
                }
            } else {
                a[i] = Number {
                    depth: depth + 1,
                    value: l,
                };
                a.insert(
                    i + 1,
                    Number {
                        depth: depth + 1,
                        value: r,
                    },
                );
            }
            i = 0;
            continue;
        }
        i += 1;
    }
}

fn add(a: &mut Vec<Number>, b: &[Number]) {
    a.extend_from_slice(b);
    a.iter_mut().for_each(|n| n.depth += 1);
    reduce(a);
}

fn parse(input: &str) -> Vec<Vec<Number>> {
    let mut numbers = vec![];
    let mut depth = 0;
    for line in input.lines() {
        let mut v = vec![];
        for b in line.bytes() {
            match b {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b',' => {}
                b'0'..=b'9' => v.push(Number {
                    depth,
                    value: (b - b'0') as u32,
                }),
                _ => unreachable!(),
            }
        }
        numbers.push(v);
    }
    numbers
}

fn magnitude(a: &[Number], pos: usize, depth: u32) -> (usize, u32) {
    if a[pos].depth == depth {
        (pos + 1, a[pos].value)
    } else {
        let (p1, x) = magnitude(a, pos, depth + 1);
        let (p2, y) = magnitude(a, p1, depth + 1);
        (p2, x * 3 + y * 2)
    }
}

fn part1(input: &str) -> u32 {
    let mut numbers = parse(input);
    let mut value = std::mem::take(&mut numbers[0]);

    for n in numbers[1..].iter() {
        add(&mut value, n);
    }

    let (_, m) = magnitude(&value, 0, 0);
    m
}

fn part2(input: &str) -> u32 {
    let mut max = u32::MIN;
    let numbers = parse(input);

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            let mut x = numbers[i].clone();
            add(&mut x, &numbers[j]);

            let (_, m) = magnitude(&x, 0, 0);
            max = max.max(m);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn magnitude() {
        assert_eq!(143, part1("[[1,2],[[3,4],5]]"));
        assert_eq!(1384, part1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
        assert_eq!(445, part1("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
        assert_eq!(791, part1("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
        assert_eq!(1137, part1("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
        assert_eq!(
            3488,
            part1("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn example() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(4140, part1(&input));
        assert_eq!(3993, part2(&input));
    }
}
