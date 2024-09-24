use std::fs::read_to_string;
use std::io::Error;
use std::iter::once;
use std::time::Instant;

const FILE_PATH: &str = "../input.txt";

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;
    // let results = parse_input_part_one(&content)
    //     .map(find_result_part_one)
    //     .product::<u64>();
    let results = parse_input_part_two(&content)
        .map(find_result_part_two)
        .product::<u64>();

    println!("Result: {}", results);
    println!("Elasped time: {:?}", start.elapsed());

    Ok(())
}

#[allow(dead_code)]
fn parse_input_part_one(content: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    let mut lines = content.lines();
    let times = lines.next().unwrap_or("").split_whitespace().skip(1);
    let distances = lines.next().unwrap_or("").split_whitespace().skip(1);
    times
        .zip(distances)
        .filter_map(|(a, b)| Some((a.parse::<u64>().ok()?, b.parse::<u64>().ok()?)))
}

#[allow(dead_code)]
fn parse_input_part_two(content: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    let mut lines = content.lines();
    let parse_line = |line: &str| {
        line.split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .unwrap_or(0)
    };
    let times = lines.next().map(parse_line).unwrap_or(0);
    let distances = lines.next().map(parse_line).unwrap_or(0);
    once((times, distances))
}

// Brute force method
#[allow(dead_code)]
fn find_result_part_one(round_detail: (u64, u64)) -> u64 {
    (1..round_detail.0)
        .filter(|i| i * (round_detail.0 - i) > round_detail.1)
        .count() as u64
}

// Using quadratic equation method: Solving t*(T-t) > D or -x^2 + ax - b > 0
fn find_result_part_two(round_detail: (u64, u64)) -> u64 {
    let a = round_detail.0 as f64;
    let b = round_detail.1 as f64;
    let x1 = (a - (a * a - 4.0 * b).sqrt()) / 2.0;
    let x2 = (a + (a * a - 4.0 * b).sqrt()) / 2.0;
    (x2.ceil() - x1.floor() - 1.0) as u64
}
