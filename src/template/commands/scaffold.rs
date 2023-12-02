use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

use crate::Day;

const MODULE_TEMPLATE: &str = r#"use anyhow::{Context, Result};
advent_of_code::solution!(DAY_NUMBER);

pub fn part_one(input: &str) -> Result<u32> {
    todo!()
}

pub fn part_two(input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            true,
            result.is_ok(),
            "part one failed: {}",
            result.err().unwrap()
        );
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            true,
            result.is_ok(),
            "part two failed: {}",
            result.err().unwrap()
        );
        assert_eq!(result.unwrap(), 0);
    }
}
"#;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

pub fn handle(day: Day) {
    let input_path = format!("data/inputs/{day}.txt");
    let example_path = format!("data/examples/{day}.txt");
    let module_path = format!("src/bin/{day}.rs");

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("DAY_NUMBER", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {e}");
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(1);
        }
    }

    println!("---");
    println!("🎄 Type `cargo solve {}` to run your solution.", day);
}
