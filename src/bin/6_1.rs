use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/6_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");
    let input: Vec<Vec<&str>> = input.lines().map(|l| l.split(")").collect()).collect();

    //println!("input: {:?}", input);

    let orbits = &mut HashMap::<&str, Vec<&str>>::new();

    for orbit in input {
        if let [l, r] = orbit[..] {
            if let Some(v) = orbits.get_mut(l) {
                v.push(r);
            } else {
                orbits.insert(l, vec![r]);
            }
        } else {
            panic!("Invalid input");
        }
    }

    //println!("orbits: {:?}", orbits);

    let stack = &mut vec![("COM", 0)];
    let mut cntr = 0;

    while let Some((n, c)) = stack.pop() {
        if let Some(satellites) = orbits.get(n) {
            for &s in satellites {
                stack.push((s, c + 1));
                cntr += c + 1;
            }
        }
    }

    println!("cntr: {}", cntr);
}
