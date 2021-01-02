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
                assert!(*increment < deck.len());

                let sz = deck.len();
                let mut other = Vec::new();
                other.resize(sz, 0);

                for (index, value) in deck.into_iter().enumerate() {
                    other[(index * increment) % sz] = *value;
                }

                *deck = other;
            },
            Op::Cut(offset) => {
                assert!((offset.abs() as usize) < deck.len());

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
                assert!(*increment < deck_size);

                return mod_mul(index as i64, *increment as i64, deck_size as u64) as usize;
            },
            Op::Cut(offset) => {
                assert!((offset.abs() as usize) < deck_size);

                return (index as isize - *offset).rem_euclid(deck_size as isize) as usize;
            }
        }
    }
    
    fn forward_collect(&self, lc: &mut LinearCongruence) {
        let lc_new;
        match self {
            Op::DealNew => {
                lc_new = LinearCongruence{a: -1, b: lc.m as i128 - 1, m: lc.m};
            },
            Op::DealIncrement(increment) => {
                assert!((*increment as i128) < lc.m);

                lc_new = LinearCongruence{a: *increment as i128, b: 0, m: lc.m};
            },
            Op::Cut(offset) => {
                assert!((offset.abs() as i128) < lc.m);

                lc_new = LinearCongruence{a: 1, b: -*offset as i128, m: lc.m};
            }
        }

        *lc = lc_new.combine(lc);
    }

    fn reverse(&self, index: usize, deck_size: usize) -> usize {
        match self {
            Op::DealNew => {
                return (deck_size - index - 1).rem_euclid(deck_size);
            },
            Op::DealIncrement(increment) => {
                assert!(*increment < deck_size);

                let y = modular_multiplicative_inverse(*increment as i128, deck_size as i128);

                return mod_mul(index as i64, y as i64, deck_size as u64) as usize;
            },
            Op::Cut(offset) => {
                assert!((offset.abs() as usize) < deck_size);

                return (index as isize + *offset).rem_euclid(deck_size as isize) as usize;
            }
        }
    }

    fn reverse_collect(&self, lc: &mut LinearCongruence) {
        let lc_new;
        match self {
            Op::DealNew => {
                lc_new = LinearCongruence{a: -1, b: lc.m as i128 - 1, m: lc.m};
            },
            Op::DealIncrement(increment) => {
                assert!((*increment as i128) < lc.m);

                let y = modular_multiplicative_inverse(*increment as i128, lc.m as i128);

                lc_new = LinearCongruence{a: y, b: 0, m: lc.m};
            },
            Op::Cut(offset) => {
                assert!((offset.abs() as i128) < lc.m);

                lc_new = LinearCongruence{a: 1, b: *offset as i128, m: lc.m};
            }
        }

        *lc = lc_new.combine(lc);
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


// Peasant multiplication
fn mod_mul(a: i64, b: i64, m: u64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }

    let mut res = 0;
    let mut a = (b.signum() * a) as i128;
    let mut b = b.abs();
    let m = m as i128;

    while b != 0 {
        if b & 1 == 1 {
            res = ((res as i128 + a).rem_euclid(m)) as i64;
        }

        b /= 2;
        a = (a * 2).rem_euclid(m);
    }

    res
}

