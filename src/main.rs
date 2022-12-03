mod day1;
mod day2;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Advent, Advent, ein Lichtlein brennt:");

    println!("\nDay1:");
    let day1_input = read_file_content("src/day1/input.txt");
    println!("  Part 1: {}", day1::most_calories(&day1_input));
    println!("  Part 2: {}", day1::top3_calories(&day1_input));


    println!("\nDay2:");
    let day2_input = read_file_content("src/day2/input.txt");
    println!("  Part 1: {}", day2::total_score(&day2_input));
    println!("  Part 2: {}", day2::total_score2(&day2_input));
}

fn read_file_content(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    assert!(file.read_to_string(&mut contents).is_ok());
    contents
}
