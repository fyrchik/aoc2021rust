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

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Graph {
    let mut graph = Graph::new();
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(a, b)| {
            graph
                .entry(a)
                .or_insert_with(|| Vec::with_capacity(1))
                .push(b);
            graph
                .entry(b)
                .or_insert_with(|| Vec::with_capacity(1))
                .push(a);
        });
    graph
}

fn dfs<'a>(
    v: &'a str,
    g: &'a Graph,
    can_visit_small: bool,
    visited: &mut HashMap<&'a str, bool>,
) -> usize {
    if v == "end" {
        return 1;
    }

    let is_lower = v.chars().any(char::is_lowercase);
    if is_lower {
        visited.insert(v, true);
    }

    let mut count = 0;
    for &n in &g[v] {
        let &was_visited = visited.get(n).unwrap_or(&false);
        if !was_visited || (can_visit_small && n != "start" && n != "end") {
            count += dfs(n, g, can_visit_small && !was_visited, visited);
            if was_visited {
                visited.insert(n, true);
            }
        }
    }

    if is_lower {
        visited.insert(v, false);
    }

    count
}

fn part1(input: &str) -> usize {
    let g = parse(input);
    dfs("start", &g, false, &mut HashMap::new())
}

fn part2(input: &str) -> usize {
    let g = parse(input);
    dfs("start", &g, true, &mut HashMap::new())
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
