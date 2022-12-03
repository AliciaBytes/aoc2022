use std::{collections::BTreeSet, io::stdin};

use itertools::Itertools;

fn main() {
    let lines: Vec<String> = stdin()
        .lines()
        .map(|line| line.unwrap_or_default())
        .filter(|line| !line.is_empty())
        .collect();

    let mut result: u32 = lines
        .iter()
        .map(|line| {
            let set = BTreeSet::from_iter(line.bytes().take(line.len() / 2));
            line.bytes()
                .skip(line.len() / 2)
                .find(|byte| set.contains(byte))
                .unwrap_or_default()
        })
        .map(|byte| match byte {
            b'a'..=b'z' => (byte - b'a') as u32 + 1,
            b'A'..=b'z' => (byte - b'A') as u32 + 27,
            _ => 0,
        })
        .sum();

    println!("Part 1: {}", result);

    result = 0;

    for chunk in &lines.iter().chunks(3) {
        result += chunk
            .map(|string| BTreeSet::from_iter(string.bytes()))
            .reduce(|accum, item| accum.intersection(&item).cloned().collect())
            .map(|set| set.iter().take(1).sum())
            .map(|byte| match byte {
                b'a'..=b'z' => (byte - b'a') as u32 + 1,
                b'A'..=b'z' => (byte - b'A') as u32 + 27,
                _ => 0,
            })
            .unwrap_or_default();
    }

    println!("Part 2: {}", result);
}
