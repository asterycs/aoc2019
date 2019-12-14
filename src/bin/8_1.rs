use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/8_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut min_zero_layer = Vec::<u32>::new();
    let mut min_zeros = std::usize::MAX;

    for layer in input.chunks(25 * 6) {
        let zeros = layer.iter().filter(|&c| *c == 0).count();

        if zeros < min_zeros {
            min_zeros = zeros;
            min_zero_layer = layer.to_vec();
        }
    }

    let ones = min_zero_layer.iter().filter(|&c| *c == 1).count();
    let twos = min_zero_layer.iter().filter(|&c| *c == 2).count();

    println!("{}", ones * twos);
}
