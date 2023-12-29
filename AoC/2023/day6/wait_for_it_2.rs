// 2023 - Day 6 - Wait For It | Part Two

/* Problem Statement
* As the race is about to start, you realize the piece of paper with race times
* and record distances you got earlier actually just has very bad kerning.
* There's really only one race - ignore the spaces between the numbers on each line.
*
* So, the example from before:
*
* [
*   Time:      7  15   30
*   Distance:  9  40  200
* ]
*
* ...now instead means this:
*
* [
*   Time:      71530
*   Distance:  940200
* ]
* Now, you have to figure out how many ways there are to win this single race.
* In this example, the race lasts for 71530 milliseconds and the record distance
* you need to beat is 940200 millimeters. You could hold the button anywhere
* from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!
*
* How many ways can you beat the record in this one much longer race?
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_variables, dead_code)]

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = parse_input(file);

    let result = wait_for_it(input);
    println!("AoC:2023 • Day 6 • Wait For It | Part Two\nResult: {}", result)
}

struct TimeDistance {
    time: i64,
    distance: i64,
}

fn parse_input(file: File) -> TimeDistance {
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let time: i64 = lines[0].split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse().unwrap();
    let distance: i64 = lines[1].split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse().unwrap();
    return TimeDistance { time, distance };
}

fn wait_for_it(input: TimeDistance) -> i64 {
    let mut ways: i64 = 0;
    for t in 0..=input.time {
        let distance = t * (input.time - t);
        if distance > input.distance {
            ways += 1;
        }
    }
    ways
}
