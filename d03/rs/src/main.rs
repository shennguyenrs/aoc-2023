use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::Error;
use std::ops::{Range, RangeInclusive};
use std::str::from_utf8;
use std::time::Instant;

const FILE_PATH: &str = "../input.txt";

lazy_static! {
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
    static ref SYMBOL_PATTERN: Regex = Regex::new(r"[^.\d]").unwrap();
    static ref GEAR_PATTERN: Regex = Regex::new(r"\*+").unwrap();
    static ref NUMBER_ADJACENT_TO_GEAR_PATTERN: Regex =
        Regex::new(r"\d+\*\d+|\*\d+|\d+\*").unwrap();
}

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;
    // let lines: Vec<&str> = content.lines().collect();
    let lines: Vec<&[u8]> = content.lines().map(str::as_bytes).collect();

    let mut sum: u32 = lines
        .par_windows(3)
        .map(|w| match w {
            // [prev, current, next] => find_valid_numbers_part_one(Some(prev), current, Some(next)),
            [prev, current, next] => find_valid_numbers_part_two(Some(prev), current, Some(next)),
            _ => unreachable!(),
        })
        .sum();

    // Part one
    // Add first and last line seperately
    // sum += find_valid_numbers_part_one(None, lines.get(0).unwrap(), Some(lines.get(1).unwrap()))
    //     + find_valid_numbers_part_one(
    //         Some(lines.get(lines.len() - 2).unwrap()),
    //         lines.get(lines.len() - 1).unwrap(),
    //         None,
    //     );

    // Part two
    // Add first and last line seperately
    sum += find_valid_numbers_part_two(None, lines.get(0).unwrap(), Some(lines.get(1).unwrap()))
        + find_valid_numbers_part_two(
            Some(lines.get(lines.len() - 2).unwrap()),
            lines.get(lines.len() - 1).unwrap(),
            None,
        );

    println!("Sum is {}", sum);
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}

#[allow(dead_code)]
fn find_valid_numbers_part_one(
    prev_line: Option<&str>,
    line: &str,
    next_line: Option<&str>,
) -> u32 {
    // Find all numbers in current line and its positions
    let numbers_in_line: Vec<(u32, usize, usize)> = NUMBER_PATTERN
        .find_iter(line)
        .map(|c| (c.as_str().parse::<u32>().unwrap(), c.start(), c.end()))
        .collect();

    // Process to filter out numbers that do not adjacent to symbol
    // then sum up remain numbers
    numbers_in_line
        .into_iter()
        .filter(|&(_, start, end)| {
            // Create a range that start before the number one character
            // and after the number one character
            let range = start.saturating_sub(1)..=end.max(end + 1).min(line.len());
            // Check if the range have adjacent symbol on prev, current and next line
            has_adjacent_symbol(prev_line, range.clone())
                || has_adjacent_symbol(Some(line), range.clone())
                || has_adjacent_symbol(next_line, range)
        })
        .map(|(i, _, _)| i)
        .sum()
}

fn has_adjacent_symbol(line: Option<&str>, range: RangeInclusive<usize>) -> bool {
    line.map_or(false, |l| {
        let slice = &l[*range.start()..*range.end()];
        SYMBOL_PATTERN.is_match(slice)
    })
}

// Old version
// fn find_valid_numbers_part_two(
//     prev_line: Option<&str>,
//     line: &str,
//     next_line: Option<&str>,
// ) -> u32 {
//     let gears_in_line: Vec<(usize, usize)> = GEAR_PATTERN
//         .find_iter(line)
//         .map(|c| (c.start(), c.end()))
//         .collect();
//
//     gears_in_line
//         .into_iter()
//         .filter_map(|(start, end)| {
//             let range = start.saturating_sub(1)..end.max(end + 1).min(line.len());
//             let mut all_numbers = vec![];
//
//             // Check ajacent numbers in current line
//             all_numbers.extend(has_adjacent_numbers(
//                 Some(line),
//                 &line[range.clone()],
//                 range.clone(),
//             ));
//
//             // Check ajacent numbers in prev line
//             if let Some(prev) = prev_line {
//                 all_numbers.extend(has_adjacent_numbers(
//                     Some(prev),
//                     &prev[range.clone()],
//                     range.clone(),
//                 ));
//             }
//
//             // Check ajacent number in next line
//             if let Some(next) = next_line {
//                 all_numbers.extend(has_adjacent_numbers(
//                     Some(next),
//                     &next[range.clone()],
//                     range.clone(),
//                 ));
//             }
//
//             if all_numbers.len() % 2 == 0 {
//                 Some(all_numbers.iter().product::<u32>())
//             } else {
//                 None
//             }
//         })
//         .map(|c| c)
//         .sum::<u32>()
// }

// fn has_adjacent_numbers(line: Option<&str>, slice: &str, range: Range<usize>) -> Vec<u32> {
//     if NUMBER_PATTERN.is_match(slice) {
//         return NUMBER_PATTERN
//             .find_iter(line.unwrap())
//             .filter_map(|c| {
//                 let found_range = c.start()..c.end();
//                 if range_intersect(found_range, range.clone()) {
//                     return c.as_str().parse::<u32>().ok();
//                 }
//                 None
//             })
//             .collect::<Vec<u32>>();
//     }
//     vec![]
// }

// fn range_intersect(range1: Range<usize>, range2: Range<usize>) -> bool {
//     range1.start < range2.end && range1.end > range2.start
// }

// New refactored version
fn find_valid_numbers_part_two(
    prev_line: Option<&[u8]>,
    line: &[u8],
    next_line: Option<&[u8]>,
) -> u32 {
    line.iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'*')
        .filter_map(|(i, _)| {
            let range = i.saturating_sub(1)..line.len().min(i + 2);
            let mut numbers = HashSet::new();

            for l in [prev_line, Some(line), next_line].iter().flatten() {
                numbers.extend(adjacent_numbers(l, &range));
            }

            if numbers.len() % 2 == 0 {
                Some(numbers.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

fn adjacent_numbers<'a>(line: &'a [u8], range: &'a Range<usize>) -> impl Iterator<Item = u32> + 'a {
    line[range.clone()]
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c.is_ascii_digit())
        .map(move |(i, _)| {
            let start = line[..range.start + i]
                .iter()
                .rposition(|&c| !c.is_ascii_digit())
                .map_or(0, |p| p + 1);
            let end = line[range.start + i..]
                .iter()
                .position(|&c| !c.is_ascii_digit())
                .map_or(line.len(), |p| range.start + i + p);
            from_utf8(&line[start..end])
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
}
