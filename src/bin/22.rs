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

    fn reverse_track(&self, index: usize, deck: &Vec<u64>) -> usize {
        match self {
            Op::DealNew => {
                let sz = deck.len();
                let half = (sz / 2) as isize;
                let index = index as isize;

                if sz % 2 == 0 {
                    return (-(index - half) + half - 1) as usize;
                } else {
                    return (-(index - half) + half) as usize;
                }
            },
            Op::DealIncrement(increment) => {
                let sz = deck.len();
                let x = modular_multiplicative_inverse(*increment as u64, sz as u64);

                return (index * x as usize) % sz;
            },
            Op::Cut(offset) => {
                let sz = deck.len();

                if *offset > 0 {
                    let offset = *offset as usize;
                    if index < offset {
                        return index + sz - offset;
                    } else {
                        return index - offset;
                    }
                } else  {
                    let offset = -offset as usize;

                    if sz - index > offset {
                        return index + offset;
                    } else {
                        return index - (sz - offset);
                    }
                };
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

fn apply(operations: &Vec<Op>, deck: &mut Vec<u64>) {
    for op in operations.iter() {
        op.exec(deck);
    }
}

fn part1(input: &String) -> Result<usize,()> {
    let operations = input.lines().into_iter().map(|line| Op::from(line)).collect::<Vec<_>>();

    let mut deck = (0..10007).collect::<Vec<_>>();
    
    apply(&operations, &mut deck);

    let position = deck.iter().position(|card| *card == 2019).unwrap();

    Ok(position)
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

fn reverse(index: usize, operations: &Vec<Op>, deck: &mut Vec<u64>) -> usize {
    let mut index = index;
    for op in operations.iter().rev() {
        index = op.reverse_track(index, deck);
    }

    index
}

fn part2(input: &String) -> Result<u64,()> {
    let operations = input.lines().into_iter().map(|line| Op::from(line)).collect::<Vec<_>>();

    let mut deck = (0..10007).collect::<Vec<_>>();
    
    let index = reverse(2020, &operations, &mut deck);

    Ok(index as u64)
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
    fn part1_modular_multiplicative_inverse1() {
        let y = modular_multiplicative_inverse(3, 8);

        assert_eq!(y, 3);
    }

    #[test]
    fn part2_test_reverse_deal_increment() {
        let op = Op::DealIncrement(3);
        let deck = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];

        let index = op.reverse_track(2, &deck);

        assert_eq!(index, 4);
    }

    #[test]
    fn part2_test_reverse_deal_new() {
        let op = Op::DealNew;
        let deck = (0..10).collect();

        let index = op.reverse_track(2, &deck);
        assert_eq!(index, 7);

        let index = op.reverse_track(9, &deck);
        assert_eq!(index, 0);

        let index = op.reverse_track(4, &deck);
        assert_eq!(index, 5);

        let deck = (0..3).collect();

        let index = op.reverse_track(0, &deck);
        assert_eq!(index, 2);

        let index = op.reverse_track(2, &deck);
        assert_eq!(index, 0);

        let index = op.reverse_track(1, &deck);
        assert_eq!(index, 1);
    }

    #[test]
    fn part2_test_reverse_cut() {
        let op = Op::Cut(2);
        let deck = (0..10).collect();

        let index = op.reverse_track(0, &deck);
        assert_eq!(index, 8);

        let index = op.reverse_track(1, &deck);
        assert_eq!(index, 9);

        let index = op.reverse_track(2, &deck);
        assert_eq!(index, 0);

        let op = Op::Cut(-2);

        let index = op.reverse_track(1, &deck);
        assert_eq!(index, 3);

        let index = op.reverse_track(9, &deck);
        assert_eq!(index, 1);

        let index = op.reverse_track(8, &deck);
        assert_eq!(index, 0);
    }
}
