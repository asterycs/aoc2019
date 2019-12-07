use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let filename = &mut PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("inputs/1_1.txt");

    println!("Reading {}", filename.display());

    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut sum = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        let mass = line.parse::<u32>().unwrap();
        sum += mass / 3 - 2;
    }

    println!("Sum: {}", sum);
}
