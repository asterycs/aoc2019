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
    filename.push("inputs/7_2.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut x = Permuter::new(vec![5, 6, 7, 8, 9]);

    let mut max_sequence = x.clone();
    let mut max_power = 0;

    loop {
        let vms = &mut [
            IntcodeVM::new(&program),
            IntcodeVM::new(&program),
            IntcodeVM::new(&program),
            IntcodeVM::new(&program),
            IntcodeVM::new(&program)
        ];

        let vm_status = &mut [
            VMStatus::Ok,
            VMStatus::Ok,
            VMStatus::Ok,
            VMStatus::Ok,
            VMStatus::Ok,
        ];
        
        let input_queue = &mut VecDeque::new();
        let output_queue = &mut vec![0].into_iter().collect::<VecDeque<_>>();

        let mut i = 0;
        let mut round = 0;
        loop {
            if let VMStatus::Halted = vm_status[i] {
                break;
            };

            if output_queue.len() > 0 {
                input_queue.push_front(output_queue.pop_back().unwrap());
            }

            if round == 0 {
                input_queue.push_front(x.x[i]);
            }

            vm_status[i] = run(&mut vms[i], &mut *input_queue, &mut *output_queue);

            i += 1;

            if i > 4 {
                round += 1;
                i = 0;
            }
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
