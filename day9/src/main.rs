use std::collections::binary_heap::BinaryHeap;
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

fn merge(aliases: &mut [usize], a: usize, b: usize) {
    let mut x = aliases[a];
    let mut y = aliases[b];

    while aliases[x] != x || aliases[y] != y {
        debug_assert!(aliases[x] <= x);
        debug_assert!(aliases[y] <= y);
        x = aliases[x];
        y = aliases[y];
    }

    let min = x.min(y);
    aliases[x] = min;
    aliases[y] = min;
    aliases[a] = min;
    aliases[b] = min;
}

pub fn part2(input: &str) -> usize {
    let (width, heights) = parse(input);

    let mut line = vec![usize::MAX; width];
    let mut aliases = Vec::<usize>::new();
    let mut sizes = Vec::<usize>::new();
    aliases.push(0);

    for row in heights.chunks(width) {
        let mut current_size = 0;
        for (&r, cell) in row.iter().zip(line.iter_mut()) {
            if r != 9 {
                current_size += 1;
                if *cell != usize::MAX && aliases[*cell] != *aliases.last().unwrap() {
                    let l = aliases.len() - 1;
                    merge(&mut aliases, l, *cell);
                }
                *cell = aliases.len() - 1;
            } else {
                *cell = usize::MAX;
                if current_size != 0 {
                    sizes.push(current_size);
                    aliases.push(aliases.len());
                    current_size = 0;
                }
            }
        }
        if current_size != 0 {
            sizes.push(current_size);
            aliases.push(aliases.len());
        }
    }

    let mut heap = BinaryHeap::new();
    for i in (1..aliases.len() - 1).rev() {
        if aliases[i] != i {
            sizes[aliases[i]] += sizes[i];
        } else {
            let size = sizes[i];
            if heap.len() < 3 {
                heap.push(std::cmp::Reverse(size));
            } else if heap.peek().unwrap().0 < size {
                *heap.peek_mut().unwrap() = std::cmp::Reverse(size);
            }
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
