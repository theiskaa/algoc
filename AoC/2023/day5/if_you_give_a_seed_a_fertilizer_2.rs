// 2023 - Day 5 - If You Give A Seed A Fertilizer | Part Two

/* Problem Statement
 * Everyone will starve if you only plant such a small number of seeds.
 * Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.
 *
 * The values on the initial seeds: line come in pairs. Within each pair,
 * the first value is the start of the range and the second value is the length of the range.
 * So, in the first line of the example above:
 * [
 *   seeds: 79 14 55 13
 * ]
 *
 * This line describes two ranges of seed numbers to be planted in the garden.
 * The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92.
 * The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.
 *
 * Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.
 *
 * In the above example, the lowest location number can be obtained from seed
 * number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77,
 * temperature 45, humidity 46, and location 46. So, the lowest location number is 46.
 *
 * Consider all of the initial seed numbers listed in the ranges on the first
 * line of the almanac. What is the lowest location number that corresponds
 * to any of the initial seed numbers?
*/

use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = if_you_give_a_seed_a_fertilizer(input);
    println!(
        "AoC:2023 • Day 5 • If You Give A Seed A Fertilizer | Part Two\nResult: {}",
        result
    )
}

#[derive(Debug)]
struct Mapping {
    dst_start: i64,
    src_start: i64,
    range_len: i64,
}

struct Data {
    seeds: Vec<(i64, i64)>,
    maps: Vec<Vec<Mapping>>,
}

fn parse_input(file: File) -> Data {
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let raw_seeds: Vec<i64> = lines[0].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let seeds = raw_seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    let mut maps: Vec<Vec<Mapping>> = Vec::new();
    let mut i = 2;
    while i < lines.len() {
        let mut map: Vec<Mapping> = Vec::new();
        i += 1;
        while i < lines.len() && !lines[i].is_empty() {
            let parts: Vec<i64> = lines[i].split_whitespace().map(|s| s.parse().unwrap()).collect();
            map.push(Mapping {
                dst_start: parts[0],
                src_start: parts[1],
                range_len: parts[2],
            });
            i += 1;
        }
        map.sort_by_key(|m| m.src_start);
        maps.push(map);
        i += 1;
    }

    Data { seeds, maps }
}

fn if_you_give_a_seed_a_fertilizer(input: Data) -> i64 {
    let seed_ranges = input.seeds;
    let maps = input.maps;

    let mut seed_ranges: Vec<(i64, i64)> = seed_ranges
        .iter()
        .map(|&(start, len)| (start, start + len))
        .collect();

    let maps: Vec<Vec<Vec<i64>>> = maps
        .iter()
        .map(|map| {
            map.iter()
                .map(|mapping| vec![mapping.dst_start, mapping.src_start, mapping.range_len])
                .collect()
        })
        .collect();

    for map in maps {
        let mut ranges: Vec<(i64, i64)> = Vec::new();

        while !seed_ranges.is_empty() {
            let mut overlap = false;
            let seed = seed_ranges.pop().unwrap();
            for row in map.iter() {
                let overlap_start = cmp::max(seed.0, row[1]);
                let overlap_end = cmp::min(seed.1, row[1] + row[2]);
                if overlap_start < overlap_end {
                    ranges.push((
                        overlap_start - row[1] + row[0],
                        overlap_end - row[1] + row[0],
                    ));
                    if overlap_start > seed.0 {
                        seed_ranges.push((seed.0, overlap_start));
                    }
                    if seed.1 > overlap_end {
                        seed_ranges.push((overlap_end, seed.1));
                    }
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                ranges.push(seed);
            }
        }
        seed_ranges = ranges;
    }

    seed_ranges.iter().map(|x: &(i64, i64)| x.0).min().unwrap()
}
