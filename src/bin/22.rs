use regex::Regex;
use lazy_static::lazy_static;

use common::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
    Cut(isize),
    DealIncrement(usize),
    DealNew
}

impl Op {
    fn exec(&self, deck: &mut Vec<u64>) {
        match self {
            Op::DealNew => deck.reverse(),
            Op::DealIncrement(increment) => {
                let sz = deck.len();
                let mut other = Vec::new();
                other.resize(sz, 0);

                for (index, value) in deck.into_iter().enumerate() {
                    other[(index * increment) % sz] = *value;
                }

                *deck = other;
            },
            Op::Cut(offset) => {
                let sz = deck.len();

                let (l, r) = if *offset > 0 {
                    deck.split_at(*offset as usize)
                } else  {
                    deck.split_at(sz - (-offset) as usize)
                };

                let mut tmp = r.to_vec();
                tmp.extend_from_slice(l);
                *deck = tmp;
            }
        }
    }

    fn forward(&self, index: usize, deck_size: usize) -> usize {
        match self {
            Op::DealNew => {
                return (deck_size - index - 1).rem_euclid(deck_size);
            },
            Op::DealIncrement(increment) => {
                return mod_mul(index as u64, *increment as u64, deck_size as u64) as usize;
            },
            Op::Cut(offset) => {
                return (index as isize - *offset).rem_euclid(deck_size as isize) as usize;
            }
        }
    }

    fn reverse(&self, index: usize, deck_size: usize) -> usize {
        match self {
            Op::DealNew => {
                return (deck_size - index - 1).rem_euclid(deck_size);
            },
            Op::DealIncrement(increment) => {
                let y = modular_multiplicative_inverse(*increment as u64, deck_size as u64);

                return mod_mul(index as u64, y as u64, deck_size as u64) as usize;
            },
            Op::Cut(offset) => {
                return (index as isize + *offset).rem_euclid(deck_size as isize) as usize;
            }
        }
    }
}

impl From<&str> for Op {
    fn from(str: &str) -> Self {
        lazy_static! {
            static ref CUT: Regex = Regex::new(r"^cut (-?\d+)$").unwrap();
            static ref DEAL_WITH_INCREMENT: Regex = Regex::new(r"^deal with increment (-?\d+)$").unwrap();
            static ref DEAL_INTO_NEW: Regex = Regex::new(r"^deal into new stack$").unwrap();
        }

        if let Some(capture) = CUT.captures(str) {
            return Op::Cut(capture.get(1).unwrap().as_str().parse::<isize>().unwrap());
        } else if let Some(capture) = DEAL_WITH_INCREMENT.captures(str) {
            return Op::DealIncrement(capture.get(1).unwrap().as_str().parse::<usize>().unwrap());
        } else if DEAL_INTO_NEW.is_match(str) {
            return Op::DealNew;
        }else {
            panic!("Unknown operation");
        }
    }
}

fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    if a == 1 {
        return b;
    }
    if b == 1 {
        return a;
    }

    let mut res = 0;
    let mut a = a;
    let mut b = b;

    while b > 0 {
        if b & 1 == 1 {
            res = (res + a).rem_euclid(m);
        }

        b /= 2;
        a = (a * 2).rem_euclid(m);
    }

    res
}

fn apply(operations: &Vec<Op>, deck: &mut Vec<u64>) {
    for op in operations.iter() {
        op.exec(deck);
    }
}

fn forward(index: usize, operations: &Vec<Op>, deck_size: usize) -> usize {
    let mut index = index;
    for op in operations.iter() {
        index = op.forward(index, deck_size);
    }

    index
}

fn part1(input: &String) -> Result<usize,()> {
    let operations = input.lines().into_iter().map(|line| Op::from(line)).collect::<Vec<_>>();

    let deck_size = 10007;
    
    let index = forward(2019, &operations, deck_size);

    Ok(index)
}

// We can trace back the original number by inverting the shuffling operations in the list.
// Deal with increment n:
//    x * n = index (mod s) (1)
//    x can be solved by using the modular multiplicative inverse (https://en.wikipedia.org/wiki/Modular_multiplicative_inverse).
//    For the modular multiplicative inverse holds
//    z * n + y * s = 1 &&
//    z * n - 1 = 0 (mod s) &&
//    z * n = 1 (mod s).
//    We can find z using the extended euclidean algorithm, and then eliminate n in (1).
//    Algorithm adapted from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures.
fn modular_multiplicative_inverse(n: u64, s: u64) -> u64 {
    let mut x0: i64 = 0;
    let mut x1: i64 = 1;

    let mut r0 = s as i64;
    let mut r1 = n as i64;

    while r1 != 0 {
        let quotient = r0 / r1;

        let next_x = x1;
        x1 = x0 - quotient * x1;
        x0 = next_x;

        let next_r = r1;
        r1 = r0 - quotient * r1;
        r0 = next_r;
    }

    if r0 > 1 {
        panic!("{} is not invertible", n);
    }

    if x0 < 0 {
        x0 += s as i64;
    }

    return x0 as u64;
}

fn reverse(index: usize, operations: &Vec<Op>, deck_size: usize) -> usize {
    let mut index = index;
    for op in operations.iter().rev() {
        index = op.reverse(index, deck_size);
    }

    index
}

fn part2(input: &String) -> Result<u64,()> {
    let operations = input.lines().into_iter().map(|line| Op::from(line)).collect::<Vec<_>>();

    let deck_size = 10007;

    let index = reverse(346, &operations, deck_size);

    Ok(mod_mul(2, index as u64, deck_size as u64) as u64)
}

