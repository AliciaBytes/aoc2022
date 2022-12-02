use std::io::stdin;

use crate::elf::Elf;

pub fn get_calories_in_input() -> Vec<Option<u32>> {
    stdin()
        .lines() // For every line in the input
        .filter_map(|line| line.ok()) // Filter out invalid lines
        .map(|line| line.trim().parse::<u32>().ok()) // Trim the line and try to parse it into calories (u32)
        .collect() // Collect the calories in a vector
}

pub fn group_calories_by_elf(calories: &[Option<u32>]) -> impl Iterator<Item = &[Option<u32>]> {
    calories // For all the calories
        .split(|calorie| calorie.is_none()) // Split them on the empty ones
}

pub fn create_elves_from_calorie_sections<'a>(
    calorie_sections: impl Iterator<Item = &'a [Option<u32>]>,
) -> Vec<Elf> {
    let mut elves = Vec::new();
    calorie_sections.for_each(|calories| {
        // For each calorie section:
        elves.push(Elf::new(
            // Create an Elf and push it to the elves vector
            calories
                .iter()
                .map(|calorie| calorie.unwrap_or(0))
                .collect(),
        ))
    });

    elves
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RockPaperScissor {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl RockPaperScissor {
    pub fn losing(&self) -> Self {
        // Return the losing hand to a specific move
        match &self {
            Self::Rock => Self::Scissor,
            Self::Paper => Self::Rock,
            Self::Scissor => Self::Paper,
        }
    }

    pub fn winning(&self) -> Self {
        // Return the winning hand to a specific move
        match &self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissor,
            Self::Scissor => Self::Rock,
        }
    }
}

pub fn map_input_to_rock_paper_scissor(
    input: &str,
    knows_meaning: bool,
) -> (RockPaperScissor, RockPaperScissor) {
    // Map the instruction String into 2 Rock, Paper, Scissor moves
    let instructions: Vec<&str> = input.split(' ').collect();

    if instructions.len() != 2 {
        panic!("Invalid amount of instructions!");
    }

    map_symbols_to_rock_paper_scissor((instructions[0], instructions[1]), knows_meaning)
}

pub fn match_symbol_to_simple_meaning(symbol: &str) -> RockPaperScissor {
    // Match the symbol to the instruction according to the simplified rules
    match symbol {
        "A" | "X" => RockPaperScissor::Rock,
        "B" | "Y" => RockPaperScissor::Paper,
        "C" | "Z" => RockPaperScissor::Scissor,
        _ => panic!("Invalid value!"),
    }
}

pub fn map_symbols_to_rock_paper_scissor(
    symbols: (&str, &str),
    knows_meaning: bool,
) -> (RockPaperScissor, RockPaperScissor) {
    if !knows_meaning {
        (
            match_symbol_to_simple_meaning(symbols.0),
            match_symbol_to_simple_meaning(symbols.1),
        )
    } else {
        match symbols {
            (a, "X") => (
                match_symbol_to_simple_meaning(a),
                match_symbol_to_simple_meaning(a).losing(),
            ),
            (a, "Y") => (
                match_symbol_to_simple_meaning(a),
                match_symbol_to_simple_meaning(a),
            ),
            (a, "Z") => (
                match_symbol_to_simple_meaning(a),
                match_symbol_to_simple_meaning(a).winning(),
            ),
            _ => (RockPaperScissor::Paper, RockPaperScissor::Paper),
        }
    }
}

pub fn map_round_to_score(round: (RockPaperScissor, RockPaperScissor)) -> u32 {
    match round {
        (a, b) if a == b => 3 + a as u32,
        (a, b) => get_round_result(a, b) + b as u32,
    }
}

pub fn get_round_result(a: RockPaperScissor, b: RockPaperScissor) -> u32 {
    use RockPaperScissor::*;

    if a == b {
        3
    } else if (a == Rock && b == Paper)
        || (a == Paper && b == Scissor)
        || (a == Scissor && b == Rock)
    {
        6
    } else {
        0
    }
}
