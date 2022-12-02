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
