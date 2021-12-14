use std::collections::HashMap;
use std::io::{self, Read};

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn parse(input: &str) -> (&[u8], HashMap<u16, u8>) {
    let mut iter = input.lines();
    let template = iter.next().unwrap().as_bytes();
    iter.next();

    let mut rules = HashMap::new();
    for line in iter {
        let (left, right) = line.split_once(" -> ").unwrap();
        rules.insert(
            ((left.as_bytes()[0] as u16) << 8) | left.as_bytes()[1] as u16,
            right.as_bytes()[0],
        );
    }

    (template, rules)
}

struct Cache {
    indices: [u8; 255],
    frequencies: HashMap<u16, Vec<usize>>,
}

fn prefill_cache(depth: usize, rules: &HashMap<u16, u8>, template: &[u8]) -> Cache {
    let mut indices = [0_u8; 255];
    let mut count = 1;
    for &c in template.iter().chain(rules.values()) {
        if indices[c as usize] == 0 {
            indices[c as usize] = count;
            count += 1;
        }
    }

    let mut prev = HashMap::<u16, Vec<usize>>::new();
    let mut curr = HashMap::<u16, Vec<usize>>::new();

    for &pair in rules.keys() {
        let mut freq = vec![0; count as usize];
        freq[indices[(pair >> 8) as usize] as usize] += 1;
        prev.insert(pair, freq);
        curr.insert(pair, vec![0; count as usize]);
    }

    for _ in 0..depth {
        for (&pair, &mid) in rules {
            let f1 = prev.get(&((pair & 0xFF00) | mid as u16)).unwrap();
            let f2 = prev.get(&(((mid as u16) << 8) | (pair & 0xFF))).unwrap();

            curr.entry(pair).and_modify(|next| {
                for (to, from) in next.iter_mut().zip(f1.iter().zip(f2.iter())) {
                    *to = from.0 + from.1
                }
            });
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    Cache {
        indices,
        frequencies: prev,
    }
}

fn get_freq_diff(template: &[u8], cache: &Cache) -> usize {
    let size = cache.indices.iter().filter(|&x| *x != 0).count();
    let mut freq = template
        .windows(2)
        .map(|w| {
            let key = ((w[0] as u16) << 8) | w[1] as u16;
            cache.frequencies.get(&key).unwrap()
        })
        .fold(vec![0_usize; size + 1], |mut freq, w| {
            for (to, from) in freq.iter_mut().zip(w) {
                *to += from
            }
            freq
        });
    freq[cache.indices[template[template.len() - 1] as usize] as usize] += 1;

    let (min, max) = freq
        .iter()
        .filter(|&f| *f != 0)
        .fold((usize::MAX, 0_usize), |(min, max), &f| {
            (std::cmp::min(min, f), std::cmp::max(max, f))
        });
    max - min
}

pub fn part1(input: &str) -> usize {
    let (template, rules) = parse(input);
    let cache = prefill_cache(10, &rules, template);

    get_freq_diff(template, &cache)
}

pub fn part2(input: &str) -> usize {
    let (template, rules) = parse(input);
    let cache = prefill_cache(40, &rules, template);

    get_freq_diff(template, &cache)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(1588, part1(&input));
        assert_eq!(2188189693529, part2(&input));
    }
}
