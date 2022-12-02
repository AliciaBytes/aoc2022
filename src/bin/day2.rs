use std::io::stdin;

use aoc2022::utilities::{map_input_to_rock_paper_scissor, map_round_to_score};

fn main() {
    let rounds: Vec<String> = stdin()
        .lines() // Get all lines in the input
        .filter_map(|line| line.ok()) // Discard invalid ones
        .map(|line| line.trim().to_owned()) // Trim whitespace
        .filter(|line| !line.is_empty()) // Discard empty lines
        .collect(); // Collect them in a vector

    let result: u32 = rounds
        .iter() // Go over all the lines
        .map(|instructions| map_input_to_rock_paper_scissor(instructions, false)) // map each line to instructions
        .map(map_round_to_score) // Map instructions to a score
        .sum(); // Sum the scores

    println!("Part 1: {}", result);

    let result: u32 = rounds
        .iter() // Go over all the lines
        .map(|instructions| map_input_to_rock_paper_scissor(instructions, true)) // map each line to instructions
        .map(map_round_to_score) // Map instructions to a score
        .sum(); // Sum the scores

    println!("Part 2: {}", result);
}
