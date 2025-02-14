use day11_cosmic_exp::{solve_part1, solve_part2, CosmicImage};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let space = CosmicImage::load_from_input(&input);

    let result = solve_part1(&mut space.clone());
    println!("Part one result: {result}");

    let result = solve_part2(&mut space.clone());
    println!("Part two result: {result}");
}
