use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;


#[derive(Debug, Clone)]
struct Permuter {
    x: Vec<i64>,
    c: Vec<usize>,
    i: usize,
}

impl Permuter {
    fn new(init: Vec<i64>) -> Permuter {
        Permuter {
            c: vec![0; init.len()],
            x: init,
            i: 0,
        }
    }

    // https://en.wikipedia.org/wiki/Heap%27s_algorithm
    fn next(&self) -> Option<Permuter> {
        let mut x = self.x.clone();
        let mut c = self.c.clone();

        let mut i = self.i;
        while i < x.len() {
            if c[i] < i {
                if i % 2 == 0 {
                    let tmp = x[0];
                    x[0] = x[i];
                    x[i] = tmp;
                } else {
                    let tmp = x[c[i]];
                    x[c[i]] = x[i];
                    x[i] = tmp;
                }

                c[i] += 1;
                i = 0;

                return Some(Permuter { x, c, i });
            } else {
                c[i] = 0;
                i += 1;
            }
        }

        None
    }
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/7_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut x = Permuter::new(vec![0, 1, 2, 3, 4]);

    let mut max_sequence = x.clone();
    let mut max_power = 0;

    loop {
        let mut input_queue = VecDeque::new();
        let mut output_queue = vec![0].into_iter().collect::<VecDeque<_>>();

        for i in 0..5 {
            let mut vm = IntcodeVM::new(&program);
            input_queue.push_front(output_queue.pop_back().unwrap());
            input_queue.push_front(x.x[i]);
            run(&mut vm, &mut input_queue, &mut output_queue);
        }

        let power = output_queue.pop_back().unwrap();
        if power > max_power {
            max_power = power;
            max_sequence = x.clone();
        }

        if let Some(p) = x.next() {
            x = p;
        } else {
            break;
        }
    }

    println!("max power: {}", max_power);
    println!("max sequence: {:?}", max_sequence);
}
