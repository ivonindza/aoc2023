//! https://adventofcode.com/2023/day/10

use std::collections::VecDeque;

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, PartialEq)]
enum TileType {
    NS,
    NE,
    NW,
    SE,
    SW,
    EW,
    Ground,
    Start,
}

#[derive(Debug, Clone, PartialEq)]
enum MiniTileType {
    Unvisited,
    Visited,
    Loop,
    Outside,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Coord {
        Coord { x, y }
    }
}

pub struct Maze {
    start_coord: Coord,
    tiles: Vec<Vec<TileType>>,
    mini_tiles: Vec<Vec<MiniTileType>>,
}

impl Maze {
    pub fn load_from_input(input: &str) -> Maze {
        let mut start_coord: Coord = Coord::from((0, 0));
        let mut maze: Vec<Vec<TileType>> = Vec::new();

        for (j, line) in input.lines().enumerate() {
            let mut maze_line: Vec<TileType> = Vec::new();
            for (i, ch) in line.chars().enumerate() {
                let tile = match ch {
                    '|' => TileType::NS,
                    'L' => TileType::NE,
                    'J' => TileType::NW,
                    'F' => TileType::SE,
                    '7' => TileType::SW,
                    '-' => TileType::EW,
                    '.' => TileType::Ground,
                    'S' => {
                        start_coord = Coord::from((i, j));
                        TileType::Start
                    },
                    _ => panic!("Unknown tile type"),
                };
                maze_line.push(tile);
            }
            maze.push(maze_line);
        }

        // For each normal tile we make a grid of 9 mini-tiles
        let (m, n) = (maze.len(), maze[0].len());
        let mini_tiles: Vec<Vec<MiniTileType>> = vec![vec![MiniTileType::Unvisited; 3 * n]; 3 * m];

        Maze {
            start_coord,
            tiles: maze,
            mini_tiles,
        }
    }

    pub fn print(&self) {
        use colored::Colorize;

        for line in self.mini_tiles.iter() {
            for minitile in line.iter() {
                match minitile {
                    MiniTileType::Loop => print!("{}", ".".blue()),
                    MiniTileType::Outside => print!("{}", ".".green()),
                    // The remainder is Inside
                    _ => print!("{}", ".".red()),
                }
            }
            println!();
        }
    }

    /// Discover the tile type of the start tile based on the neighbouring tiles.
    fn discover_start_tile_type(&self, start_coord: &Coord) -> TileType {
        let n_tile = self.tiles[start_coord.y - 1][start_coord.x].clone();
        let s_tile = self.tiles[start_coord.y + 1][start_coord.x].clone();
        let e_tile = self.tiles[start_coord.y][start_coord.x + 1].clone();

        match n_tile {
            // Start tile connects to N
            TileType::NS | TileType::SE | TileType::SW => {
                match s_tile {
                    // Start tile connects to N and S
                    TileType::NS | TileType::NE | TileType::NW => TileType::NS,
                    // Start tile connects to N, but not to S
                    _ => {
                        match e_tile {
                            // Start tile connects to N and E
                            TileType::NW | TileType::SW | TileType::EW => TileType::NE,
                            // Start tile connects to N, but not to S or E
                            _ => TileType::NW,
                        }
                    },
                }
            },
            // Start tile does not connect to N
            _ => {
                match s_tile {
                    // Start tile connects to S, but not to N
                    TileType::NS | TileType::NE | TileType::NW => {
                        match e_tile {
                            // Start tile connects to S and E
                            TileType::NW | TileType::SW | TileType::EW => TileType::SE,
                            // Start tile connects to S, but not to N or e
                            _ => TileType::SW,
                        }
                    },
                    // Start tile does not connect to N or S
                    _ => TileType::EW,
                }
            },
        }
    }

