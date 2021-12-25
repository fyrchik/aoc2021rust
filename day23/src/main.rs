use std::cmp::*;
use std::collections::*;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn abs_diff(a: u8, b: u8) -> u8 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

// State contains mask of occupied positions and positions of all crabs.
//
// Crabs are enumerated from 0 to 15 grouped by their type,
// i.e. 0..=3 are Amber crabs, 4..=7 are Bronze crabs etc.
//
// Positions are enumerated as follows:
// ############################
// #0 1  2 3  4 5  6 7  8 9 10#
// #### 11 # 15 # 19 # 23 #####
//    # 12 # 16 # 20 # 24 #
//    # 13 # 17 # 21 # 25 #
//    # 14 # 18 # 22 # 26 #
//    #####################
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State(u32, [u8; 16]);

impl State {
    const COSTS: [u32; 4] = [1, 10, 100, 1000];

    fn goal() -> Self {
        let mut goal = State(0, [0u8; 16]);
        for i in 0u8..16 {
            goal.set_no_sort(i as usize, i / 4 * 4 + 11 + i % 4)
        }
        goal
    }

    #[inline]
    fn is_in_hallway(&self, i: usize) -> bool {
        self.1[i] <= 10
    }

    #[inline]
    fn is_target_room_unsafe(&self, i: usize) -> bool {
        let start = i as u8 / 4 * 4 + 11;
        (start..start + 4).any(|pos| {
            self.1
                .iter()
                .enumerate()
                .any(|(j, &x)| j as u8 / 4 != i as u8 / 4 && x == pos)
        })
    }

    #[inline]
    fn free(&self, pos: u8) -> bool {
        self.0 & (1 << pos) == 0
    }

    #[inline]
    fn room(&self, i: usize) -> u8 {
        (self.1[i] - 11) / 4
    }

    #[inline]
    fn is_hallway_free(&self, i: usize, pos: u8) -> bool {
        if self.1[i] < pos {
            (self.1[i] + 1..=pos).all(|i| self.free(i))
        } else {
            (pos..self.1[i]).all(|i| self.free(i))
        }
    }

    #[inline]
    fn set_no_sort(&mut self, i: usize, pos: u8) {
        self.0 &= !(1 << self.1[i]);
        self.0 |= 1 << pos;
        self.1[i] = pos;
    }

    #[inline]
    fn set(&mut self, i: usize, pos: u8) {
        self.set_no_sort(i, pos);
        // Sort items to consider states modulo crab group position.
        let start = i / 4 * 4;
        self.1[start..start + 4].sort_unstable();
    }

    fn path_and_cost(&self, i: usize, dist: u8, pos: u8) -> (u32, State) {
        let mut new_state = *self;
        new_state.set(i, pos);
        (dist as u32 * State::COSTS[i / 4], new_state)
    }

    fn neighbours(&self) -> Vec<(u32, State)> {
        let mut v = vec![];
        for i in 0..self.1.len() {
            let target_room = i as u8 / 4;

            if self.is_in_hallway(i) {
                let room_exit = target_room * 4 + 11;
                let room_hallway = (room_exit - 7) / 2;
                if self.is_target_room_unsafe(i) || !self.is_hallway_free(i, room_hallway) {
                    continue;
                }

                let index = (room_exit..room_exit + 4)
                    .find(|&j| !self.free(j))
                    .unwrap_or(room_exit + 4)
                    - 1;
                let dist = index + 1 - room_exit + abs_diff(self.1[i], room_hallway);
                v.push(self.path_and_cost(i, dist, index))
            } else {
                if self.room(i) == target_room && !self.is_target_room_unsafe(i) {
                    continue;
                }

                let room_exit = self.room(i) * 4 + 11;
                if !(room_exit..self.1[i]).all(|i| self.free(i)) {
                    continue;
                }

                let room_hallway = (room_exit - 7) / 2;
                let room_dist = self.1[i] + 1 - room_exit;
                v.extend(
                    (0..room_hallway)
                        .rev()
                        .take_while(|&n| self.free(n))
                        .chain((room_hallway..=10).take_while(|&n| self.free(n)))
                        .filter(|&n| !matches!(n, 2 | 4 | 6 | 8))
                        .map(|n| {
                            let dist = room_dist + abs_diff(n, room_hallway);
                            self.path_and_cost(i, dist, n)
                        }),
                );
            }
        }
        v
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut pos = vec!['.'; 27];
        for i in 0..16 {
            pos[self.1[i] as usize] = (b'A' + (i as u8 / 4)) as char
        }

        println!("#############");
        print!("#");
        for c in pos.iter().take(11) {
            print!("{}", c);
        }
        println!("#");
        println!("###{}#{}#{}#{}###", pos[11], pos[15], pos[19], pos[23]);
        println!("  #{}#{}#{}#{}#  ", pos[12], pos[16], pos[20], pos[24]);
        println!("  #{}#{}#{}#{}#  ", pos[13], pos[17], pos[21], pos[25]);
        println!("  #{}#{}#{}#{}#  ", pos[14], pos[18], pos[22], pos[26]);
        println!();
    }
}

