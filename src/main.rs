mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day11;


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

    println!("\nDay5:");
    let day5_input = read_file_content("src/day5/input.txt");
    println!("  Part 1: {}", day5::crate_message(&day5_input));
    println!("  Part 2: {}", day5::crate_message2(&day5_input));

    println!("\nDay6:");
    let day6_input = read_file_content("src/day6/input.txt");
    println!("  Part 1: {}", day6::unique_characters_offset(&day6_input, 4));
    println!("  Part 2: {}", day6::unique_characters_offset(&day6_input, 14));

    println!("\nDay7:");
    let day7_input = read_file_content("src/day7/input.txt");
    println!("  Part 1: {}", day7::size_of_small_dirs(&day7_input));
    println!("  Part 2: {}", day7::smallest_directory_to_delete(&day7_input));

    println!("\nDay11:");
    let day11_input = read_file_content("src/day11/input.txt");
    println!("  Part 1: {}", day11::monkey_business(&day11_input, 20, true));
    println!("  Part 2: {}", day11::monkey_business(&day11_input, 10_000, false));
}

fn read_file_content(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    assert!(file.read_to_string(&mut contents).is_ok());
    contents
}
