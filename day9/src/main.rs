use std::collections::{binary_heap::BinaryHeap, HashMap};
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    io::stdout().write_fmt(format_args!("Part 1: {}\n", part1(&input)))?;
    io::stdout().write_fmt(format_args!("Part 2: {}\n", part2(&input)))?;

    Ok(())
}

fn parse(input: &str) -> (usize, Vec<u8>) {
    let mut width = 0;
    let mut heights = Vec::new();

    for line in input.lines().map(|line| line.trim().as_bytes()) {
        width = line.len();
        heights.extend(line.iter().map(|c| c - b'0'))
    }

    (width, heights)
}

fn part1(input: &str) -> usize {
    let (width, heights) = parse(input);
    let mut sum = 0_usize;
    let mut i = 0_usize;

    while i < heights.len() {
        let rem = i % width;
        if (rem == 0 || heights[i - 1] > heights[i])
            && (rem == width - 1 || heights[i] < heights[i + 1])
        {
            if (i < width || heights[i - width] > heights[i])
                && (i + width >= heights.len() || heights[i] < heights[i + width])
            {
                sum += 1 + heights[i] as usize
            }
            i += 1 + (rem < width - 1) as usize
        } else {
            i += 1
        }
    }

    sum
}

fn dfs(i: usize, width: usize, heights: &mut [u8]) -> usize {
    if heights[i] == 9 {
        return 0;
    }

    let mut size = 1;

    heights[i] = 9;
    if width <= i {
        size += dfs(i - width, width, heights);
    }
    if i % width != 0 {
        size += dfs(i - 1, width, heights);
    }
    if i % width != width - 1 {
        size += dfs(i + 1, width, heights);
    }
    if i + width < heights.len() {
        size += dfs(i + width, width, heights);
    }

    size
}

pub fn part2(input: &str) -> usize {
    let (width, mut heights) = parse(input);

    let mut heap = BinaryHeap::with_capacity(3);
    for i in 0..heights.len() {
        if heights[i] == 9 {
            continue;
        }

        let size = dfs(i, width, &mut heights);
        if heap.len() < 3 {
            heap.push(std::cmp::Reverse(size));
        } else if heap.peek().unwrap().0 < size {
            *heap.peek_mut().unwrap() = std::cmp::Reverse(size);
        }
    }

    heap.iter().map(|t| t.0).product()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example() {
        let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

        assert_eq!(15, part1(&input));
        assert_eq!(1134, part2(&input));
    }
}
