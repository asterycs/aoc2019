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

    let mut canvas = vec![0; 25 * 6];

    for layer in input.chunks(25 * 6).rev() {
        for (i, v) in layer.iter().enumerate() {
            match v {
                2 => (),
                _ => canvas[i] = *v,
            }
        }
    }

    for row in canvas.chunks(25) {
        let x = row
            .iter()
            .map(|x| match x {
                1 => '*',
                0 => ' ',
                _ => '?',
            })
            .collect::<String>();
        println!("{:?}", x);
    }
}
