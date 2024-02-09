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
        "valid_parentheses" => valid_parentheses::run(),
        "merge_two_sorted_lists" => merge_two_sorted_lists::run(),
        "max_profit" | "best_time_to_buy_and_sell_stock" => best_time_to_buy_and_sell_stock::run(),
        name => eprintln!("Error: problem doesn't exists with name {}", name),
    }
}
