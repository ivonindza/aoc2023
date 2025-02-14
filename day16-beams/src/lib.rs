//! https://adventofcode.com/2023/day/16

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn next(&self, direction: Direction) -> Coord {
        match direction {
            Direction::N => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::S => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::E => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::W => Coord {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    pub fn to_usize_pair(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Coord {
        Coord { x, y }
    }
}

#[derive(Clone)]
pub struct Layout {
    tiles: Vec<Vec<char>>,
    energized_tiles: HashSet<(Coord, Direction)>,
}

impl Layout {
    pub fn load_from_input(input: &str) -> Layout {
        let mut tiles: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            tiles.push(line.chars().collect());
        }

        Layout {
            tiles,
            energized_tiles: HashSet::new(),
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        let max_x = self.tiles[0].len() as i32;
        let max_y = self.tiles.len() as i32;

        if coord.x < 0 || coord.x >= max_x {
            false
        } else if coord.y < 0 || coord.y >= max_y {
            false
        } else {
            true
        }
    }

    fn track_beam(&mut self, coord: Coord, direction: Direction) {
        // Stop if out of bounds of the layout
        if !self.contains(&coord) {
            return;
        }

        // Stop if the beam is entering a loop
        if self.energized_tiles.contains(&(coord, direction)) {
            return;
        }

        self.energized_tiles.insert((coord, direction));

        let (x, y) = coord.to_usize_pair();
        match self.tiles[y][x] {
            '.' => {
                self.track_beam(coord.next(direction), direction);
            },
            '/' => match direction {
                Direction::N => {
                    self.track_beam(coord.next(Direction::E), Direction::E);
                },
                Direction::S => {
                    self.track_beam(coord.next(Direction::W), Direction::W);
                },
                Direction::E => {
                    self.track_beam(coord.next(Direction::N), Direction::N);
                },
                Direction::W => {
                    self.track_beam(coord.next(Direction::S), Direction::S);
                },
            },
            '\\' => match direction {
                Direction::N => {
                    self.track_beam(coord.next(Direction::W), Direction::W);
                },
                Direction::S => {
                    self.track_beam(coord.next(Direction::E), Direction::E);
                },
                Direction::E => {
                    self.track_beam(coord.next(Direction::S), Direction::S);
                },
                Direction::W => {
                    self.track_beam(coord.next(Direction::N), Direction::N);
                },
            },
            '-' => match direction {
                Direction::E | Direction::W => {
                    self.track_beam(coord.next(direction), direction);
                },
                Direction::N | Direction::S => {
                    self.track_beam(coord.next(Direction::E), Direction::E);
                    self.track_beam(coord.next(Direction::W), Direction::W);
                },
            },
            '|' => match direction {
                Direction::N | Direction::S => {
                    self.track_beam(coord.next(direction), direction);
                },
                Direction::E | Direction::W => {
                    self.track_beam(coord.next(Direction::N), Direction::N);
                    self.track_beam(coord.next(Direction::S), Direction::S);
                },
            },
            _ => unreachable!(),
        }
    }
}

/// Count the energized tiles for the given start tile and direction.
fn count_energized(mut layout: Layout, start_coord: Coord, direction: Direction) -> u32 {
    layout.track_beam(start_coord, direction);

    // Count each energized tile once
    layout
        .energized_tiles
        .iter()
        .map(|(coord, _)| coord)
        .collect::<HashSet<_>>()
        .len() as u32
}

/// Discover all tiles that become energized in the layout and count them. Start from the top-left
/// corner going to the right.
pub fn solve_part1(layout: &Layout) -> u32 {
    count_energized(layout.clone(), Coord::from((0, 0)), Direction::E)
}

/// Discover the starting point on the edge that maximizes the energized tiles. Return the max
/// energized tiles.
pub fn solve_part2(layout: &Layout) -> u32 {
    let max_x = layout.tiles[0].len() as i32;
    let max_y = layout.tiles.len() as i32;

    let mut start_configurations = Vec::new();
    for x in 0..max_x {
        start_configurations.push((Coord::from((x, 0)), Direction::S));
        start_configurations.push((Coord::from((x, max_y - 1)), Direction::N));
    }
    for y in 0..max_y {
        start_configurations.push((Coord::from((0, y)), Direction::E));
        start_configurations.push((Coord::from((max_x - 1, y)), Direction::W));
    }

    let mut max_energized: u32 = 0;
    for (start_coord, direction) in start_configurations {
        let count = count_energized(layout.clone(), start_coord, direction);
        max_energized = std::cmp::max(max_energized, count);
    }
    max_energized
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "};

        let layout = Layout::load_from_input(&input);
        let result = solve_part1(&layout);
        assert_eq!(result, 46);
        let result = solve_part2(&layout);
        assert_eq!(result, 51);
    }
}
