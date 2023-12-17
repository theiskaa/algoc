// 2023 - Day 3 - Gear Ratios | Part Two

/* Problem Statement
 * The engineer finds the missing part and installs it in the engine!
 * As the engine springs to life, you jump in the closest gondola,
 * finally ready to ascend to the water source.
 *
 * You don't seem to be going very fast, though.
 * Maybe something is still wrong? Fortunately, the gondola has a phone
 * labeled "help", so you pick it up and the engineer answers.
 *
 * Before you can explain the situation, she suggests that you look out the window.
 * There stands the engineer, holding a phone in one hand and waving with the other.
 * You're going so slowly that you haven't even left the station. You exit the gondola.
 *
 * The missing part wasn't the only issue - one of the gears in the engine is wrong.
 * A gear is any * symbol that is adjacent to exactly two part numbers.
 * Its gear ratio is the result of multiplying those two numbers together.
 *
 * This time, you need to find the gear ratio of every gear and add them all
 * up so that the engineer can figure out which gear needs to be replaced.
 *
 * Consider the same engine schematic again:
 * [
 *   467..114..
 *   ...*......
 *   ..35..633.
 *   ......#...
 *   617*......
 *   .....+.58.
 *   ..592.....
 *   ......755.
 *   ...$.*....
 *   .664.598..
 * ]
 *
 * In this schematic, there are two gears. The first is in the top left;
 * it has part numbers 467 and 35, so its gear ratio is 16345.
 * The second gear is in the lower right; its gear ratio is 451490.
 * (The * adjacent to 617 is not a gear because it is only adjacent to one part number.)
 * Adding up all of the gear ratios produces 467835.
 *
 * What is the sum of all of the gear ratios in your engine schematic?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

#[allow(unused_variables, dead_code)]

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = gear_ratios(input);
    println!(
        "AoC:2023 • Day 3 • Gear Ratios | Part Two\nResult: {}",
        result
    );
}

fn parse_input(file: File) -> String {
    let reader = BufReader::new(file);
    reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .join("\n")
}

pub fn gear_ratios(input: String) -> u32 {
    let engine = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut stars: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    for (row_index, row) in engine.iter().enumerate() {
        let mut current = 0;
        let mut adjacent_positions: HashSet<(i32, i32)> = HashSet::new();

        for (col_index, &value) in row.iter().enumerate() {
            if value.is_ascii_digit() {
                current = current * 10 + value.to_digit(10).unwrap();

                for row_offset in -1..=1 {
                    for col_offset in -1..=1 {
                        if row_offset == 0 && col_offset == 0 {
                            continue;
                        }

                        let adjacent_row_index = row_index as i32 + row_offset;
                        let adjacent_col_index = col_index as i32 + col_offset;

                        if adjacent_row_index >= 0
                            && adjacent_row_index < engine.len() as i32
                            && adjacent_col_index >= 0
                            && adjacent_col_index < row.len() as i32
                            && engine[adjacent_row_index as usize][adjacent_col_index as usize] == '*'
                        {
                            adjacent_positions.insert((adjacent_row_index, adjacent_col_index));
                        }
                    }
                }

                if col_index + 1 == row.len() || !row[col_index + 1].is_ascii_digit() {
                    for point in &adjacent_positions {
                        stars.entry(*point).or_default().push(current);
                    }

                    current = 0;
                    adjacent_positions.clear();
                }
            }
        }
    }

    stars
        .values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum()
}
