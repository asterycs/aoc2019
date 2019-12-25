use std::env;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl FromStr for Moon {
    type Err = ParseIntError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_re = Regex::new(r"x=(-?\d+)").unwrap();
        let y_re = Regex::new(r"y=(-?\d+)").unwrap();
        let z_re = Regex::new(r"z=(-?\d+)").unwrap();
    
        let x = x_re.captures(s).unwrap().get(1).unwrap().as_str().parse::<isize>()?;
        let y = y_re.captures(s).unwrap().get(1).unwrap().as_str().parse::<isize>()?;
        let z = z_re.captures(s).unwrap().get(1).unwrap().as_str().parse::<isize>()?;

        Ok(Moon { pos: [x, y, z], vel: [0, 0, 0] })
    }
}

impl Moon {
    fn get_kin(&self) -> isize {
        self.vel[0].abs() + self.vel[1].abs() + self.vel[2].abs()
    }

    fn get_pot(&self) -> isize {
        self.pos[0].abs() + self.pos[1].abs() + self.pos[2].abs()
    }
}

fn get_total_energy(moons: &Vec<Moon>) -> isize {
    let mut energy = 0;

    for m in moons.iter() {
        energy += m.get_pot() * m.get_kin();
    }

    energy
}

fn get_acc(l: isize, r: isize) -> isize {
    if l < r {
        1
    }else if l > r {
        -1
    }else {
        0
    }
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/12_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");
    let input = input.lines().collect::<Vec<_>>();

    let moons = &mut input.iter().map(|l| Moon::from_str(l).unwrap()).collect::<Vec<_>>();

    for m in moons.iter_mut() {
        println!("{:?}", m);
    }

    println!("");

    for t in 0..1000 {
        for i in 0..moons.len() {
            for j in i..moons.len() {
                for axis in 0..3 {
                    moons[i].vel[axis] += get_acc(moons[i].pos[axis], moons[j].pos[axis]);
                    moons[j].vel[axis] -= get_acc(moons[i].pos[axis], moons[j].pos[axis]);
                }
            }
        }

        println!("Time {}:", t+1);
        for m in moons.iter_mut() {
            for axis in 0..3 {
                m.pos[axis] += m.vel[axis];
            }

            println!("{:?}", m);
        }

        println!("Total energy: {}", get_total_energy(moons));
        println!("");
    }
    
}