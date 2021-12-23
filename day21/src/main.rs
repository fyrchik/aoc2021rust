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

fn parse(input: &str) -> (u32, u32) {
    let mut iter = input.lines();
    let a = iter.next().unwrap()[28..].parse().unwrap();
    let b = iter.next().unwrap()[28..].parse().unwrap();
    (a, b)
}

fn part1(input: &str) -> u32 {
    let (mut a, mut b) = parse(input);
    let (mut score_a, mut score_b) = (0_u32, 0_u32);
    let mut count = 0;
    let initial_sum = 3; // 0 + 1 + 2

    a -= 1;
    b -= 1;
    loop {
        a = (a + (initial_sum + count * 9) % 100 + 3) % 10;
        count += 1;
        score_a += a + 1;
        if score_a >= 1000 {
            break;
        }

        b = (b + (initial_sum + count * 9) % 100 + 3) % 10;
        count += 1;
        score_b += b + 1;
        if score_b >= 1000 {
            break;
        }
    }
    score_a.min(score_b) as u32 * count * 3
}

const STEPS: [(usize, u64); 7] = [
    (3, 1), // 1 + 1 + 1
    (4, 3), // 1 + 1 + 2
    (5, 6), // 1 + 1 + 3 == 1 + 2 + 2
    (6, 7), // 1 + 2 + 3 == 2 + 2 + 2
    (7, 6), // 2 + 2 + 3 == 1 + 3 + 3
    (8, 3), // 2 + 3 + 3
    (9, 1), // 3 + 3 + 3
];

fn part2(input: &str) -> u64 {
    let (a, b) = parse(input);

    const MAX_TURNS: usize = 11;
    const MAX_SCORE: usize = 21;

    // Count how much possibilities are there for a player to reach n points.
    // *_state[x][y][z] = number of ways to have z points at position y after x turns.
    let mut a_state = [[[0_u64; MAX_SCORE]; 10]; MAX_TURNS];
    let mut b_state = [[[0_u64; MAX_SCORE]; 10]; MAX_TURNS];
    a_state[0][a as usize - 1][0] = 1;
    b_state[0][b as usize - 1][0] = 1;

    let mut a_final = [0_u64; MAX_TURNS];
    let mut b_final = [0_u64; MAX_TURNS];
    for turn in 1..MAX_TURNS {
        for (step, sum) in STEPS {
            for pos in 0..=9 {
                let new_pos = (pos + step) % 10;
                for score in 0..MAX_SCORE {
                    let old_a = (sum as u64) * a_state[turn - 1][pos][score];
                    let old_b = (sum as u64) * b_state[turn - 1][pos][score];

                    let new_score = score + new_pos + 1;
                    if new_score >= MAX_SCORE {
                        a_final[turn] += old_a;
                        b_final[turn] += old_b;
                    } else {
                        a_state[turn][new_pos][new_score] += old_a;
                        b_state[turn][new_pos][new_score] += old_b;
                    }
                }
            }
        }
    }

    let mut a_won = 0;
    let mut b_won = 0;
    for turn in 1..MAX_TURNS {
        let a_less: u64 = a_state[turn]
            .iter()
            .map(|row| row.iter().sum::<u64>())
            .sum();
        let b_less: u64 = b_state[turn - 1]
            .iter()
            .map(|row| row.iter().sum::<u64>())
            .sum();
        a_won += a_final[turn] * b_less;
        b_won += b_final[turn] * a_less;
    }

    a_won.max(b_won)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "Player 1 starting position: 4
Player 2 starting position: 8";
        assert_eq!(739785, part1(&input));
        assert_eq!(444356092776315, part2(&input));

        let input = include_str!("../input");
        assert_eq!(805932, part1(&input));
        assert_eq!(133029050096658, part2(&input));
    }
}
