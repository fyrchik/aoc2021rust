use std::cmp::{Ord, Ordering, PartialOrd};
use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn parse<T>(input: &str) -> Vec<Vec<T>>
where
    T: From<u8>,
{
    input
        .lines()
        .map(|line| line.bytes().map(|b| T::from(b - b'0')).collect())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    node: usize,
    cost: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

#[inline]
fn risk(i: usize, j: usize, cave: &[Vec<u32>]) -> u32 {
    let row = &cave[i % cave.len()];
    let r = row[j % row.len()];
    (r + (i / cave.len() + j / row.len()) as u32 - 1) % 9 + 1
}

fn shortest_path(cave: &[Vec<u32>], scale: usize) -> u32 {
    let mut v = vec![u32::MAX; scale * scale * cave.len() * cave[0].len()];
    let dist = v.as_mut_slice();
    let mut queue = BinaryHeap::<State>::with_capacity(4);

    queue.push(State { node: 0, cost: 0 });
    dist[0] = 0;

    let width = cave[0].len() * scale;
    while let Some(State { node, cost }) = queue.pop() {
        if node == dist.len() - 1 {
            return dist[node];
        } else if cost > dist[node] {
            continue;
        }

        let (x, y) = (node / width, node % width);
        if x > 0 {
            let alt = cost + risk(x - 1, y, cave);
            let v = node - width;
            if alt < dist[v] {
                queue.push(State { node: v, cost: alt });
                dist[v] = alt;
            }
        }
        if y > 0 {
            let v = node - 1;
            let alt = cost + risk(x, y - 1, cave);
            if alt < dist[v] {
                queue.push(State { node: v, cost: alt });
                dist[v] = alt;
            }
        }
        if y + 1 < width {
            let v = node + 1;
            let alt = cost + risk(x, y + 1, cave);
            if alt < dist[v] {
                queue.push(State { node: v, cost: alt });
                dist[v] = alt;
            }
        }
        if x + 1 < cave.len() * scale {
            let v = node + width;
            let alt = dist[node] + risk(x + 1, y, cave);
            if alt < dist[v] {
                queue.push(State { node: v, cost: alt });
                dist[v] = alt;
            }
        }
    }

    *dist.last().unwrap()
}

pub fn part1(input: &str) -> u32 {
    let cave = parse::<u32>(input);
    shortest_path(&cave, 1)
}

pub fn part2(input: &str) -> u32 {
    let cave = parse::<u32>(input);
    shortest_path(&cave, 5)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(40, part1(&input));
        assert_eq!(315, part2(&input));

        let input = "19999
19111
11191";
        assert_eq!(8, part1(&input));

        let input = include_str!("../input");
        assert_eq!(702, part1(&input));
        assert_eq!(2955, part2(&input));
    }
}
