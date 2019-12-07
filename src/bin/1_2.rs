use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn get_fuel(weight: i32) -> i32 {
    let fuel = std::cmp::max(weight / 3 - 2, 0);

    if fuel > 0 {
        return fuel + get_fuel(fuel);
    } else {
        return fuel;
    }
}

fn main() {
    let filename = &mut PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("inputs/1_1.txt");

    println!("Reading {}", filename.display());

    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut sum = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        sum += get_fuel(line.parse::<i32>().unwrap());
    }

    println!("Sum: {}", sum);
}
