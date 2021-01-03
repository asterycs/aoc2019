use std::collections::{VecDeque, HashSet};

use common::*;

type Map = [[Tile; 5]; 5];
type RecursiveMap = VecDeque<Map>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Bug,
    Empty
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Bug,
            '.' => Tile::Empty,
            _ => panic!("Unknown tile"),
        }
    }
}

fn get_neighbours(map: &Map, r: usize, c: usize, check_center: bool) -> i32 {
    let mut neighbours = 0;

    if c > 0 {
        if let Tile::Bug = map[r][c - 1] {
            neighbours += 1;
        }
    }
    if r > 0 {
        if let Tile::Bug = map[r - 1][c] {
            neighbours += 1;
        }
    }
    if c + 1 < map[r].len() {
        if let Tile::Bug = map[r][c + 1] {
            neighbours += 1;
        }
    }
    if r + 1 < map.len() {
        if let Tile::Bug = map[r + 1][c] {
            neighbours += 1;
        }
    }

    if !check_center {
        if ((r as isize - 2).abs() == 1 && c == 2) || ((c as isize - 2).abs() == 1 && r == 2) {
            if let Tile::Bug = map[2][2] {
                neighbours -= 1;
            }
        }
    }

    neighbours
}

fn advance(map: &Map) -> Map {
    let mut new_map = *map;

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let neighbours = get_neighbours(map, r, c, true);

            if new_map[r][c] == Tile::Bug && neighbours != 1 {
                new_map[r][c] = Tile::Empty;
            } else if new_map[r][c] == Tile::Empty && (neighbours == 1 || neighbours == 2) {
                new_map[r][c] = Tile::Bug;
            }
        }
    }

    new_map
}

fn is_on_outer_northern_edge(row: usize, col: usize) -> bool {
    if row == 0 {
        return true;
    }

    return false;
}

fn check_outwards_north(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    if let Tile::Bug = map[level - 1][1][2] {
        return 1;
    }

    return 0;
}

fn is_on_outer_eastern_edge(row: usize, col: usize) -> bool {
    if col == 4 {
        return true;
    }

    return false;
}

fn check_outwards_east(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    if let Tile::Bug = map[level - 1][2][3] {
        return 1;
    }

    return 0;
}

fn is_on_outer_southern_edge(row: usize, col: usize) -> bool {
    if row == 4 {
        return true;
    }

    return false;
}

fn check_outwards_south(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    if let Tile::Bug = map[level - 1][3][2] {
        return 1;
    }

    return 0;
}

fn is_on_outer_western_edge(row: usize, col: usize) -> bool {
    if col == 0 {
        return true;
    }

    return false;
}

fn check_outwards_west(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    if let Tile::Bug = map[level - 1][2][1] {
        return 1;
    }

    return 0;
}

fn is_on_inner_northern_edge(row: usize, col: usize) -> bool {
    if row == 1 && col == 2 {
        return true;
    }

    return false;
}

fn check_inwards_north(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    let mut neighbours = 0;

    let level = &map[level + 1];
    level[0].iter().for_each(|t| if *t == Tile::Bug {
        neighbours += 1;
    });

    return neighbours;
}

fn is_on_inner_eastern_edge(row: usize, col: usize) -> bool {
    if row == 2 && col == 3 {
        return true;
    }

    return false;
}

fn check_inwards_east(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    let mut neighbours = 0;

    let level = &map[level + 1];
    for row in level.iter() {
        if Tile::Bug == *row.last().unwrap() {
            neighbours += 1;
        }
    }

    return neighbours;
}

fn is_on_inner_southern_edge(row: usize, col: usize) -> bool {
    if row == 3 && col == 2 {
        return true;
    }

    return false;
}

fn check_inwards_south(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    let mut neighbours = 0;

    let level = &map[level + 1];
    level.last().unwrap().iter().for_each(|t| if *t == Tile::Bug {
        neighbours += 1;
    });

    return neighbours;
}

fn is_on_inner_western_edge(row: usize, col: usize) -> bool {
    if row == 2 && col == 1 {
        return true;
    }

    return false;
}

fn check_inwards_west(map: &RecursiveMap, level: usize, row: usize, col: usize) -> i32 {
    let mut neighbours = 0;

    let level = &map[level + 1];
    for row in level.iter() {
        if Tile::Bug == *row.first().unwrap() {
            neighbours += 1;
        }
    }

    return neighbours;
}

