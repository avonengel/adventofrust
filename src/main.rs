mod day1;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Advent, Advent, ein Lichtlein brennt:");

    println!("\nDay1:");
    let day1_input = read_file_content("src/day1/input");
    // println!("  Part 1: {}", day1::count_increases(&day1_input));
}

fn read_file_content(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    assert!(file.read_to_string(&mut contents).is_ok());
    contents
}
