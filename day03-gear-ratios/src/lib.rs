//! https://adventofcode.com/2023/day/3

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NumberCoordinate {
    pub x1: i32,
    pub x2: i32,
    pub y: i32,
}

impl From<Coordinate> for NumberCoordinate {
    fn from(coord: Coordinate) -> Self {
        NumberCoordinate {
            x1: coord.x,
            x2: coord.x,
            y: coord.y,
        }
    }
}

#[derive(Debug)]
struct DiagramNumber {
    number: u32,
    coord: NumberCoordinate,
    is_part_number: bool,
}

impl DiagramNumber {
    pub fn new(digit: u32, coord: Coordinate) -> Self {
        DiagramNumber {
            number: digit,
            coord: NumberCoordinate::from(coord),
            is_part_number: false,
        }
    }

    pub fn add_digit(&mut self, digit: u32) {
        self.number = self.number * 10 + digit;
        self.coord.x2 += 1;
    }

    pub fn neighbor_coords(&self) -> HashSet<Coordinate> {
        let mut coords: HashSet<Coordinate> = HashSet::new();

        let row_up = self.coord.y - 1;
        let row_down = self.coord.y + 1;
        let col_left = self.coord.x1 - 1;
        let col_right = self.coord.x2 + 1;

        // Insert neighbor coords above and below the number
        for i in col_left..=col_right {
            coords.insert(Coordinate::from((i, row_up)));
            coords.insert(Coordinate::from((i, row_down)));
        }

        // Insert neighbor coords left and right of the number
        for j in row_up..=row_down {
            coords.insert(Coordinate::from((col_left, j)));
            coords.insert(Coordinate::from((col_right, j)));
        }

        coords
    }
}

#[derive(Debug)]
struct Symbol {
    is_star_symbol: bool,
    adjacent_part_numbers: u32,
    product_of_adjacent_part_numbers: u32,
}

impl Symbol {
    pub fn new(is_star_symbol: bool) -> Symbol {
        Symbol {
            is_star_symbol: is_star_symbol,
            adjacent_part_numbers: 0,
            product_of_adjacent_part_numbers: 1,
        }
    }

    pub fn is_gear_symbol(&self) -> bool {
        self.is_star_symbol && self.adjacent_part_numbers == 2
    }
}

#[derive(Debug)]
pub struct Solver {
    numbers: Vec<DiagramNumber>,
    symbols_by_coord: HashMap<Coordinate, Symbol>,
}

impl Solver {
    pub fn parse_from_input(input: &str) -> Solver {
        let mut numbers: Vec<DiagramNumber> = vec![];
        let mut symbols_by_coord: HashMap<Coordinate, Symbol> = HashMap::new();

        // Track the current number as we assemble it digit by digit
        let mut current_number: Option<DiagramNumber> = None;

        for (j, line) in input.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                let coord = Coordinate::from((i as i32, j as i32));
                match ch {
                    '0'..='9' => {
                        let digit = ch.to_digit(10).expect("Failed to convert digit {ch}");
                        match current_number {
                            None => {
                                current_number = Some(DiagramNumber::new(digit, coord));
                            },
                            Some(ref mut current_number) => {
                                current_number.add_digit(digit);
                            },
                        }
                    },
                    _ => {
                        if let Some(current_number_inner) = current_number {
                            numbers.push(current_number_inner);
                            current_number = None;
                        }
                        if ch != '.' {
                            let symbol = Symbol::new(ch == '*');
                            symbols_by_coord.insert(coord, symbol);
                        }
                    },
                }
            }

            // If a number finishes on the last character of the line, we need to push it now,
            // because next line can start with another number and we don't want them to be treated
            // as one long number that spans over the end of line.
            if let Some(current_number_inner) = current_number {
                numbers.push(current_number_inner);
                current_number = None;
            }
        }

        Solver {
            numbers,
            symbols_by_coord,
        }
    }

    fn mark_part_numbers_and_gears(&mut self) {
        let symbol_coords: HashSet<Coordinate> = self.symbols_by_coord.keys().cloned().collect();

        for number in &mut self.numbers {
            let neighbor_coords = number.neighbor_coords();
            if neighbor_coords.is_disjoint(&symbol_coords) {
                continue;
            }

            // Otherwise, we have a part number. We should update the symbol info as well.
            number.is_part_number = true;
            for coord in &neighbor_coords {
                if let Some(symbol) = self.symbols_by_coord.get_mut(coord) {
                    symbol.adjacent_part_numbers += 1;
                    symbol.product_of_adjacent_part_numbers *= number.number;
                }
            }
        }
    }

    fn sum_of_part_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|number| number.is_part_number)
            .map(|number| number.number)
            .sum()
    }

    fn sum_of_gear_ratios(&self) -> u32 {
        self.symbols_by_coord
            .values()
            .filter(|symbol| symbol.is_gear_symbol())
            .map(|symbol| symbol.product_of_adjacent_part_numbers)
            .sum()
    }

    pub fn solve(&mut self) -> (u32, u32) {
        self.mark_part_numbers_and_gears();
        let part1 = self.sum_of_part_numbers();
        let part2 = self.sum_of_gear_ratios();
        (part1, part2)
    }

    // Print the input with part numbers marked in green, other numbers marked in red, and gear
    // symbols marked in blue.
    //
    // Call this after running the solve method.
    pub fn print_colored_input(&self, input: &str) {
        use colored::Colorize;

        let mut red_digits: HashSet<Coordinate> = HashSet::new();
        let mut green_digits: HashSet<Coordinate> = HashSet::new();
        for number in &self.numbers {
            for i in number.coord.x1..=number.coord.x2 {
                let coord = Coordinate::from((i, number.coord.y));
                if number.is_part_number {
                    green_digits.insert(coord);
                } else {
                    red_digits.insert(coord);
                }
            }
        }

        let mut blue_symbols: HashSet<Coordinate> = HashSet::new();
        for (coord, symbol) in &self.symbols_by_coord {
            if symbol.is_gear_symbol() {
                blue_symbols.insert(coord.clone());
            }
        }

        for (j, line) in input.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                let coord = Coordinate::from((i as i32, j as i32));
                if red_digits.contains(&coord) {
                    print!("{}", ch.to_string().red());
                } else if green_digits.contains(&coord) {
                    print!("{}", ch.to_string().green());
                } else if blue_symbols.contains(&coord) {
                    print!("{}", ch.to_string().blue());
                } else {
                    print!("{}", ch);
                }
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        let mut solver = Solver::parse_from_input(input);

        let (result_part1, result_part2) = solver.solve();
        assert_eq!(result_part1, 4361);
        assert_eq!(result_part2, 467835);
    }

    #[test]
    fn test2() {
        let input = indoc! {"
            ..100
            200*.
            ...-.
            ..300
        "};

        let mut solver = Solver::parse_from_input(input);

        let (result_part1, result_part2) = solver.solve();
        assert_eq!(result_part1, 600);
        assert_eq!(result_part2, 20000);
    }
}
