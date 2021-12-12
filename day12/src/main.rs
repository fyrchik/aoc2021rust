use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    io::stdout().write_fmt(format_args!("Part 1: {}\n", part1(&input)))?;
    io::stdout().write_fmt(format_args!("Part 2: {}\n", part2(&input)))?;

    Ok(())
}

type Graph = Vec<Vec<usize>>;

const START_VERTICE: usize = 0;
const END_VERTICE: usize = 1;
// UPPER_BIT is 1 if vertice name is in uppercase.
const UPPER_BIT: u32 = usize::BITS - 1;
// VALUE_MASK is a mask to retrieve real vertex index.
const VALUE_MASK: usize = (1 << UPPER_BIT) - 1;

fn parse(input: &str) -> Graph {
    let mut vertices = HashMap::<&str, usize>::new();
    vertices.insert("start", START_VERTICE);
    vertices.insert("end", END_VERTICE);

    let mut graph = vec![vec![], vec![]];
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(a_name, b_name)| {
            let &mut a = vertices.entry(a_name).or_insert_with(|| {
                graph.push(Vec::with_capacity(1));
                graph.len() - 1
            });
            let &mut b = vertices.entry(b_name).or_insert_with(|| {
                graph.push(Vec::with_capacity(1));
                graph.len() - 1
            });
            graph[a].push(b | ((b_name.chars().any(char::is_uppercase) as usize) << UPPER_BIT));
            graph[b].push(a | ((a_name.chars().any(char::is_uppercase) as usize) << UPPER_BIT));
        });
    graph
}

fn dfs(v: usize, g: &[Vec<usize>], can_visit_small: bool, visited: &mut [bool]) -> usize {
    if v == END_VERTICE {
        return 1;
    }

    let is_lower = v & (1 << UPPER_BIT) == 0;
    if is_lower {
        visited[v & VALUE_MASK] = true;
    }

    let mut count = 0;
    for &n in &g[v & VALUE_MASK] {
        let was_visited = visited[n & VALUE_MASK];
        if !was_visited || (can_visit_small && n != START_VERTICE && n != END_VERTICE) {
            count += dfs(n, g, can_visit_small && !was_visited, visited);
            if was_visited {
                visited[n] = true;
            }
        }
    }

    if is_lower {
        visited[v & VALUE_MASK] = false;
    }

    count
}

fn part1(input: &str) -> usize {
    let g = parse(input);
    dfs(START_VERTICE, &g, false, &mut vec![false; g.len()])
}

fn part2(input: &str) -> usize {
    let g = parse(input);
    dfs(START_VERTICE, &g, true, &mut vec![false; g.len()])
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn example() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        assert_eq!(10, part1(&input));
        assert_eq!(36, part2(&input));

        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        assert_eq!(19, part1(&input));
        assert_eq!(103, part2(&input));

        let input = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";
        assert_eq!(226, part1(&input));
        assert_eq!(3509, part2(&input));
    }
}
