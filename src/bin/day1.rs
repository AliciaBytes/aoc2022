use aoc2022::utilities::{
    create_elves_from_calorie_sections, get_calories_in_input, group_calories_by_elf,
};
use itertools::Itertools;

fn main() {
    let calories = get_calories_in_input(); // Collect the calories in a vector

    let elves = create_elves_from_calorie_sections(group_calories_by_elf(&calories));

    println!(
        "Part 1: {}",
        elves
            .iter() // Iterate over all the elves
            .map(|elf| elf.total_calories()) // Getting the total calories for each Elf
            .max() // Take out the max calories an Elf carries
            .unwrap_or(0) // Or 0 if there are no elves/calories
    );

    println!(
        "Part 2: {}",
        elves
            .iter() // Iterate over all the elves
            .map(|elf| elf.total_calories()) // Getting the total calories for each Elf
            .sorted() // Take out the max calories an Elf carries
            .rev()
            .take(3) // Take the top 3
            .sum::<u32>() // Sum the calories of the top 3 elves
    );
}
