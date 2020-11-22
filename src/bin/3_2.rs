use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    dist: isize,
}

impl Move {
    pub fn get_offset(&self) -> (isize, isize) {
        match self.dir {
            Direction::Up => (0, self.dist),
            Direction::Down => (0, -self.dist),
            Direction::Left => (-self.dist, 0),
            Direction::Right => (self.dist, 0),
        }
    }
}

impl From<&str> for Move {
    fn from(input: &str) -> Move {
        let dir;

        match input.chars().next() {
            Some('U') => dir = Direction::Up,
            Some('D') => dir = Direction::Down,
            Some('L') => dir = Direction::Left,
            Some('R') => dir = Direction::Right,
            _ => panic!(),
        }

        let dist = input
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        Move { dir, dist }
    }
}

#[derive(Debug, Copy, Clone)]
struct LineSegment {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl LineSegment {
    fn advance(&mut self, x: isize, y: isize) {
        self.x1 = self.x2;
        self.y1 = self.y2;

        self.x2 += x;
        self.y2 += y;
    }
}

fn intersects(a: &LineSegment, b: &LineSegment) -> Option<(isize, isize)> {
    let t_den = (a.x1 - b.x1) * (b.y1 - b.y2) - (a.y1 - b.y1) * (b.x1 - b.x2);
    let u_den = (a.x1 - a.x2) * (a.y1 - b.y1) - (a.y1 - a.y2) * (a.x1 - b.x1);
    let det = (a.x1 - a.x2) * (b.y1 - b.y2) - (a.y1 - a.y2) * (b.x1 - b.x2);

    if det == 0 {
        return None;
    }

    let t = t_den as f32 / det as f32;
    let u = u_den as f32 / det as f32;

    if t >= 0.0 && t <= 1.0 && -u >= 0.0 && -u <= 1.0 {
        Some((
            (a.x1 as f32 + t * (a.x2 as f32 - a.x1 as f32)) as isize,
            (a.y1 as f32 + t * (a.y2 as f32 - a.y1 as f32)) as isize,
        ))
    } else {
        None
    }
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/3.txt");

    println!("Reading {}", filename.display());

    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);

    let moves: Vec<Vec<Move>> = f
        .lines()
        .map(|x| x.unwrap().split(",").map(|y| Move::from(y)).collect())
        .collect();

    let segments = &mut Vec::new();
    let steps = &mut Vec::new();
    let mut min_intersection_steps = std::isize::MAX;

    {
        let segment = &mut LineSegment {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };

        let mut step = 0;

        for i in 0..moves[0].len() {
            let offset = moves[0][i].get_offset();
            segment.advance(offset.0, offset.1);

            segments.push(*segment);
            steps.push(step);

            step += offset.0.abs() + offset.1.abs();
        }
    }

    {
        let other_segment = &mut LineSegment {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };

        let mut other_step = 0;

        for i in 0..moves[1].len() {
            let offset = moves[1][i].get_offset();

            other_segment.advance(offset.0, offset.1);

            for (j, s) in segments.iter().enumerate() {
                if let Some(c) = intersects(s, other_segment) {
                    if c == (0, 0) {
                        continue;
                    }

                    let offset = (c.0 - s.x1).abs()
                        + (c.1 - s.y1).abs()
                        + (c.0 - other_segment.x1).abs()
                        + (c.1 - other_segment.y1).abs();

                    min_intersection_steps =
                        std::cmp::min(min_intersection_steps, steps[j] + other_step + offset);
                }
            }

            other_step += offset.0.abs() + offset.1.abs();
        }
    }

    println!("dist: {}", min_intersection_steps);
}