    fn move_one_tile(
        &self,
        current_coord: &mut Coord,
        current_tile: &mut TileType,
        current_direction: &mut Direction,
    ) {
        (*current_coord, *current_tile, *current_direction) =
            match (&current_tile, &current_direction) {
                (TileType::NS, Direction::S) => self.move_up(current_coord),
                (TileType::NS, Direction::N) => self.move_down(current_coord),
                (TileType::NE, Direction::E) => self.move_up(current_coord),
                (TileType::NE, Direction::N) => self.move_right(current_coord),
                (TileType::NW, Direction::W) => self.move_up(current_coord),
                (TileType::NW, Direction::N) => self.move_left(current_coord),
                (TileType::SE, Direction::E) => self.move_down(current_coord),
                (TileType::SE, Direction::S) => self.move_right(current_coord),
                (TileType::SW, Direction::W) => self.move_down(current_coord),
                (TileType::SW, Direction::S) => self.move_left(current_coord),
                (TileType::EW, Direction::W) => self.move_right(current_coord),
                (TileType::EW, Direction::E) => self.move_left(current_coord),
                _ => panic!("Unexpected TileType/Direction combination"),
            }
    }

    fn move_up(&self, prev_coord: &Coord) -> (Coord, TileType, Direction) {
        let coord = Coord {
            x: prev_coord.x,
            y: prev_coord.y - 1,
        };
        let tile = self.tiles[coord.y][coord.x].clone();
        let direction = Direction::S;
        (coord, tile, direction)
    }

    fn move_down(&self, prev_coord: &Coord) -> (Coord, TileType, Direction) {
        let coord = Coord {
            x: prev_coord.x,
            y: prev_coord.y + 1,
        };
        let tile = self.tiles[coord.y][coord.x].clone();
        let direction = Direction::N;
        (coord, tile, direction)
    }

    fn move_left(&self, prev_coord: &Coord) -> (Coord, TileType, Direction) {
        let coord = Coord {
            x: prev_coord.x - 1,
            y: prev_coord.y,
        };
        let tile = self.tiles[coord.y][coord.x].clone();
        let direction = Direction::E;
        (coord, tile, direction)
    }

    fn move_right(&self, prev_coord: &Coord) -> (Coord, TileType, Direction) {
        let coord = Coord {
            x: prev_coord.x + 1,
            y: prev_coord.y,
        };
        let tile = self.tiles[coord.y][coord.x].clone();
        let direction = Direction::W;
        (coord, tile, direction)
    }

    /// Detect and mark the main loop using DFS. Returns the length of the loop.
    fn detect_loop(&mut self) -> u32 {
        let mut loop_len = 0;
        let mut current_coord = self.start_coord.clone();
        let mut current_tile = self.discover_start_tile_type(&current_coord);
        let mut current_direction = pick_start_direction(&current_tile);

        loop {
            self.mark_loop_minitiles(&current_coord, &current_tile);

            self.move_one_tile(
                &mut current_coord,
                &mut current_tile,
                &mut current_direction,
            );
            loop_len += 1;

            if current_tile == TileType::Start {
                break;
            }
        }

        loop_len
    }

    fn mark_loop_minitiles(&mut self, coord: &Coord, tile: &TileType) {
        // Coordinates of mini-tiles corresponding to the tile of `coord` (without the 4 corners)
        let n = (coord.x * 3 + 1, coord.y * 3);
        let w = (coord.x * 3, coord.y * 3 + 1);
        let c = (coord.x * 3 + 1, coord.y * 3 + 1);
        let e = (coord.x * 3 + 2, coord.y * 3 + 1);
        let s = (coord.x * 3 + 1, coord.y * 3 + 2);

        match tile {
            TileType::NS => {
                for (x, y) in [n, c, s] {
                    self.mini_tiles[y][x] = MiniTileType::Loop;
                }
            },
            TileType::NE => {
                for (x, y) in [n, c, e] {
                    self.mini_tiles[y][x] = MiniTileType::Loop;
                }
            },
            TileType::NW => {
                for (x, y) in [n, c, w] {
                    self.mini_tiles[y][x] = MiniTileType::Loop;
                }
            },
            TileType::SE => {
                for (x, y) in [s, c, e] {
                    self.mini_tiles[y][x] = MiniTileType::Loop;
                }
            },
            TileType::SW => {
                for (x, y) in [s, c, w] {
                    self.mini_tiles[y][x] = MiniTileType::Loop;
                }
            },
            TileType::EW => {
                for (x, y) in [e, c, w] {
                    self.mini_tiles[y][x] = MiniTileType::Loop;
                }
            },
            _ => panic!("Unexpected tile in the loop"),
        }
    }

