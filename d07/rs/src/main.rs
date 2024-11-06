use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;
use std::time::Instant;

const FILE_PATH: &str = "../input.txt";
enum CardType {
    FiveOfaKind,
    FourOfaKind,
    FullHouse,
    ThreeOfaKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;
    let result_arr = parse_input(&content);

    let sum: u64 = result_arr
        .iter()
        .enumerate()
        .map(|(i, &x)| x * (i as u64 + 1))
        .sum();

    println!("Result: {:?}", sum);
    println!("Elasped time: {:?}", start.elapsed());
    Ok(())
}

fn parse_input(content: &str) -> Vec<u64> {
    let parsed_lines: Vec<(&str, u64)> = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let word = parts.next()?;
            let value = parts.next()?.parse::<u64>().ok()?;
            Some((word, value))
        })
        .collect();

    let mut result = vec![
        Vec::new(), // FiveOfaKind
        Vec::new(), // FourOfaKind
        Vec::new(), // FullHouse
        Vec::new(), // ThreeOfaKind
        Vec::new(), // TwoPair
        Vec::new(), // OnePair
        Vec::new(), // HighCard
    ];

    for line in &parsed_lines {
        let card_type = sort_card_type_part_two(&line.0);
        match card_type {
            CardType::FiveOfaKind => result[0].push(line),
            CardType::FourOfaKind => result[1].push(line),
            CardType::FullHouse => result[2].push(line),
            CardType::ThreeOfaKind => result[3].push(line),
            CardType::TwoPair => result[4].push(line),
            CardType::OnePair => result[5].push(line),
            CardType::HighCard => result[6].push(line),
        }
    }

    let mut sum_arr: Vec<u64> = Vec::new();

    for row in result.iter_mut() {
        if !row.is_empty() {
            row.sort_by(|&(word1, _), &(word2, _)| {
                for (c1, c2) in word1.chars().zip(word2.chars()) {
                    let strength1 = match_card_strength_part_two(c1);
                    let strength2 = match_card_strength_part_two(c2);
                    if strength1 != strength2 {
                        return strength2.cmp(&strength1);
                    }
                }
                std::cmp::Ordering::Equal
            });

            for card in row {
                sum_arr.push(card.1);
            }
        }
    }

    sum_arr.reverse();
    sum_arr
}

#[allow(dead_code)]
fn sort_card_type(word: &str) -> CardType {
    let mut char_frequencies = HashMap::new();
    for c in word.chars() {
        *char_frequencies.entry(c).or_insert(0) += 1;
    }

    let mut counts = char_frequencies.values().cloned().collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));

    match counts.as_slice() {
        [5] => CardType::FiveOfaKind,
        [4, ..] => CardType::FourOfaKind,
        [3, 2, ..] => CardType::FullHouse,
        [3, ..] => CardType::ThreeOfaKind,
        [2, 2, ..] => CardType::TwoPair,
        [2, ..] => CardType::OnePair,
        _ => CardType::HighCard,
    }
}

fn sort_card_type_part_two(word: &str) -> CardType {
    let mut char_frequencies = HashMap::new();
    for c in word.chars() {
        *char_frequencies.entry(c).or_insert(0) += 1;
    }

    let joker_count = char_frequencies.remove(&'J').unwrap_or(0);
    if joker_count == 5 {
        return CardType::FiveOfaKind;
    }

    let mut counts: Vec<u64> = char_frequencies.values().cloned().collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));

    if joker_count > 0 {
        counts[0] += joker_count;
    }

    match counts.as_slice() {
        [5] => CardType::FiveOfaKind,
        [4, ..] => CardType::FourOfaKind,
        [3, 2, ..] => CardType::FullHouse,
        [3, ..] => CardType::ThreeOfaKind,
        [2, 2, ..] => CardType::TwoPair,
        [2, ..] => CardType::OnePair,
        _ => CardType::HighCard,
    }
}

#[allow(dead_code)]
fn match_card_strength(c: char) -> u64 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        '9' => 7,
        '8' => 6,
        '7' => 5,
        '6' => 4,
        '5' => 3,
        '4' => 2,
        '3' => 1,
        '2' => 0,
        _ => 0,
    }
}

fn match_card_strength_part_two(c: char) -> u64 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 0,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}
