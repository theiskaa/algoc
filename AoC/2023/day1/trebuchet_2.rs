// 2023 - Day 1 - Trebuchet?! | Part Two

/* Problem Statement
 *
 * Your calculation isn't quite right.
 * It looks like some of the digits are actually spelled out
 * with letters: one, two, three, four, five, six, seven, eight,
 * and nine also count as valid "digits".
 *
 * Equipped with this new information, you now need to find the real
 * first and last digit on each line. For example:
 * [
 *  two1nine
 *  eightwothree
 *  abcone2threexyz
 *  xtwone3four
 *  4nineeightseven2
 *  zoneight234
 *  7pqrstsixteen
 * ]
 *
 * In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
 * Adding these together produces 281.
 *
 * What is the sum of all of the calibration values?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = trebuchet(input);
    println!(
        "AoC:2023 • Day 1 • Trebuchet | Part Two \nResult: {}",
        result
    )
}

// Parses the [input.txt] file to the Vector String
fn parse_input(file: File) -> Vec<String> {
    let reader = BufReader::new(file);
    let mut res: Vec<String> = Vec::new();
    for line in reader.lines() {
        let current = line.unwrap();
        res.push(current.clone());
    }

    return res;
}

fn trebuchet(input: Vec<String>) -> i32 {
    let mut result = 0;

    for line in input {
        let replaced = replace_word_numbers(line.clone());
        let mut collected = replaced.chars().filter(|ch| ch.is_numeric()).collect::<String>();

        if let (Some(first), Some(last)) = (collected.chars().next(), collected.chars().last()) {
            collected = format!("{}{}", first, last);
            if let Ok(calculated) = collected.parse::<i32>() {
                result += calculated;
            }
        }
    }

    result
}

// Replaces the digit-word matches in line by ordering them.
fn replace_word_numbers(line: String) -> String {
    let matches = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut replaced = line.clone();
    while let Some((start, _, number)) = matches.iter()
        .filter_map(|&(word, number)| replaced.find(word).map(|start| (start, word, number)))
        .min_by_key(|&(start, _, _)| start) {
            replaced.replace_range(start..start + number.len(), number);
    }

    replaced
}
