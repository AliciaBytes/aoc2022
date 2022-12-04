use std::io::stdin;

fn main() {
    let lines: Vec<String> = stdin()
        .lines()
        .map(|line| line.unwrap_or_default())
        .filter(|line| !line.is_empty())
        .collect();

    let bounds: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split(|char| char == ',' || char == '-')
                .map(|string| string.parse::<u32>().unwrap_or_default())
                .collect::<Vec<u32>>()
        })
        .collect();

    let result = bounds
        .iter()
        .filter(|bounds| {
            (bounds[0] <= bounds[2] && bounds[1] >= bounds[3])
                || (bounds[0] >= bounds[2] && bounds[1] <= bounds[3])
        })
        .count();

    println!("Part 1: {}", result);

    let result = bounds
        .iter()
        .filter(|bounds| {
            (bounds[0] >= bounds[2] && bounds[0] <= bounds[3])
                || (bounds[1] >= bounds[2] && bounds[1] <= bounds[3])
                || (bounds[2] >= bounds[0] && bounds[2] <= bounds[1])
                || (bounds[3] >= bounds[0] && bounds[3] <= bounds[1])
        })
        .count();

    println!("Part 2: {}", result);
}