    /// Return the unvisited neighbors of the mini-tile
    fn minitile_neighbors(&self, coord: Coord) -> Vec<Coord> {
        let Coord { x, y } = coord;
        let mut neighbors = Vec::new();

        let (m, n) = (self.mini_tiles.len(), self.mini_tiles[0].len());

        // N
        if x != 0 {
            neighbors.push(Coord::from((x - 1, y)));
        }
        // S
        if x != n - 1 {
            neighbors.push(Coord::from((x + 1, y)));
        }
        // W
        if y != 0 {
            neighbors.push(Coord::from((x, y - 1)));
        }
        // E
        if y != m - 1 {
            neighbors.push(Coord::from((x, y + 1)));
        }

        neighbors.retain(|Coord { x, y }| self.mini_tiles[*y][*x] == MiniTileType::Unvisited);
        neighbors
    }

    /// Use a flood fill algorithm to detect all the mini-tiles that are outside the loop. Starts
    /// from the top-left corner, since that mini-tile is always on the outside (the corner
    /// mini-tiles are not part of the loop).
    fn detect_outside(&mut self) {
        let start = Coord::from((0, 0));
        let mut queue: VecDeque<Coord> = VecDeque::new();
        queue.push_back(start);

        while !queue.is_empty() {
            let coord = queue.pop_front();

            match coord {
                None => break,
                Some(coord) => {
                    self.mini_tiles[coord.y][coord.x] = MiniTileType::Outside;

                    let neighbors = self.minitile_neighbors(coord);
                    for Coord { x, y } in &neighbors {
                        self.mini_tiles[*y][*x] = MiniTileType::Visited;
                    }
                    queue.extend(neighbors);
                },
            }
        }
    }

    /// Count the number of central mini-tiles which are neither Loop nor Outside. This corresponds
    /// to the number of Inside tiles.
    fn count_inside_tiles(&self) -> u32 {
        let mut count: u32 = 0;
        let (m, n) = (self.tiles.len(), self.tiles[0].len());

        for i in 0..n {
            for j in 0..m {
                // Coordinate of the central minitile for the tile on (i, j)
                let (x, y) = (i * 3 + 1, j * 3 + 1);
                if self.mini_tiles[y][x] == MiniTileType::Unvisited {
                    count += 1;
                }
            }
        }

        count
    }
}

/// Pick a start direction from among the two possibilities. This choice is irrelevant.
fn pick_start_direction(start_tile: &TileType) -> Direction {
    match start_tile {
        TileType::NS | TileType::NE | TileType::NW => Direction::N,
        TileType::SE | TileType::SW => Direction::S,
        TileType::EW => Direction::E,
        _ => panic!("Unexpected TileType"),
    }
}

/// Return the pair:
/// - Longest distance from the start tile in the main loop. This is equal to half of the
/// length of the loop.
/// - Number of tiles inside of the main loop.
pub fn solve(maze: &mut Maze) -> (u32, u32) {
    let loop_length = maze.detect_loop();
    maze.detect_outside();
    let inside_tiles_count = maze.count_inside_tiles();

    let solution_part1 = loop_length / 2;
    let solution_part2 = inside_tiles_count;
    (solution_part1, solution_part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "};

        let mut maze = Maze::load_from_input(&input);
        let (result_part1, _) = solve(&mut maze);
        assert_eq!(result_part1, 4);
    }

    #[test]
    fn test2() {
        let input = indoc! {"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "};

        let mut maze = Maze::load_from_input(&input);
        let (result_part1, _) = solve(&mut maze);
        assert_eq!(result_part1, 8);
    }

    #[test]
    fn test3() {
        let input = indoc! {"
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        "};

        let mut maze = Maze::load_from_input(&input);
        let (_, result_part2) = solve(&mut maze);
        assert_eq!(result_part2, 4);
    }
    #[test]
    fn test4() {
        let input = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};

        let mut maze = Maze::load_from_input(&input);
        let (_, result_part2) = solve(&mut maze);
        assert_eq!(result_part2, 8);
    }
}
