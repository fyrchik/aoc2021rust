use std::{
    io::{self, Read},
    ops::{BitOr, Shl},
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn parse(input: &str) -> Packet {
    let message = input.lines().next().unwrap();
    Packet {
        data: (0..message.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&message[i..i + 2], 16).unwrap())
            .collect(),
        pos: 0,
    }
}

struct Packet {
    data: Vec<u8>,
    pos: usize,
}

impl Packet {
    #[inline]
    fn next_bit(&mut self) -> u8 {
        let bit = (self.data[self.pos / 8] >> (7 - self.pos % 8)) & 1;
        self.pos += 1;
        bit
    }

    #[inline]
    fn read_multi<T>(&mut self, n: usize) -> T
    where
        T: From<u8> + Shl<Output = T> + BitOr<Output = T>,
    {
        (0..n)
            .map(|_| self.next_bit())
            .fold(T::from(0_u8), |a, b| (a << T::from(1_u8)) | T::from(b))
    }

    fn eval(&mut self, stack: &mut Vec<u64>) -> u32 {
        let mut ver = ((self.next_bit() << 2) | (self.next_bit() << 1) | self.next_bit()) as u32;

        let typ = (self.next_bit() << 2) | (self.next_bit() << 1) | self.next_bit();
        if typ == 4 {
            let mut num = 0_u64;
            loop {
                let bit = self.next_bit();
                num = (num << 4) | self.read_multi::<u64>(4);
                if bit == 0 {
                    stack.push(num);
                    return ver;
                }
            }
        }

        let mut count = 0_usize;
        if self.next_bit() == 0 {
            let bit_len: usize = self.read_multi(15);
            let end = self.pos + bit_len;
            while self.pos < end {
                count += 1;
                ver += self.eval(stack);
            }
        } else {
            count = self.read_multi(11);
            stack.reserve(count);
            for _ in 0..count {
                ver += self.eval(stack);
            }
        }

        let res: u64 = match typ {
            0 => stack.iter().rev().take(count).sum(),
            1 => stack.iter().rev().take(count).product(),
            2 => stack
                .iter()
                .rev()
                .take(count)
                .fold(u64::MAX, |a, b| a.min(*b)),
            3 => stack.iter().rev().take(count).fold(0, |a, b| a.max(*b)),
            5 => (stack[stack.len() - 2] > stack[stack.len() - 1]) as u64,
            6 => (stack[stack.len() - 2] < stack[stack.len() - 1]) as u64,
            7 => (stack[stack.len() - 2] == stack[stack.len() - 1]) as u64,
            _ => unreachable!(),
        };
        stack.truncate(stack.len() - count);
        stack.push(res);

        ver
    }
}

fn part1(input: &str) -> u32 {
    let mut p = parse(input);
    let mut stack = vec![];
    p.eval(&mut stack)
}

fn part2(input: &str) -> u64 {
    let mut p = parse(input);
    let mut stack = Vec::with_capacity(10);
    p.eval(&mut stack);
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn input() {
        let input = include_str!("../input");
        assert_eq!(953, part1(&input));
        assert_eq!(246225449979, part2(&input));
    }

    #[test]
    fn example_part1() {
        assert_eq!(16, part1("8A004A801A8002F478"));
        assert_eq!(12, part1("620080001611562C8802118E34"));
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn example_part2() {
        assert_eq!(3, part2("C200B40A82"));
        assert_eq!(54, part2("04005AC33890"));
        assert_eq!(7, part2("880086C3E88112"));
        assert_eq!(9, part2("CE00C43D881120"));
        assert_eq!(1, part2("D8005AC2A8F0"));
        assert_eq!(0, part2("F600BC2D8F"));
        assert_eq!(0, part2("9C005AC2F8F0"));
        assert_eq!(1, part2("9C0141080250320F1802104A08"));
    }
}
