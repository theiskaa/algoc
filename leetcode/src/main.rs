pub mod problems;
use std::env;

use problems::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <problem_name>");
        return;
    }

    match args[1].as_str() {
        "two_sum" => two_sum::run(),
        name => eprintln!("Error: problem doesn't exists with name {}", name),
    }
}
