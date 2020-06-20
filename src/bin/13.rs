use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Ball,
    Paddle,
    Wall,
    Block,
}

#[derive(Debug, Hash, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl PartialEq for Coord {
    fn eq(&self, r: &Coord) -> bool {
        self.x == r.x && self.y == r.y
    }
}

impl From<isize> for Tile {
    fn from(x: isize) -> Self {
        match x {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile"),
        }
    }
}

impl Into<&str> for &Tile {
    fn into(self) -> &'static str {
        match self {
            Tile::Empty => " ",
            Tile::Wall => "w",
            Tile::Block => "#",
            Tile::Paddle => "-",
            Tile::Ball => "o",
        }
    }
}

struct GameState {
    screen: HashMap<Coord, Tile>,
    score: usize,
    paddle_pos: Coord,
    ball_pos: Coord,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            screen: HashMap::new(),
            score: 0,
            paddle_pos: Coord { x: 0, y: 0 },
            ball_pos: Coord { x: 0, y: 0 },
        }
    }
}

fn update_game_state(output: &VecDeque<isize>, game_state: &mut GameState) {
    for c in output.clone().into_iter().collect::<Vec<_>>().chunks(3) {
        let x = c[0];
        let y = c[1];
        let t = c[2];

        if x == -1 && y == 0 {
            game_state.score = t as usize;
            continue;
        }

        let x = x as usize;
        let y = y as usize;
        let t = Tile::from(t);

        match t {
            Tile::Ball => game_state.ball_pos = Coord { x, y },
            Tile::Paddle => game_state.paddle_pos = Coord { x, y },
            _ => (),
        }

        game_state.screen.insert(Coord { x, y }, t);
    }
}

fn draw_game(game: &GameState) {
    let max_x = game.screen.keys().max_by_key(|c| c.x).unwrap().x;
    let max_y = game.screen.keys().max_by_key(|c| c.y).unwrap().y;

    let mut to_draw = String::new();
    let mut block_cntr = 0;

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let next = game.screen.get(&Coord { x, y }).unwrap();
            to_draw += next.into();

            if let Tile::Block = next {
                block_cntr += 1;
            }
        }

        to_draw += "\n";
    }

    print!("{}", to_draw);

    println!("Score: {}", game.score);
    println!("Blocks left: {}", block_cntr);
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/13_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    // Part 1
    let input_queue = &mut VecDeque::new();
    let output_queue = &mut VecDeque::new();

    let mut state = ProgramState::new(program);

    run(&mut state, &mut *input_queue, &mut *output_queue);

    let mut game_state = GameState::new();
    update_game_state(&output_queue, &mut game_state);
    draw_game(&game_state);

    // Part 2
    program[0] = 2;
    let mut state = ProgramState::new(program);

    let input_queue = &mut vec![].into_iter().collect();

    let mut game_state = GameState::new();

    loop {
        let output_queue = &mut VecDeque::new();

        run(&mut state, &mut *input_queue, &mut *output_queue);
        update_game_state(&output_queue, &mut game_state);
        draw_game(&game_state);

        if state.status == ExecutionStatus::Waiting {
            let input = match game_state.paddle_pos.x.cmp(&game_state.ball_pos.x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            input_queue.push_back(input);
        } else {
            break;
        }
    }
}
