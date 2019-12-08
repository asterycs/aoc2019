use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn n_orbits<'a>(body: &str, orbits: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let stack = &mut vec![(body, 0)];
    let mut hierarchy = Vec::new();

    while let Some((n, c)) = stack.pop() {
        if let Some(satellites) = orbits.get(n) {
            for &s in satellites {
                stack.push((s, c + 1));
                hierarchy.push(s);
            }
        }
    }

    hierarchy
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/6_2.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");
    let input: Vec<Vec<&str>> = input.lines().map(|l| l.split(")").collect()).collect();

    let orbits = &mut HashMap::<&str, Vec<&str>>::new();

    for orbit in input {
        if let [l, r] = orbit[..] {
            if let Some(v) = orbits.get_mut(r) {
                v.push(l);
            } else {
                orbits.insert(r, vec![l]);
            }
        } else {
            panic!("Invalid input: {:?}", orbit);
        }
    }

    let my_hierarchy = n_orbits("YOU", orbits);
    let san_hierarchy = n_orbits("SAN", orbits);

    //println!("my: {:?}", my_hierarchy);
    //println!("san: {:?}", san_hierarchy);

    let mut min_dist = std::usize::MAX;

    for (i, &body) in my_hierarchy.iter().enumerate() {
        if let Some(j) = san_hierarchy.iter().position(|&x| x == body) {
            min_dist = std::cmp::min(min_dist, i + j);
        }
    }

    println!("min dist: {}", min_dist);
}
