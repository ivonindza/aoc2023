//! https://adventofcode.com/2023/day/17
//!
//! The idea of the solution is to implement Dijkstra's SSSP algorithm on the graph whose nodes are
//! defined by the combination of coordinate and orientation. For each coordinate, we have two
//! nodes, one that can be entered horizontally and another that can be entered vertically.
//!
//! The crucible alternates between horizontal and vertical movements of 1-3 tiles. Therefore in
//! our neighbour generation function, we model that by generating neighbour only in the
//! perpendicular orientation. E.g. if the current node is ((5, 5), horizontal), we generate the
//! neighbour only on the vertical axis: (5, 4), (5, 3), (5, 2), (5, 6), (5, 7), and (5, 8).

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    Horizontal,
    Vertical,
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
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Coord {
        Coord { x, y }
    }
}

/// Coordinates between 'from' and 'to', including 'to', but not 'from'. 'from' and 'to' must be on
/// the same line, otherwise the output is meaningless.
fn coords_between(from: Coord, to: Coord) -> Vec<Coord> {
    let mut coords = Vec::new();

    // Try both directions - one will be an empty range and thus a noop.
    for x in from.x..to.x {
        coords.push(Coord::from((x + 1, from.y)));
    }
    for x in to.x..from.x {
        coords.push(Coord::from((x, from.y)));
    }

    for y in from.y..to.y {
        coords.push(Coord::from((from.x, y + 1)));
    }
    for y in to.y..from.y {
        coords.push(Coord::from((from.x, y)));
    }

    coords
}

/// `TilePosition` contains the information needed to locate the tile. We have 2 tiles per
/// coordinate, one for each orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TilePosition {
    coord: Coord,
    orientation: Orientation,
}

impl TilePosition {
    pub fn horizontal(coord: Coord) -> TilePosition {
        TilePosition {
            coord,
            orientation: Orientation::Horizontal,
        }
    }

    pub fn vertical(coord: Coord) -> TilePosition {
        TilePosition {
            coord,
            orientation: Orientation::Vertical,
        }
    }
}

/// `Tile` contains the values associated with the tile as well as the mutable information that we
/// need to modify during the execution of the algorithm.
#[derive(Debug, Clone)]
struct Tile {
    cost: u32,
    path_cost: u32,
    prev: Option<TilePosition>,
    visited: bool,
}

impl Tile {
    pub fn heap_info(&self, position: TilePosition) -> TileHeapInfo {
        TileHeapInfo {
            path_cost: self.path_cost,
            position,
        }
    }
}

#[derive(Debug, Eq)]
struct TileHeapInfo {
    pub path_cost: u32,
    pub position: TilePosition,
}

/// Implement the Ord trait, so that we use the BinaryHeap as a min-heap, instead of the default
/// max-heap. We care about the 'path_cost' value for comparissons.
impl Ord for TileHeapInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering between self and other to invert the comparison.
        other.path_cost.cmp(&self.path_cost)
    }
}

impl PartialOrd for TileHeapInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TileHeapInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path_cost == other.path_cost
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    tiles: HashMap<TilePosition, Tile>,
    max_x: i32,
    max_y: i32,
    min_tiles: usize,
    max_tiles: usize,
}

