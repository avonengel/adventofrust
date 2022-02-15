mod day1;
mod day2;
mod day3;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Advent, Advent, ein Lichtlein brennt:");
    
    println!("\nDay1:");
    let day1_input = read_file_content("src/day1/input");
    println!("  Part 1: {}", day1::count_increases(&day1_input));
    println!("  Part 2: {}", day1::count_sliding_window_increases(&day1_input));

    println!("\nDay2:");
    let day2_input = read_file_content("src/day2/input");
    println!("  Part 1: {}", day2::position_depth_product(&day2_input));
    println!("  Part 2: {}", day2::position_depth_product_with_aim(&day2_input));

    println!("\nDay3:");
    let day3_input = read_file_content("src/day3/input");
    println!("  Part 1 power consumption: {}", day3::power_consumption(&day3_input));
}

fn read_file_content(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    assert!(file.read_to_string(&mut contents).is_ok());
    contents
}