task!(22.txt, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_deal_new() {
        let op = Op::DealNew;
        let mut deck = (0..10).collect();

        op.exec(&mut deck);

        assert_eq!(deck, (0..10).rev().collect::<Vec<_>>());
    }

    #[test]
    fn part1_test_cut_positive() {
        let op = Op::Cut(3);
        let mut deck = (0..10).collect();

        op.exec(&mut deck);

        assert_eq!(deck, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn part1_test_cut_negative() {
        let op = Op::Cut(-4);
        let mut deck = (0..10).collect();

        op.exec(&mut deck);

        assert_eq!(deck, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn part1_test_deal_increment() {
        let op = Op::DealIncrement(3);
        let mut deck = (0..10).collect();

        op.exec(&mut deck);

        assert_eq!(deck, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn part1_cascade1() {
        let ops = vec![Op::DealIncrement(7), Op::DealNew, Op::DealNew];
        let mut deck = (0..10).collect();

        apply(&ops, &mut deck);

        assert_eq!(deck, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn part1_cascade2() {
        let ops = vec![Op::Cut(6), Op::DealIncrement(7), Op::DealNew];
        let mut deck = (0..10).collect();

        apply(&ops, &mut deck);

        assert_eq!(deck, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn part1_cascade3() {
        let ops = vec![Op::DealIncrement(7), Op::DealIncrement(9), Op::Cut(-2)];
        let mut deck = (0..10).collect();

        apply(&ops, &mut deck);

        assert_eq!(deck, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn part1_cascade4() {
        let ops = vec![Op::DealNew, Op::Cut(-2), Op::DealIncrement(7), Op::Cut(8), Op::Cut(-4), Op::DealIncrement(7), Op::Cut(3), Op::DealIncrement(9), Op::DealIncrement(3), Op::Cut(-1)];
        let mut deck = (0..10).collect();

        apply(&ops, &mut deck);

        assert_eq!(deck, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn part1_test_forward_deal_new() {
        let op = Op::DealNew;
        let deck_size = 10;

        let index = op.forward(0, deck_size);

        assert_eq!(index, 9);
    }

    #[test]
    fn part1_test_forward_cut_positive() {
        let op = Op::Cut(3);
        let deck_size = 10;

        let index = op.forward(0, deck_size);

        assert_eq!(index, 7);
    }

    #[test]
    fn part1_test_forward_cut_negative() {
        let op = Op::Cut(-4);
        let deck_size = 10;

        let index = op.forward(0, deck_size);

        assert_eq!(index, 4);
    }

    #[test]
    fn part1_test_forward_deal_increment() {
        let op = Op::DealIncrement(3);
        let deck_size = 10;

        let index = op.forward(1, deck_size);

        assert_eq!(index, 3);
    }

    #[test]
    fn part2_modular_multiplicative_inverse1() {
        let y = modular_multiplicative_inverse(3, 8);

        assert_eq!(y, 3);
    }

    #[test]
    fn part2_test_reverse_deal_increment() {
        let op = Op::DealIncrement(3);
        let deck_size = 10;

        let index = op.reverse(2, deck_size);
        assert_eq!(index, 4);

        let index = op.reverse(0, deck_size);
        assert_eq!(index, 0);

        let index = op.reverse(7, deck_size);
        assert_eq!(index, 9);
    }

    #[test]
    fn part2_test_reverse_deal_new() {
        let op = Op::DealNew;
        let deck_size = 10;

        let index = op.reverse(2, deck_size);
        assert_eq!(index, 7);

        let index = op.reverse(9, deck_size);
        assert_eq!(index, 0);

        let index = op.reverse(4, deck_size);
        assert_eq!(index, 5);

        let deck_size = 3;

        let index = op.reverse(0, deck_size);
        assert_eq!(index, 2);

        let index = op.reverse(2, deck_size);
        assert_eq!(index, 0);

        let index = op.reverse(1, deck_size);
        assert_eq!(index, 1);
    }

    #[test]
    fn part2_test_reverse_cut() {
        let op = Op::Cut(2);
        let deck_size = 10;

        let index = op.reverse(0, deck_size);
        assert_eq!(index, 2);

        let index = op.reverse(1, deck_size);
        assert_eq!(index, 3);

        let index = op.reverse(9, deck_size);
        assert_eq!(index, 1);

        let op = Op::Cut(-2);

        let index = op.reverse(0, deck_size);
        assert_eq!(index, 8);

        let index = op.reverse(9, deck_size);
        assert_eq!(index, 7);

        let index = op.reverse(8, deck_size);
        assert_eq!(index, 6);
    }

    #[test]
    fn part2_test_mod_mul() {
        assert_eq!(mod_mul(1, 1, 2), 1);
        assert_eq!(mod_mul(2, 2, 2), 0);
        assert_eq!(mod_mul(2, 3, 5), 1);
        assert_eq!(mod_mul(1, 0, 2), 0);
        assert_eq!(mod_mul(0, 0, 2), 0);
        assert_eq!(mod_mul(11, 6, 4), 2);
        assert_eq!(mod_mul(27, 81, 99), 9);

        assert_eq!(mod_mul(634591239837, 9837683454, 254), 26);
        assert_eq!(mod_mul(634591239837683454, 9837683454, 254), 248);

        assert_eq!(mod_mul(10123465234878998, 65746311545646431, 10005412336548794), 4652135769797794);
    }
}
