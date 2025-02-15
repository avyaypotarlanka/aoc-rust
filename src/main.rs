use std::env;
use aoc24_rust::solve_day;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1)
        .and_then(|day_val| day_val.parse().ok())
        .unwrap_or(1);
    solve_day(day)
}
