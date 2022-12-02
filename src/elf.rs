use std::borrow::Cow;

pub struct Elf {
    pub calories: Vec<u32>,
}

impl Elf {
    pub fn new(calories: Cow<[u32]>) -> Self {
        Elf {
            calories: calories.to_vec(),
        }
    }

    pub fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}