impl Layout {
    pub fn load_from_input(input: &str) -> Layout {
        let mut tiles: HashMap<TilePosition, Tile> = HashMap::new();
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;

        for (j, line) in input.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                let cost = ch.to_digit(10).unwrap();
                let coord = Coord::from((i as i32, j as i32));

                let tile = Tile {
                    cost,
                    path_cost: u32::MAX,
                    prev: None,
                    visited: false,
                };
                let position = TilePosition::horizontal(coord);
                tiles.insert(position, tile.clone());
                let position = TilePosition::vertical(coord);
                tiles.insert(position, tile);

                max_x = i;
            }
            max_y = j;
        }

        Layout {
            tiles,
            max_x: max_x as i32 + 1,
            max_y: max_y as i32 + 1,
            min_tiles: 1,
            max_tiles: 1,
        }
    }

    fn print_path(&self, path: &Vec<Coord>) {
        use colored::Colorize;

        let path: HashSet<Coord> = path.iter().copied().collect();

        for j in 0..self.max_y {
            for i in 0..self.max_x {
                let coord = Coord::from((i, j));
                let position = TilePosition::horizontal(coord);
                let cost = self.tiles.get(&position).unwrap().cost;

                if path.contains(&coord) {
                    print!("{}", cost.to_string().blue());
                } else {
                    print!("{}", cost);
                }
            }
            println!()
        }
    }

    fn configure_steps(&mut self, min: usize, max: usize) {
        self.min_tiles = min;
        self.max_tiles = max;
    }

    fn min_tiles(&self) -> usize {
        self.min_tiles
    }

    fn max_tiles(&self) -> usize {
        self.max_tiles
    }

    fn generate_neighbors_in_direction(
        &self,
        mut coord: Coord,
        orientation: Orientation,
        direction: Direction,
    ) -> Vec<(TilePosition, u32)> {
        let mut neighbours = Vec::new();
        let mut cost: u32 = 0;

        // Skip the first `MIN_TILES - 1` tiles, but still accumulate their cost
        for _ in 1..self.min_tiles() {
            coord = coord.next(direction);
            let position = TilePosition { coord, orientation };

            match self.tiles.get(&position) {
                Some(tile) => {
                    cost += tile.cost;
                },
                None => return vec![],
            }
        }

        for _ in self.min_tiles()..=self.max_tiles() {
            coord = coord.next(direction);

            let position = TilePosition { coord, orientation };

            match self.tiles.get(&position) {
                Some(tile) => {
                    cost += tile.cost;
                    neighbours.push((position, cost))
                },
                None => break,
            }
        }
        neighbours
    }

    /// At each step we swap orientations and generate neighbours up to distance 3. E.g. if the
    /// current orientation is horizontal, we get the north and south neighbors.
    ///
    /// For each neighbor, returns the position and path cost from the tile at `position` to the
    /// neighbor (which is the sum of costs of tiles that are skipped plus the neighbor cost).
    fn generate_neighbors(&self, position: TilePosition) -> Vec<(TilePosition, u32)> {
        match position.orientation {
            Orientation::Horizontal => {
                let neighbors_n = self.generate_neighbors_in_direction(
                    position.coord,
                    Orientation::Vertical,
                    Direction::N,
                );
                let neighbors_s = self.generate_neighbors_in_direction(
                    position.coord,
                    Orientation::Vertical,
                    Direction::S,
                );
                neighbors_n.into_iter().chain(neighbors_s).collect()
            },
            Orientation::Vertical => {
                let neighbors_e = self.generate_neighbors_in_direction(
                    position.coord,
                    Orientation::Horizontal,
                    Direction::E,
                );
                let neighbors_w = self.generate_neighbors_in_direction(
                    position.coord,
                    Orientation::Horizontal,
                    Direction::W,
                );
                neighbors_e.into_iter().chain(neighbors_w).collect()
            },
        }
    }

    fn dijkstra(&mut self, start_coord: Coord) {
        let mut to_visit: BinaryHeap<TileHeapInfo> = BinaryHeap::new();

        // All path costs were initialized to infinity when creating Layout, so we only need to
        // change the start tiles path cost.
        for position in [
            TilePosition::horizontal(start_coord),
            TilePosition::vertical(start_coord),
        ] {
            let start_tile = self.tiles.get_mut(&position).unwrap();
            start_tile.path_cost = 0;
            to_visit.push(start_tile.heap_info(position));
        }

        while let Some(TileHeapInfo {
            path_cost,
            position,
        }) = to_visit.pop()
        {
            // Given that we may enter the same tile repeatedly into the min heap, we could have
            // already found a better path previously. We must check for this so as not to worsen
            // the path costs for neighbors.
            let tile = self.tiles.get_mut(&position).unwrap();
            if tile.path_cost < path_cost || (tile.path_cost == path_cost && tile.visited) {
                continue;
            }
            tile.visited = true;

            for (neighbor_position, cost_to_neighbor) in self.generate_neighbors(position) {
                let neighbor = self.tiles.get_mut(&neighbor_position).unwrap();

                let new_path_cost = path_cost + cost_to_neighbor;
                if new_path_cost <= neighbor.path_cost {
                    neighbor.path_cost = new_path_cost;
                    neighbor.prev = Some(position);

                    to_visit.push(neighbor.heap_info(neighbor_position));
                }
            }
        }
    }

    fn construct_path(&self, position: TilePosition) -> Vec<Coord> {
        let mut path_coords: Vec<Coord> = Vec::new();
        path_coords.push(position.coord);

        let mut tile = self.tiles.get(&position).unwrap();
        let mut last_coord = position.coord;

        while let Some(position) = tile.prev {
            path_coords.extend(coords_between(last_coord, position.coord));
            tile = self.tiles.get(&position).unwrap();
            last_coord = position.coord;
        }
        path_coords
    }
}

/// Finds the min-cost path from top-left corner to the bottom-right corner. Min and max steps in
/// the same direction are defined by `min_steps` and `max_steps` parameters. Also reconstruct the
/// path and print it.
fn solve(layout: &mut Layout, min_steps: usize, max_steps: usize) -> u32 {
    let start_coord = Coord::from((0, 0));
    let end_coord = Coord::from((layout.max_x - 1, layout.max_y - 1));

    layout.configure_steps(min_steps, max_steps);
    layout.dijkstra(start_coord);

    let res1 = layout.tiles.get(&TilePosition::horizontal(end_coord)).unwrap();
    let res2 = layout.tiles.get(&TilePosition::vertical(end_coord)).unwrap();

    if res1.path_cost < res2.path_cost {
        let path = layout.construct_path(TilePosition::horizontal(end_coord));
        layout.print_path(&path);
        res1.path_cost
    } else {
        let path = layout.construct_path(TilePosition::vertical(end_coord));
        layout.print_path(&path);
        res2.path_cost
    }
}

/// Finds the min-cost path from top-left corner to the bottom-right corner, while moving no more
/// than 3 tiles in the same direction.
pub fn solve_part1(layout: &Layout) -> u32 {
    solve(&mut layout.clone(), 1, 3)
}

/// Finds the min-cost path from top-left corner to the bottom-right corner, while moving at
/// minimum 4 tiles and at maximum 10 tiles in the same direction.
pub fn solve_part2(layout: &Layout) -> u32 {
    solve(&mut layout.clone(), 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {r"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};

        let layout = Layout::load_from_input(&input);
        let result = solve_part1(&layout);
        assert_eq!(result, 102);
        let result = solve_part2(&layout);
        assert_eq!(result, 94);
    }

    #[test]
    fn test2() {
        let input = indoc! {r"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "};

        let layout = Layout::load_from_input(&input);
        let result = solve_part2(&layout);
        assert_eq!(result, 71);
    }
}
