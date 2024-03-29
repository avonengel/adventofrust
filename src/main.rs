#![feature(test)]

use std::fs::File;
use std::io::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day11;
mod day12;
mod day10;
mod day13;
mod day14;


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

    println!("\nDay8:");
    let day8_input = read_file_content("src/day8/input.txt");
    println!("  Part 1: {}", day8::count_visible_trees(&day8_input));
    println!("  Part 2: {}", day8::highest_scenic_score(&day8_input));

    println!("\nDay9:");
    let day9_input = read_file_content("src/day9/input.txt");
    println!("  Part 1: {}", day9::part1(&day9_input));
    println!("  Part 2: {}", day9::part2(&day9_input));

    println!("\nDay10:");
    let day10_input = read_file_content("src/day10/input.txt");
    println!("  Part 1: {}", day10::signal_strength(&day10_input));
    println!("  Part 2: \n{}", day10::print(&day10_input));

    println!("\nDay11:");
    let day11_input = read_file_content("src/day11/input.txt");
    println!("  Part 1: {}", day11::monkey_business(&day11_input, 20, true));
    println!("  Part 2: {}", day11::monkey_business(&day11_input, 10_000, false));

    println!("\nDay12:");
    let day12_input = read_file_content("src/day12/input.txt");
    println!("  Part 1: {}", day12::steps_to_signal(&day12_input));
    println!("  Part 2: {}", day12::shortest_hike(&day12_input));

    println!("\nDay13:");
    let day13_input = read_file_content("src/day13/input.txt");
    println!("  Part 1: {}", day13::part1(&day13_input));
    println!("  Part 2: {}", day13::part2(&day13_input));

    println!("\nDay14:");
    let day14_input = read_file_content("src/day14/input.txt");
    // TODO this seems to be extremely inefficient, as it takes several seconds => learn how to profile and optimize it
    println!("  Part 1: {}", day14::part1(&day14_input));
    // println!("  Part 2: {}", day13::part2(&day13_input));
}

fn read_file_content(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    assert!(file.read_to_string(&mut contents).is_ok());
    contents
}
