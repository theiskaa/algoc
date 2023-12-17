// 2023 - Day 3 - Gear Ratios

/* Problem Statement
 * You and the Elf eventually reach a gondola lift station; he says the gondola
 * lift will take you up to the water source, but this is as far as he can
 * bring you. You go inside.
 *
 * It doesn't take long to find the gondolas, but there seems to be a problem:
 * they're not moving.
 *
 * "Aaah!"
 *
 * You turn around to see a slightly-greasy Elf with a wrench and
 * a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift
 * isn't working right now; it'll still be a while before I can fix it."
 * You offer to help.
 *
 * The engineer explains that an engine part seems to be missing from the engine,
 * but nobody can figure out which one. If you can add up all the part numbers
 * in the engine schematic, it should be easy to work out which part is missing.
 *
 * The engine schematic (your puzzle input) consists of a visual representation
 * of the engine. There are lots of numbers and symbols you don't really
 * understand, but apparently any number adjacent to a symbol, even diagonally,
 * is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
 *
 * Here is an example engine schematic:
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
 * In this schematic, two numbers are not part numbers because they are not
 * adjacent to a symbol: 114 (top right) and 58 (middle right).
 * Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
 *
 * Of course, the actual engine schematic is much larger.
 * What is the sum of all of the part numbers in the engine schematic?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = gear_ratios(input);
    println!("AoC:2023 • Day 3 • Gear Ratios\nResult: {}", result);
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
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut sum = 0;
    let mut current = 0;
    let mut adjacent = false;

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if !value.is_ascii_digit() {
                continue;
            }

            for row_offset in -1..=1 {
                for col_offset in -1..=1 {
                    if row_offset == 0 && col_offset == 0 {
                        continue;
                    }

                    let adjacent_row_idx = row_index as i32 + row_offset;
                    let adjacent_col_idx = col_index as i32 + col_offset;

                    if adjacent_row_idx < 0
                        || adjacent_row_idx >= grid.len() as i32
                        || adjacent_col_idx < 0
                        || adjacent_col_idx >= row.len() as i32
                    {
                        continue;
                    }

                    let adjacent_value = grid[adjacent_row_idx as usize][adjacent_col_idx as usize];
                    if !adjacent_value.is_ascii_digit() && adjacent_value != '.' {
                        adjacent = true;
                    }
                }
            }

            current = current * 10 + value.to_digit(10).unwrap();

            if col_index + 1 >= row.len() || !row[col_index + 1].is_ascii_digit() {
                if adjacent {
                    sum += current;
                }

                current = 0;
                adjacent = false;
            }
        }
    }

    sum
}