// From https://en.wikipedia.org/wiki/Modular_exponentiation
fn mod_pow(a: u128, b: u128, m: u128) -> u128 {
    if m == 1 {
        return 0;
    }

    let t = m - 1;
    let t = t.checked_mul(t);
    assert_ne!(t, None);

    let mut res = 1;
    let mut a = a.rem_euclid(m);
    let mut b = b;

    while b > 0 {
        if b.rem_euclid(2) == 1 {
            res = (res * a).rem_euclid(m);
        }

        b /= 2;
        a = (a * a).rem_euclid(m);
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

fn forward_collect(operations: &Vec<Op>, deck_size: usize) -> LinearCongruence {
    let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

    for op in operations.iter() {
        op.forward_collect(&mut lc);
    }

    lc
}

fn reverse_collect(operations: &Vec<Op>, deck_size: usize) -> LinearCongruence {
    let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

    for op in operations.iter().rev() {
        op.reverse_collect(&mut lc);
    }

    lc
}

fn part1(input: &String) -> Result<usize,()> {
    let operations = input.lines().into_iter().map(|line| Op::from(line)).collect::<Vec<_>>();

    let deck_size = 10007;

    let lc = forward_collect(&operations, deck_size);

    Ok(lc.apply(2019) as usize)
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
fn modular_multiplicative_inverse(n: i128, s: i128) -> i128 {
    let mut x0: i128 = 0;
    let mut x1: i128 = 1;

    let mut r0 = s as i128;
    let mut r1 = n as i128;

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
        x0 += s;
    }

    return x0;
}

fn reverse(index: usize, operations: &Vec<Op>, deck_size: usize) -> usize {
    let mut index = index;
    for op in operations.iter().rev() {
        index = op.reverse(index, deck_size);
    }

    index
}

#[derive(Debug)]
struct LinearCongruence {
    // a*x + b (mod m)
    a: i128,
    b: i128,
    m: i128
}

impl LinearCongruence {
    fn apply(&self, x: i128) -> i128 {
        (x * self.a + self.b).rem_euclid(self.m)
    }

    fn combine(&self, other: &LinearCongruence) -> LinearCongruence {
        assert_eq!(self.m, other.m);

        let m = self.m;

        LinearCongruence{a: mod_mul(self.a as i64, other.a as i64, m as u64) as i128, b: (mod_mul(self.a as i64, other.b as i64, m as u64) as i128 + self.b).rem_euclid(m as i128), m: m}
    }

    fn skip_forward(&self, times: u64) -> LinearCongruence {
        let apt = mod_pow(self.a as u128, times as u128, self.m as u128) as i128;
        let a = apt;
        let b = mod_mul((apt - 1) as i64, modular_multiplicative_inverse(self.a - 1, self.m) as i64, self.m as u64) as i128 * self.b;
        let m = self.m;

        LinearCongruence{a, b, m}
    }

    fn skip_backward(&self, times: u64) -> LinearCongruence {
        let a_inv = modular_multiplicative_inverse(self.a as i128, self.m) as i64;
        let b_inv = -mod_mul(a_inv, self.b as i64, self.m as u64);
        let a_inv_pt = mod_pow(a_inv as u128, times as u128, self.m as u128);
        let a = a_inv_pt as i128;
        let b = (a_inv_pt as i128 - 1) * b_inv as i128;
        let num = modular_multiplicative_inverse(a_inv as i128 - 1, self.m);
        let b = mod_mul(num as i64, b as i64, self.m as u64) as i128;
        let m = self.m;

        LinearCongruence{a, b, m}
    }
}

fn part2(input: &String) -> Result<u64,()> {
    let operations = input.lines().into_iter().map(|line| Op::from(line)).collect::<Vec<_>>();
    let deck_size = 119_315_717_514_047;

    let lc = reverse_collect(&operations, deck_size);

    let times = 101_741_582_076_661;
    let lc = lc.skip_forward(times);

    let start_index = lc.apply(2020);

    Ok(start_index as u64)
}

task!(22.txt, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    // These tests are given in the description and are used to test the reference implementations.
    // The reference implementations are then used in subsequent tests to verify the faster functions.
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
    //
    // Reference tests end here.
    //

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
    fn part1_test_forward_collect_deal_new() {
        let op = Op::DealNew;
        let deck_size: usize = 7;

        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        op.forward_collect(&mut lc);

        assert_eq!(lc.apply(0), 6);
        assert_eq!(lc.apply(1), 5);
        assert_eq!(lc.apply(2), 4);

        assert_eq!(lc.apply(3), 3);

        assert_eq!(lc.apply(4), 2);
        assert_eq!(lc.apply(5), 1);
        assert_eq!(lc.apply(6), 0);
    }

    #[test]
    fn part1_test_forward_collect_cut_positive() {
        let op = Op::Cut(3);
        let deck_size: usize = 7;
        let index = 0;
        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        op.forward_collect(&mut lc);

        let index = lc.apply(index);

        assert_eq!(index, 4);
    }

    #[test]
    fn part1_test_forward_collect_cut_negative() {
        let op = Op::Cut(-4);
        let deck_size: usize = 7;
        let index = 0;
        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        op.forward_collect(&mut lc);

        let index = lc.apply(index);

        assert_eq!(index, 4);
    }

    #[test]
    fn part1_test_forward_collect_deal_increment() {
        let op = Op::DealIncrement(3);
        let deck_size: usize = 7;
        let index = 1;
        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        op.forward_collect(&mut lc);

        let index = lc.apply(index);

        assert_eq!(index, 3);
    }

    #[test]
    fn part1_collect_cascade1() {
        let ops = vec![Op::DealIncrement(5), Op::DealNew, Op::DealNew];
        let deck_size: usize = 7;
        let mut deck = (0..deck_size as u64).collect();

        let lc = forward_collect(&ops, deck_size);

        // Reference
        apply(&ops, &mut deck);

        for i in 0..deck_size {
            assert_eq!(lc.apply(deck[i] as i128) as usize, i);
        }
    }

    #[test]
    fn part1_collect_cascade2() {
        let ops = vec![Op::Cut(6), Op::DealIncrement(5), Op::DealNew];
        let deck_size: usize = 7;
        let mut deck = (0..deck_size as u64).collect();

        let lc = forward_collect(&ops, deck_size);

        // Reference
        apply(&ops, &mut deck);

        for i in 0..deck_size {
            assert_eq!(lc.apply(deck[i] as i128) as usize, i);
        }
    }

    #[test]
    fn part1_collect_cascade3() {
        let ops = vec![Op::DealIncrement(3), Op::DealIncrement(4), Op::Cut(-2)];
        let deck_size: usize = 7;
        let mut deck = (0..deck_size as u64).collect();

        let lc = forward_collect(&ops, deck_size);

        // Reference
        apply(&ops, &mut deck);
        
        for i in 0..deck_size {
            assert_eq!(lc.apply(deck[i] as i128) as usize, i);
        }
    }

    #[test]
    fn part1_collect_cascade4() {
        let ops = vec![Op::DealNew, Op::Cut(-2), Op::DealIncrement(5), Op::Cut(5), Op::Cut(-4), Op::DealIncrement(4), Op::Cut(3), Op::DealIncrement(2), Op::DealIncrement(3), Op::Cut(-1)];
        let deck_size: usize = 7;
        let mut deck = (0..deck_size as u64).collect();

        let lc = forward_collect(&ops, deck_size);

        // Reference
        apply(&ops, &mut deck);
        
        for i in 0..deck_size {
            assert_eq!(lc.apply(deck[i] as i128) as usize, i);
        }
    }

    #[test]
    fn part2_modular_multiplicative_inverse1() {
        let y = modular_multiplicative_inverse(3, 8);

        assert_eq!(y, 3);
    }

    #[test]
    fn part2_test_reverse_deal_increment() {
        let op = Op::DealIncrement(3);
        let deck_size = 7;

        let index = op.reverse(5, deck_size);
        assert_eq!(index, 4);

        let index = op.reverse(0, deck_size);
        assert_eq!(index, 0);

        let index = op.reverse(4, deck_size);
        assert_eq!(index, 6);
    }

    #[test]
    fn part2_test_reverse_deal_new() {
        let op = Op::DealNew;
        let deck_size = 7;

        let index = op.reverse(2, deck_size);
        assert_eq!(index, 4);

        let index = op.reverse(6, deck_size);
        assert_eq!(index, 0);

        let index = op.reverse(4, deck_size);
        assert_eq!(index, 2);

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
        let deck_size = 7;

        let index = op.reverse(0, deck_size);
        assert_eq!(index, 2);

        let index = op.reverse(1, deck_size);
        assert_eq!(index, 3);

        let index = op.reverse(6, deck_size);
        assert_eq!(index, 1);

        let op = Op::Cut(-2);

        let index = op.reverse(0, deck_size);
        assert_eq!(index, 5);

        let index = op.reverse(6, deck_size);
        assert_eq!(index, 4);

        let index = op.reverse(5, deck_size);
        assert_eq!(index, 3);
    }

    #[test]
    fn part2_test_mod_mul() {
        assert_eq!(mod_mul(1, 1, 2), 1);
        assert_eq!(mod_mul(-1, -1, 2), 1);
        assert_eq!(mod_mul(1, -1, 2), 1);
        assert_eq!(mod_mul(-1, 1, 2), 1);
        assert_eq!(mod_mul(2, 2, 2), 0);
        assert_eq!(mod_mul(2, 3, 5), 1);
        assert_eq!(mod_mul(1, 0, 2), 0);
        assert_eq!(mod_mul(0, 0, 2), 0);
        assert_eq!(mod_mul(11, 6, 4), 2);
        assert_eq!(mod_mul(27, 81, 99), 9);
        assert_eq!(mod_mul(-27, 81, 99), 90);
        assert_eq!(mod_mul(27, -81, 99), 90);

        assert_eq!(mod_mul(634591239837, 9837683454, 254), 26);
        assert_eq!(mod_mul(634591239837683454, 9837683454, 254), 248);

        assert_eq!(mod_mul(10123465234878998, 65746311545646431, 10005412336548794), 4652135769797794);
        assert_eq!(mod_mul(10123465234878998, -65746311545646431, 10005412336548794), 5353276566751000);
        assert_eq!(mod_mul(-10123465234878998, 65746311545646431, 10005412336548794), 5353276566751000);

        assert_eq!(mod_mul(71, 2999721784550400, 10007), 6010);
        assert_eq!(mod_mul(71, -432980374444289705, 10007), 389);
    }

    #[test]
    fn part2_test_mod_pow() {
        assert_eq!(mod_pow(1, 1, 2), 1);
        assert_eq!(mod_pow(1, 0, 2), 1);

        assert_eq!(mod_pow(15, 31, 3), 0);
        assert_eq!(mod_pow(111, 317, 99), 45);
    }

    #[test]
    fn part2_test_repeat_forward() {
        let ops = vec![Op::DealIncrement(5)];
        let deck_size = 7usize;
        let mut deck = (0..deck_size as u64).collect();
        let repetitions = 3;

        for _ in 0..repetitions {
            apply(&ops, &mut deck);
        }

        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        for op in ops.iter() {
            op.forward_collect(&mut lc);
        }
        
        let lc = lc.skip_forward(repetitions);

        for i in 0..deck_size {
            assert_eq!(lc.apply(deck[i] as i128) as usize, i);
        }
    }

    #[test]
    fn part2_test_repeat_forward_cascade() {
        let ops = vec![Op::DealNew, Op::Cut(-2), Op::DealIncrement(5), Op::Cut(5), Op::Cut(-4), Op::DealIncrement(4), Op::Cut(3), Op::DealIncrement(2), Op::DealIncrement(2), Op::Cut(-1)];
        let deck_size = 7usize;
        let mut deck = (0..deck_size as u64).collect();
        let repetitions = 3;

        for _ in 0..repetitions {
            apply(&ops, &mut deck);
        }

        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        for op in ops.iter() {
            op.forward_collect(&mut lc);
        }

        let lc = lc.skip_forward(repetitions);

        for i in 0..deck_size {
            assert_eq!(lc.apply(deck[i] as i128) as usize, i);
        }
    }

    #[test]
    fn part2_test_repeat_backwards_cascade() {
        let ops = vec![Op::DealNew, Op::Cut(-2), Op::DealIncrement(5), Op::Cut(5), Op::Cut(-4), Op::DealIncrement(4), Op::Cut(3), Op::DealIncrement(2), Op::DealIncrement(2), Op::Cut(-1)];
        let deck_size = 7usize;
        let mut deck = (0..deck_size as u64).collect();
        let repetitions = 33;

        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        for op in ops.iter() {
            op.forward_collect(&mut lc);
        }

        let lc = lc.skip_forward(repetitions);

        for _ in 0..repetitions {
            apply(&ops, &mut deck);
        }

        let lc = lc.skip_backward(repetitions);

        for i in 0..deck_size {
            assert_eq!(lc.apply(i as i128) as u64, deck[i]);
        }
    }

    #[test]
    fn part2_test_repeat_forward_backwards_cascade() {
        let ops = vec![Op::DealNew, Op::Cut(-2), Op::DealIncrement(5), Op::Cut(5), Op::Cut(-4), Op::DealIncrement(4), Op::Cut(3), Op::DealIncrement(2), Op::DealIncrement(2), Op::Cut(-1)];
        let deck_size = 7usize;
        let deck = (0..deck_size as u64).collect::<Vec<_>>();
        let repetitions = 101741582076661;

        let mut lc = LinearCongruence{a: 1, b: 0, m: deck_size as i128};

        for op in ops.iter() {
            op.forward_collect(&mut lc);
        }

        let lc = lc.skip_forward(repetitions);

        let deck = deck.into_iter().map(|x| lc.apply(x as i128) as u64).collect::<Vec<_>>();

        let lc = lc.skip_backward(repetitions);

        for i in 0..deck_size {
            assert_eq!(lc.apply(i as i128) as u64, deck[i]);
        }
    }

    #[test]
    fn part2_test_repeat_forward_reverse_cascade() {
        let ops = vec![Op::DealNew, Op::Cut(-2), Op::DealIncrement(5), Op::Cut(5), Op::Cut(-4), Op::DealIncrement(4), Op::Cut(3), Op::DealIncrement(2), Op::DealIncrement(2), Op::Cut(-1)];
        let deck_size = 7usize;
        let deck = (0..deck_size as u64).collect::<Vec<_>>();
        let repetitions = 101741582076661;

        let lc_forward = forward_collect(&ops, deck_size);
        let lc_reverse = reverse_collect(&ops, deck_size);

        let lc_forward = lc_forward.skip_forward(repetitions);
        let lc_reverse = lc_reverse.skip_forward(repetitions);

        let deck = deck.into_iter().map(|x| lc_forward.apply(x as i128) as u64).collect::<Vec<_>>();
        let deck = deck.into_iter().map(|x| lc_reverse.apply(x as i128) as u64).collect::<Vec<_>>();

        for i in 0..deck_size {
            assert_eq!(deck[i], i as u64);
        }
    }
}
