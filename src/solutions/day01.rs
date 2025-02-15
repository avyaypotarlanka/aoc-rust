use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve() {

    if let Ok(lines) = read_lines("input/day01.txt") {
        let mut v1: Vec<i32> = Vec::new();
        let mut v2: Vec<i32> = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let split_str: Vec<&str> = line.split_whitespace().collect();
            v1.push(split_str[0].parse().unwrap());
            v2.push(split_str[1].parse().unwrap());
        }
        v1.sort();
        v2.sort();

        let total_distance: i32 = v1.iter().zip(v2.iter())
            .map(|(a,b)| (a - b).abs())
            .sum();

        println!("{total_distance}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
