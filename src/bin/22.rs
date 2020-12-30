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
    fn exec(&self, deck: &mut Vec<u32>) {
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

fn apply(operations: &Vec<Op>, deck: &mut Vec<u32>) {
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

task!(22.txt, part1);

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
}
