use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

const FILE_PATH: &str = "../sample.txt";
// const ALLOW_RED: u32 = 12;
// const ALLOW_GREEN: u32 = 13;
// const ALLOW_BLUE: u32 = 14;

lazy_static! {
    static ref RED_REGEX: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref GREEN_REGEX: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref BLUE_REGEX: Regex = Regex::new(r"(\d+) blue").unwrap();
}

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let total: u32 = reader
        .lines()
        // .filter_map(|l| l.ok().map(|l| find_game_part_one(&l)))
        .filter_map(|l| l.ok().map(|l| find_fewest_cubes_part_two(&l)))
        .sum();

    println!("Total: {}", total);
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}

// fn find_game_part_one(line: &str) -> u32 {
//     let (game, sets): (&str, &str) = line.split_once(":").unwrap();
//     let game_id: u32 = game
//         .split_whitespace()
//         .nth(1)
//         .unwrap()
//         .parse::<u32>()
//         .unwrap();
//
//     for set in sets.split(";") {
//         if parse_color(set, &RED_REGEX) > ALLOW_RED {
//             return 0;
//         }
//
//         if parse_color(set, &GREEN_REGEX) > ALLOW_GREEN {
//             return 0;
//         };
//
//         if parse_color(set, &BLUE_REGEX) > ALLOW_BLUE {
//             return 0;
//         };
//     }
//
//     game_id
// }

fn parse_color(line: &str, color_regex: &Regex) -> u32 {
    color_regex
        .captures(line)
        .map(|c| c[1].parse::<u32>().unwrap_or(0))
        .unwrap_or(0)
}

fn find_fewest_cubes_part_two(line: &str) -> u32 {
    let (_, sets): (&str, &str) = line.split_once(":").unwrap();
    let mut greatest_red_cube: u32 = 0;
    let mut greatest_green_cube: u32 = 0;
    let mut greatest_blue_cube: u32 = 0;

    for set in sets.split(";") {
        let red = parse_color(set, &RED_REGEX);
        let green = parse_color(set, &GREEN_REGEX);
        let blue = parse_color(set, &BLUE_REGEX);

        if red > greatest_red_cube {
            greatest_red_cube = red;
        }

        if green > greatest_green_cube {
            greatest_green_cube = green;
        }

        if blue > greatest_blue_cube {
            greatest_blue_cube = blue;
        }
    }

    greatest_red_cube * greatest_green_cube * greatest_blue_cube
}
