use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::Error;
use std::time::Instant;

const FILE_PATH: &str = "../input.txt";

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;
    // let sum: u32 = content.lines().map(|l| find_answer_part_one(l)).sum();
    // println!("Total: {:?}", sum);

    let mut dict = HashMap::new();
    for line in content.lines() {
        find_answer_part_two(&mut dict, line);
    }
    println!("Total: {:?}", dict.values().sum::<u32>());

    println!("Elapsed time: {:?}", start.elapsed());
    Ok(())
}

// Part one
// fn find_answer_part_one(line: &str) -> u32 {
//     let (_, numbers): (&str, &str) = line.split_once(':').unwrap();
//     let (winning_numbers_str, my_numbers_str): (&str, &str) = numbers.split_once('|').unwrap();
//     let winning_numbers = winning_numbers_str
//         .split_whitespace()
//         .map(|s| s.parse::<u32>().unwrap())
//         .collect::<HashSet<u32>>();
//     let my_numbers = my_numbers_str
//         .split_whitespace()
//         .map(|s| s.parse::<u32>().unwrap())
//         .collect::<HashSet<u32>>();
//
//     let intersection_count = winning_numbers.intersection(&my_numbers).count();
//
//     if intersection_count > 0 {
//         1 << (intersection_count - 1)
//     } else {
//         0
//     }
// }

// Part two
fn find_answer_part_two(dict: &mut HashMap<u32, u32>, line: &str) {
    let (game_with_id, numbers): (&str, &str) = line.split_once(':').unwrap();
    let id: u32 = game_with_id
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let (winning_numbers_str, my_numbers_str): (&str, &str) = numbers.split_once('|').unwrap();
    let winning_numbers = winning_numbers_str
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    let my_numbers = my_numbers_str
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();

    let intersection_count = winning_numbers.intersection(&my_numbers).count() as u32;

    // Add current card id to dict if it was not in there before
    // else increase one value to it current value
    let current_card_value = dict.entry(id).and_modify(|v| *v += 1).or_insert(1);

    // Add copy for forward card if there is interection in the current game
    if intersection_count > 0 {
        let copies_to_add = *current_card_value;
        for i in 1..=intersection_count {
            let next_id = id + i;
            let next_card_count = dict.entry(next_id).or_insert(0);
            *next_card_count += copies_to_add;
        }
    }
}
