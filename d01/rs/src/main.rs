use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

static FILE_PATH: &str = "../input.txt";
static WORD_NUM: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() -> Result<(), Error> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    let total: u32 = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| extract_number_part_one(&line))
        // .map(|line| extract_number_part_two(&line))
        .sum();

    println!("{}", total);

    Ok(())
}

#[allow(dead_code)]
fn extract_number_part_one(line: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;

    for c in line.chars() {
        if let Some(d) = c.to_digit(10) {
            last_digit = Some(d);
            if first_digit.is_none() {
                first_digit = Some(d);
            }
        }
    }

    first_digit.unwrap_or(0) * 10 + last_digit.unwrap_or(0)
}

#[allow(dead_code)]
fn extract_number_part_two(line: &str) -> u32 {
    let word_map: HashMap<&str, u32> = WORD_NUM.iter().cloned().collect();
    let (first_digit, last_digit) = find_digits(line, &word_map);
    first_digit * 10 + last_digit
}

#[allow(dead_code)]
fn find_digits(line: &str, word_map: &HashMap<&str, u32>) -> (u32, u32) {
    let mut first = None;
    let mut last = None;

    for (i, c) in line.char_indices() {
        let digit = if let Some(d) = c.to_digit(10) {
            Some(d)
        } else {
            word_map
                .iter()
                .find(|(&word, _)| line[i..].starts_with(word))
                .map(|(_, &value)| value)
        };

        if let Some(d) = digit {
            last = Some(d);
            if first.is_none() {
                first = Some(d);
            }
        }
    }

    (first.unwrap_or(0), last.unwrap_or(0))
}
