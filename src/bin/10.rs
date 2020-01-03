use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops;
use std::path::PathBuf;

#[derive(Eq, Debug, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn is_aligned(&self, other: &Coord) -> bool {
        let y1 = other.x * self.y;
        let y2 = self.x * other.y;

        y1 == y2
    }

    fn length2(&self) -> usize {
        (self.x * self.x + self.y * self.y) as usize
    }

    fn reduced(&self) -> Coord {
        let mut a = self.x.abs();
        let mut b = self.y.abs();
        let mut t;

        loop {
            if b == 0 {
                return Coord {
                    x: self.x / a,
                    y: self.y / a,
                };
            }

            t = b;
            b = ((a % b) + b) % b;
            a = t;
        }
    }

    fn azimuth(&self) -> f32 {
        (self.y as f32).atan2(self.x as f32)
    }
}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Coord {
    fn eq(&self, r: &Coord) -> bool {
        self.x == r.x && self.y == r.y
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, r: Coord) -> Coord {
        Coord {
            x: self.x + r.x,
            y: self.y + r.y,
        }
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, r: Coord) -> Coord {
        Coord {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }
}

fn is_same<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/10_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let row_len = 20;

    let asteroids = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c == '#' {
                Some(Coord {
                    x: (i % (row_len + 1)) as isize,
                    y: (i / (row_len + 1)) as isize,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut max_view = HashMap::new();
    let mut max_asteroid = Coord { x: 0, y: 0 };

    for center in &asteroids {
        let mut set: HashMap<Coord, Vec<Coord>> = HashMap::new();
        for other in &asteroids {
            if is_same(center, other) {
                continue;
            }

            let other_relative_position = *other - *center;
            let reduced = other_relative_position.reduced();

            if set.contains_key(&reduced) {
                set.get_mut(&reduced).unwrap().push(other_relative_position);
            } else {
                set.insert(reduced, vec![other_relative_position]);
            }
        }

        if set.len() > max_view.len() {
            max_view = set;
            max_asteroid = *center;
        }
    }

    println!("Max visible: {:?}", max_view.len());
    println!("At location: {:?}", max_asteroid);

    max_view.iter_mut().for_each(|(dir, list)| {
        list.sort_by(|a, b| a.length2().partial_cmp(&b.length2()).unwrap())
    });

    let pi = std::f32::consts::PI;

    let mut azimuth_asteroids = max_view
        .iter()
        .map(|(dir, list)| {
            (
                (((dir.azimuth() / pi - 1.5) % 2.0) + 2.0) % 2.0,
                list.clone().into_iter().collect::<VecDeque<Coord>>(),
            )
        })
        .collect::<Vec<(f32, VecDeque<Coord>)>>();

    azimuth_asteroids.sort_by(|l, r| l.0.partial_cmp(&r.0).unwrap());

    let mut cntr = 1;
    {
        for i in 0..azimuth_asteroids.len() {
            let list = &azimuth_asteroids[i].1;
            if list.len() > 0 {
                println!(
                    "cntr: {} pos: {:?}",
                    cntr,
                    max_asteroid + azimuth_asteroids[i].1[0]
                );
                azimuth_asteroids.get_mut(i).unwrap().1.pop_front();
                cntr += 1;
            }
        }
    }
}
