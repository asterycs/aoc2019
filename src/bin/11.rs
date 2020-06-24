use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;


struct RobotState {
    x: isize,
    y: isize,
    dir: isize,
}

impl RobotState {
    fn turn(&mut self, dir: isize) {
        let dir = if dir == 1 { 1 }else{ -1 };

        self.dir += dir;
        self.dir = self.dir.rem_euclid(4);
    }

    fn move_forward(&mut self) {
        let dx;
        let dy;

        match self.dir {
            0 => { dx =  0; dy =  1; },
            1 => { dx =  1; dy =  0; },
            2 => { dx =  0; dy = -1; },
            3 => { dx = -1; dy =  0; },
            _ => unreachable!("Invalid move dir"),
        }

        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

impl From<isize> for Color {
    fn from(i: isize) -> Color {
        match i {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unknown color"),
        }
    }
}

impl Into<isize> for &Color {
    fn into(self) -> isize {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

fn draw(map: &HashMap<(isize,isize), Color>) {
    let mut min_x = std::isize::MAX;
    let mut min_y = std::isize::MAX;

    let mut max_x = std::isize::MIN;
    let mut max_y = std::isize::MIN;

    for (k,_) in map {
        min_x = std::cmp::min(min_x, k.0);
        min_y = std::cmp::min(min_y, k.1);

        max_x = std::cmp::max(max_x, k.0);
        max_y = std::cmp::max(max_y, k.1);
    }
    
    for r in (min_y..max_y+1).rev() {
        for c in min_x..max_x+1 {
            let col = map.get(&(c,r)).unwrap_or(&Color::Black);
            let sign;

            match col {
                Color::Black => sign = '.',
                Color::White => sign = '#',
            }

            print!("{}", sign);
        }
        println!();
    }


}

fn run_painter(init_tile_color: isize, program: &Vec<isize>) -> HashMap<(isize, isize), Color> {
    let input_queue: &mut VecDeque<isize> = &mut vec![init_tile_color].into_iter().collect();
    let output_queue: &mut VecDeque<isize> = &mut VecDeque::new();

    let mut program_state = IntcodeVM::new(&program);
    let mut robot_state = RobotState { x: 0, y: 0, dir: 0 };
    let mut hull: HashMap<(isize, isize), Color> = HashMap::new();

    loop {
        let result = run(&mut program_state, &mut *input_queue, &mut *output_queue);

        let color = Color::from(output_queue.pop_front().unwrap());
        let direction = output_queue.pop_front().unwrap();

        hull.insert((robot_state.x, robot_state.y), color);

        robot_state.turn(direction);
        robot_state.move_forward();

        input_queue.push_back(hull.get(&(robot_state.x, robot_state.y)).unwrap_or(&Color::Black).into());

        if let Err(ExecutionError::Halted) = result {
            break;
        };
    }

    hull
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/11_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    
    let hull = run_painter(0, &program);
    println!("printed panels: {}", hull.len());

    let hull = run_painter(1, &program);
    draw(&hull);
}
