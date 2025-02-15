pub mod solutions;

pub fn solve_day(day: u8) {
    match day {
        1 => solutions::day01::solve(),
        _ => println!("Day {} not built yet", day),
    }

}
