use day17_crucible::{solve_part1, solve_part2, Layout};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let layout = Layout::load_from_input(&input);

    let result = solve_part1(&layout);
    println!("Part one result: {result}");

    let result = solve_part2(&layout);
    println!("Part two result: {result}");
}
