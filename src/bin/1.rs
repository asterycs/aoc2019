use std::env;
use std::fs;
use std::path::PathBuf;

fn get_fuel1(masses: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for mass in masses.iter() {
        sum += *mass / 3 - 2;
    }

    sum
}

fn get_fuel2_helper(mass: i32) -> i32{
    let fuel = std::cmp::max(mass / 3 - 2, 0);

    if fuel > 0 {
        return fuel + get_fuel2_helper(fuel);
    } else {
        return fuel;
    }
}

fn get_fuel2(masses: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for mass in masses.iter() {
        sum += get_fuel2_helper(*mass);
    }

    sum
}

fn main() {
    let filename = &mut PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("inputs/1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let input = input
    .split("\n")
    .map(|x| x.parse::<i32>().unwrap())
    .collect::<Vec<_>>();

    let sum = get_fuel1(&input);

    println!("Part 1: {}", sum);

    let sum = get_fuel2(&input);

    println!("Part 2: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1_input0() -> Vec<i32> {
        vec![12]
    }

    #[test]
    fn part1_test0() {
        let test_input = part1_input0();

        assert_eq!(get_fuel1(&test_input), 2);
    }

    fn part1_input1() -> Vec<i32> {
        vec![14]
    }

    #[test]
    fn part1_test1() {
        let test_input = part1_input1();

        assert_eq!(get_fuel1(&test_input), 2);
    }

    fn part1_input2() -> Vec<i32> {
        vec![1969]
    }

    #[test]
    fn part1_test2() {
        let test_input = part1_input2();

        assert_eq!(get_fuel1(&test_input), 654);
    }

    fn part1_input3() -> Vec<i32> {
        vec![100756]
    }

    #[test]
    fn part1_test3() {
        let test_input = part1_input3();

        assert_eq!(get_fuel1(&test_input), 33583);
    }

    fn part2_input0() -> Vec<i32> {
        vec![14]
    }

    #[test]
    fn part2_test0() {
        let test_input = part2_input0();

        assert_eq!(get_fuel2(&test_input), 2);
    }

    fn part2_input1() -> Vec<i32> {
        vec![1969]
    }

    #[test]
    fn part2_test1() {
        let test_input = part2_input1();

        assert_eq!(get_fuel2(&test_input), 966);
    }

    fn part2_input2() -> Vec<i32> {
        vec![100756]
    }

    #[test]
    fn part2_test2() {
        let test_input = part2_input2();

        assert_eq!(get_fuel2(&test_input), 50346);
    }
}
