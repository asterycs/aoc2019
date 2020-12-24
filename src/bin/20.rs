use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::{Cell,RefCell};
use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::Hash;

type Map = Vec<Vec<Tile>>;

fn build_map(input: &String) -> Map {
    let mut map = Map::new();

    let mut portals_coordinates: Vec<(Rc<RefCell<String>>, Vec<Vec2u>)> = Vec::new();
    let mut coordinates_portals = HashMap::new();

    for (row, line) in input.split('\n').enumerate() {
        for (column, char) in line.chars().enumerate() {
            match char {
                'A'..='Z' => {
                    let coord = Vec2u{r: row, c: column};
                    
                    // TODO: Use indices for referring to the already existings portals instead?
                    if let Some(portal) = coordinates_portals.get(&coord) {
                        let old_portal_index = portals_coordinates.iter().position(|x| Rc::ptr_eq(&x.0, &portal)).unwrap();
                        let (old_portal, old_coordinates) = portals_coordinates.get_mut(old_portal_index).unwrap();

                        old_portal.borrow_mut().push(char);
                        old_coordinates.push(coord);
                    } else {
                        let portal = Rc::new(RefCell::new(char.to_string()));
                        portals_coordinates.push((portal.clone(), vec![coord]));

                        coordinates_portals.insert(coord, Rc::new(RefCell::new(char.to_string())));

                        let neighbours = coord.get_neighbors();

                        for neighbor in neighbours.iter() {
                            coordinates_portals.insert(neighbor.clone(), portal.clone());
                        }
                    }
                },
                _ => (),
            }
        }
    }

    println!("p_c {:?}", portals_coordinates);
    println!("c_p {:?}", coordinates_portals);

    map
}

fn get_input() -> String {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/20.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    input
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Vec2u {
    r: usize,
    c: usize,
}

impl Vec2u {
    fn get_neighbors(&self) -> Vec<Vec2u> {
        let mut neighbours = Vec::new();

        if self.r > 0 {
            neighbours.push(Vec2u{r: self.r - 1, c: self.c});
        }

        if self.c > 0 {
            neighbours.push(Vec2u{r: self.r, c: self.c - 1});
        }

        neighbours.push(Vec2u{r: self.r + 1, c: self.c});
        neighbours.push(Vec2u{r: self.r, c: self.c + 1});

        neighbours
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Wall,
    Portal(String)
}

fn part1(input: String) -> u32 {
    
    let map = build_map(&input);

    0
}

fn main() {
    let input = get_input();

    let steps = part1(input.clone());
    println!("part 1 steps: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test0() {
        let input = r#"
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z           
            "#.to_owned();

        let steps = part1(input);

        assert_eq!(steps, 8);
    }
}
