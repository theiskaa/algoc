// 2023 - Day 1 - Trebuchet?!

/* Problem Statement
 *
 * Something is wrong with global snow production, and you've been selected to take a look.
 * The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
 *
 * You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
 * Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar;
 * the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
 *
 * You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even
 * sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on
 * did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already
 * loading you into a trebuchet ("please hold still, we need to strap you in").
 *
 * As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been
 * amended by a very young Elf who was apparently just excited to show off her art skills. Consequently,
 * the Elves are having trouble reading the values on the document.
 *
 * The newly-improved calibration document consists of lines of text;
 * each line originally contained a specific calibration value that the Elves now need to recover.
 * On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
 *
 * For example:
 *
 * 1abc2
 * pqr3stu8vwx
 * a1b2c3d4e5f
 * treb7uchet
 *
 * In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
 * Consider your entire calibration document. What is the sum of all of the calibration values?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    // Read input and parse to vector value.
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = trebuchet(input);
    println!("AoC:2023 • Day 1 • Trebuchet\nResult: {}", result)
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
        let mut collected = line
            .chars()
            .filter(|ch| ch.is_numeric())
            .collect::<String>();

        if let (Some(first), Some(last)) = (collected.chars().next(), collected.chars().last()) {
            collected = format!("{}{}", first, last);
            if let Ok(calculated) = collected.parse::<i32>() {
                result += calculated;
            }
        }
    }

    result
}