fn parse(input: &str) -> State {
    let mut iter = input.lines();
    iter.next();
    iter.next();

    let mut s = State(0, [0u8; 16]);
    let mut indices: [usize; 4] = [0, 4, 8, 12];
    for i in 0..2 {
        let line = iter.next().unwrap();
        for (index, c) in line
            .bytes()
            .filter_map(|c| match c {
                b'A'..=b'D' => Some((c - b'A') as usize),
                _ => None,
            })
            .enumerate()
        {
            s.set_no_sort(indices[c], 11 + 4 * index as u8 + i);
            indices[c] += 1;
        }
    }

    s
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Edge {
    node: State,
    cost: u32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.0.cmp(&other.node.0))
    }
}

fn shortest_path(start: State, goal: State) -> Option<u32> {
    let mut dist = HashMap::<State, u32>::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Edge {
        cost: 0,
        node: start,
    });

    while let Some(Edge { cost, node }) = heap.pop() {
        if node == goal {
            return Some(cost);
        }

        if cost > *dist.get(&node).unwrap_or(&u32::MAX) {
            continue;
        }

        for (c, new_state) in node.neighbours() {
            let next = Edge {
                cost: cost + c,
                node: new_state,
            };

            let e = dist.entry(next.node).or_insert_with(|| {
                heap.push(next);
                next.cost
            });
            if next.cost < *e {
                heap.push(next);
                *e = next.cost;
            }
        }
    }

    None
}

fn part1(input: &str) -> u32 {
    let mut start = parse(input);
    for i in 0u8..4 {
        start.set_no_sort(i as usize * 4 + 2, 11 + i * 4 + 2);
        start.set_no_sort(i as usize * 4 + 3, 11 + i * 4 + 3);
        start.1[i as usize * 4..i as usize * 4 + 4].sort_unstable();
    }

    let goal = State::goal();
    shortest_path(start, goal).unwrap()
}

fn part2(input: &str) -> u32 {
    let mut start = parse(input);

    // Move the second row to the fourth row.
    for i in (12..=24).step_by(4) {
        let crab = start.1.iter().position(|&p| p == i).unwrap();
        start.set_no_sort(crab, start.1[crab] + 2);
    }

    // Between the first and second lines of text that contain
    // amphipod starting positions, insert the following lines:
    // #D#C#B#A# | # 12 # 16 # 20 # 24 #
    // #D#B#A#C# | # 13 # 17 # 21 # 25 #
    start.set_no_sort(2, 24); // A in the first row
    start.set_no_sort(3, 21); // A in the second row
    start.set_no_sort(6, 20); // B in the first row
    start.set_no_sort(7, 17); // B in the second row
    start.set_no_sort(10, 16); // C in the first row
    start.set_no_sort(11, 25); // C in the second row
    start.set_no_sort(14, 12); // D in the first row
    start.set_no_sort(15, 13); // D in the second row

    for i in 0..4 {
        start.1[i * 4..i * 4 + 4].sort_unstable();
    }

    let goal = State::goal();
    shortest_path(start, goal).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "#############
#...........#
###B#A#C#D###
  #A#B#C#D#
  #########";
        assert_eq!(46, part1(&input));

        let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        assert_eq!(12521, part1(&input));
        assert_eq!(44169, part2(&input));
    }
}
