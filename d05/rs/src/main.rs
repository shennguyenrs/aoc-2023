use std::fs::read_to_string;
use std::io::Error;
use std::time::Instant;

const FILE_PATH: &str = "../sample.txt";

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let content = read_to_string(FILE_PATH)?;

    // let result = find_destination_part_one(&content);
    // println!("Result: {:?}", result);
    let result = find_destination_part_two(&content);
    println!("Result: {:?}", result);

    println!("Elapsed time: {:?}", start.elapsed());

    Ok(())
}

fn parse_maps(raw_map_parts: &[&str]) -> Vec<Vec<(i64, i64, i64)>> {
    raw_map_parts
        .iter()
        .skip(1)
        .map(|&category| {
            category
                .lines()
                .skip(1)
                .filter_map(|line| {
                    let nums = line
                        .split_whitespace()
                        .filter_map(|s| s.parse::<i64>().ok())
                        .collect::<Vec<i64>>();
                    if nums.len() == 3 {
                        Some((nums[0], nums[1], nums[2]))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

// fn find_destination_part_one(content: &str) -> i64 {
//     let parts = content
//         .split("\n\n")
//         .map(str::trim_end)
//         .collect::<Vec<&str>>();
//     let category_maps = parse_maps(&parts);
//     let seeds = if let Some(("", seed_str)) = parts[0].split_once("seeds: ") {
//         seed_str
//             .split_whitespace()
//             .filter_map(|s| s.parse::<i64>().ok())
//             .collect::<Vec<i64>>()
//     } else {
//         return 0; // Early return to indicate an error
//     };

//     seeds
//         .iter()
//         .map(|&seed| {
//             category_maps.iter().fold(seed, |current, map| {
//                 map.iter()
//                     .find_map(|&(dest, start, range)| {
//                         if current >= start && current < start + range {
//                             Some(dest + (current - start))
//                         } else {
//                             None
//                         }
//                     })
//                     .unwrap_or(current)
//             })
//         })
//         .min()
//         .unwrap_or(0)
// }

fn find_destination_part_two(content: &str) -> i64 {
    let parts = content
        .split("\n\n")
        .map(str::trim_end)
        .collect::<Vec<&str>>();
    let category_maps = parse_maps(&parts);

    let seed_ranges = if let Some(("", seed_str)) = parts[0].split_once("seeds: ") {
        seed_str
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<i64>>()
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
            .collect::<Vec<(i64, i64)>>()
    } else {
        return 0; // Early return to indicate an error
    };

    let mut current_ranges = seed_ranges;

    for map in category_maps.iter() {
        let mut new_ranges = Vec::new();

        while let Some((start, end)) = current_ranges.pop() {
            let mut mapped = false;

            for &(dest, src, len) in map {
                let overlap_start = start.max(src);
                let overlap_end = end.min(src + len);

                if overlap_start < overlap_end {
                    // Map the overlapping part
                    new_ranges.push((overlap_start - src + dest, overlap_end - src + dest));

                    // Add unmapped parts back to current_ranges
                    if start < overlap_start {
                        current_ranges.push((start, overlap_start));
                    }
                    if overlap_end < end {
                        current_ranges.push((overlap_end, end));
                    }

                    mapped = true;
                    break;
                }
            }

            if !mapped {
                // If no mapping applied, keep the range as is
                new_ranges.push((start, end));
            }
        }

        current_ranges = new_ranges;
    }

    current_ranges
        .into_iter()
        .map(|(start, _)| start)
        .min()
        .unwrap_or(0)
}
