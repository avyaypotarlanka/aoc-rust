pub mod solutions;

pub fn solve_day(day: u8) {
    match day {
        1 => solutions::day01::solve(),
	2 => println!("Day 02 to be built"),
        _ => println!("Day {} not built yet", day),
    }

}
