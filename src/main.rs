mod day1;
mod day2;
mod day3;
mod day4;


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

    println!("\nDay3:");
    let day3_input = read_file_content("src/day3/input.txt");
    println!("  Part 1: {}", day3::priority_sum(&day3_input));
    println!("  Part 2: {}", day3::badge_priority_sum(&day3_input));
    println!("\nDay4:");
    let day4_input = read_file_content("src/day4/input.txt");
    println!("  Part 1: {}", day4::fully_contained_pairs(&day4_input));
    println!("  Part 2: {}", day4::overlapping_pairs(&day4_input));
}

fn read_file_content(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    assert!(file.read_to_string(&mut contents).is_ok());
    contents
}