fn advance_recursive(map: &RecursiveMap) -> RecursiveMap {
    let mut new_map = map.clone();

    for level in 0..map.len() {
        for r in 0..map[level].len() {
            for c in 0..map[level][r].len() {
                if r == 2 && c == 2 {
                    continue;
                }
                let mut neighbours = get_neighbours(&map[level], r, c, false);

                if is_on_outer_northern_edge(r, c) && level > 0 {
                    neighbours += check_outwards_north(map, level, r, c);
                }
                if is_on_outer_eastern_edge(r, c) && level > 0 {
                    neighbours += check_outwards_east(map, level, r, c);
                }
                if is_on_outer_southern_edge(r, c) && level > 0 {
                    neighbours += check_outwards_south(map, level, r, c);
                }
                if is_on_outer_western_edge(r, c) && level > 0 {
                    neighbours += check_outwards_west(map, level, r, c);
                }

                if is_on_inner_northern_edge(r, c) && level < map.len() - 1 {
                    neighbours += check_inwards_north(map, level, r, c);
                }
                if is_on_inner_eastern_edge(r, c) && level < map.len() - 1 {
                    neighbours += check_inwards_east(map, level, r, c);
                }
                if is_on_inner_southern_edge(r, c) && level < map.len() - 1 {
                    neighbours += check_inwards_south(map, level, r, c);
                }
                if is_on_inner_western_edge(r, c) && level < map.len() - 1 {
                    neighbours += check_inwards_west(map, level, r, c);
                }

                if new_map[level][r][c] == Tile::Bug && neighbours != 1 {
                    new_map[level][r][c] = Tile::Empty;
                } else if new_map[level][r][c] == Tile::Empty && (neighbours == 1 || neighbours == 2) {
                    new_map[level][r][c] = Tile::Bug;
                }
            }
        }
    }

    new_map
}

fn get_map(str: &String) -> Map {
    let m = str.split_whitespace().map(|row| row.chars().map(|c| Tile::from(c)).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    [[m[0][0], m[0][1], m[0][2], m[0][3], m[0][4]],
    [m[1][0], m[1][1], m[1][2], m[1][3], m[1][4]],
    [m[2][0], m[2][1], m[2][2], m[2][3], m[2][4]],
    [m[3][0], m[3][1], m[3][2], m[3][3], m[3][4]],
    [m[4][0], m[4][1], m[4][2], m[4][3], m[4][4]]]
}

fn part1(input: &String) -> Result<i64, ()> {
    let mut map = get_map(input);

    let mut seen_layouts = HashSet::new();

    loop {
        seen_layouts.insert(map);
        map = advance(&map);

        if seen_layouts.contains(&map) {
            let mut diversity = 0;
            for (row_index, row) in map.iter().enumerate() {
                for (col_index, col) in row.iter().enumerate() {
                    if *col == Tile::Bug {
                        diversity += 2i64.pow(row_index as u32 * 5 + col_index as u32);
                    }
                }
            }

            return Ok(diversity);
        }
    }

    
    Err(())
}

fn get_bugs(level: &Map) -> i32 {
    let mut bugs = 0;
    for row in level.iter() {
        for t in row.iter() {
            if *t == Tile::Bug {
                bugs += 1;
            }
        }
    }

    bugs
}

fn get_empty_level() -> Map {
    [[Tile::Empty; 5]; 5]
}

fn expand(map: &mut RecursiveMap) {
    let bugs = get_bugs(map.front().unwrap());

    if bugs > 0 {
        map.push_front(get_empty_level());
    }

    let bugs = get_bugs(map.back().unwrap());

    if bugs > 0 {
        map.push_back(get_empty_level());
    }
}

fn print(map: &RecursiveMap) {
    for (level_idx, level) in map.iter().enumerate() {
        println!("Level idx: {}", level_idx);
        for row in level.iter() {
            let mut row_str = String::new();
            for c in row.iter() {
                if *c == Tile::Empty {
                    row_str += ".";
                } else if *c == Tile::Bug {
                    row_str += "#";
                }
            }
            println!("{}", row_str);
        }
    }
}

fn part2(input: &String) -> Result<i32, ()> {
    let map = get_map(input);
    let mut map = vec![map].into_iter().collect();

    for _ in 0..200 {
        expand(&mut map);
        //print(&map);
        //println!();

        map = advance_recursive(&map);
    }

    let mut total_bugs = 0;
    for level in map.iter() {
        total_bugs += get_bugs(level);
    }
    
    Ok(total_bugs)
}

task!(24.txt, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"
....#
#..#.
#..##
..#..
#...."#.trim_start().to_string();

        let map = get_map(&input);
        let map = advance(&map);

        assert_eq!(map[0], [Tile::Bug,Tile::Empty,Tile::Empty,Tile::Bug,Tile::Empty]);
        assert_eq!(map[1], [Tile::Bug,Tile::Bug,Tile::Bug,Tile::Bug,Tile::Empty]);
        assert_eq!(map[2], [Tile::Bug,Tile::Bug,Tile::Bug,Tile::Empty,Tile::Bug]);
        assert_eq!(map[3], [Tile::Bug,Tile::Bug,Tile::Empty,Tile::Bug,Tile::Bug]);
        assert_eq!(map[4], [Tile::Empty,Tile::Bug,Tile::Bug,Tile::Empty,Tile::Empty]);
    }

    #[test]
    fn part2_example() {
        let input = r#"
....#
#..#.
#..##
..#..
#...."#.trim_start().to_string();


        let map = get_map(&input);
        let mut map = vec![map].into_iter().collect();

        for _ in 0..10 {
            expand(&mut map);
            print(&map);
            println!();

            map = advance_recursive(&map);
        }

        let mut total_bugs = 0;
        for level in map.iter() {
            total_bugs += get_bugs(level);
        }

        assert_eq!(total_bugs, 99);
    }

}
