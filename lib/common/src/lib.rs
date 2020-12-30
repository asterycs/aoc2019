use std::{env, num::ParseIntError};
use std::fs;
use std::path::PathBuf;

pub fn get_input(input_file: &str) -> String {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push( "inputs/".to_owned() + input_file);

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    input
}

pub fn to_intcode(input: String) -> Result<Vec<isize>, ParseIntError> {
    input
        .split(",")
        .map(|x| x.parse::<isize>())
        .collect()
}

#[macro_export]
macro_rules! intcode_task {
    ($input_file:expr, $func_name1:ident $(, $func_name2:ident)*) => {
        fn main() {
            let input = get_input(stringify!($input_file));
            let program = to_intcode(input);

            let program = program.expect("Invalid intcode program");

            println!("{} returned: {:?}", stringify!($func_name1), $func_name1(&program));

            $(
                println!("{} returned: {:?}", stringify!($func_name2), $func_name2(&program));
            )*
        }
    };
}

#[macro_export]
macro_rules! task {
    ($input_file:expr, $func_name1:ident $(, $func_name2:ident)*) => {
        fn main() {
            let input = get_input(stringify!($input_file));

            println!("{} returned: {:?}", stringify!($func_name1), $func_name1(&input));

            $(
                println!("{} returned: {:?}", stringify!($func_name2), $func_name2(&input));
            )*
        }
    };
}