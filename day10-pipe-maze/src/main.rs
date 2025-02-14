use day10_pipe_maze::{solve, Maze};
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./input");
    let input = fs::read_to_string(file_path).expect("Input file missing");

    let mut maze = Maze::load_from_input(&input);

    let (result_part1, result_part2) = solve(&mut maze);
    maze.print();
    println!("Part one result: {result_part1}");
    println!("Part two result: {result_part2}");
}
