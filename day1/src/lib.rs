use std::fs::File;
use std::io::{BufReader, Lines, Result};
use std::path::Path;
use helpers;

const FILESTRING: &str = &"day1/src/resources/input.txt";

pub fn solve() {
    let path :&Path = Path::new(FILESTRING);
    let optional_lines: Result<Lines<BufReader<File>>> = helpers::read_file_lines(path);
    match optional_lines {
        Ok(lines) => {
            let initial_mass: u32 = 0;
            let result: u32 = lines
                .map(|string: Result<String>| string.unwrap().as_str().parse::<u32>().unwrap())
                .map(|mass: u32| convert_mass_to_fuel(mass)) // integer division rounds down automatically
                .fold(initial_mass, |accumulated_value: u32, new_element: u32| accumulated_value + new_element);
            println!("Result is {}", result);
        }
        Err(_) => {println!("Couldn't parse the file");}
    }
}

pub fn solve2() {
    let path :&Path = Path::new(FILESTRING);
    let optional_lines: Result<Lines<BufReader<File>>> = helpers::read_file_lines(path);
    match optional_lines {
        Ok(lines) => {
            let initial_mass: u32 = 0;
            let result: u32 = lines
                .map(|string: Result<String>| string.unwrap().as_str().parse::<u32>().unwrap())
                .map(|mass: u32| recursively_convert_mass_to_fuel(mass)) // integer division rounds down automatically
                .fold(initial_mass, |accumulated_value: u32, new_element: u32| accumulated_value + new_element);
            println!("Result is {}", result);
        }
        Err(_) => {println!("Couldn't parse the file");}
    }
}

fn convert_mass_to_fuel(mass: u32) -> u32 {
    (mass / 3) - 2
}

fn recursively_convert_mass_to_fuel(mass: u32) -> u32 {
    let mut mut_mass = mass;
    let mut sum: u32 = 0;
    // If mass is over 8, we must account for its fuel equivalent
    while mut_mass > 8 {
        mut_mass = (mut_mass / 3) - 2;
        sum += mut_mass;
    }
    sum
}

