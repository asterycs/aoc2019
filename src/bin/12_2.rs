use std::env;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
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

fn get_acc(l: isize, r: isize) -> isize {
    if l < r {
        1
    }else if l > r {
        -1
    }else {
        0
    }
}

fn is_axis_similar(l: &Vec<Moon>, r: &Vec<Moon>, axis: usize) -> bool {
    for i in 0..l.len() {
        if l[i].pos[axis] != r[i].pos[axis] || l[i].vel[axis] != r[i].vel[axis] {
            return false;
        }
    }

    return true;
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b { 
        if a > b {
           a = a - b; 
        }else{
           b = b - a; 
        }
    }
    return a;
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/12_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");
    let input = input.lines().collect::<Vec<_>>();

    let moons = &mut input.iter().map(|l| Moon::from_str(l).unwrap()).collect::<Vec<_>>();

    let initial = moons.clone();

    for m in moons.iter_mut() {
        println!("{:?}", m);
    }

    println!("");

    let mut min_period = [0, 0, 0];

    for axis in 0..3 {
        let mut t = 1;
        loop {
            for i in 0..moons.len() {
                for j in i..moons.len() {
                    moons[i].vel[axis] += get_acc(moons[i].pos[axis], moons[j].pos[axis]);
                    moons[j].vel[axis] -= get_acc(moons[i].pos[axis], moons[j].pos[axis]);
                }
            }

            for m in moons.iter_mut() {
                m.pos[axis] += m.vel[axis];
            }

            if is_axis_similar(&moons, &initial, axis) {
                min_period[axis] = t;
                break;
            }

            t += 1;
        }
    }

    let gcd_xy = gcd(min_period[0], min_period[1]);
    let lcm_xy = (min_period[0] * min_period[1]) / gcd_xy;
    
    let gcd_xyz = gcd(lcm_xy, min_period[2]);
    let lcm_xyz = (lcm_xy * min_period[2]) / gcd_xyz;

    println!("{:?}", lcm_xyz);
    
}