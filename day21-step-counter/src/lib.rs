//! https://adventofcode.com/2023/day/21
//!
//! The input of this problem has a couple of properties which are important for the second part of
//! the problem:
//! 1) Most importantly: the line and the column of the start tile are empty (they have no rocks),
//!    as well as the entire border of the layout. This property is very important, because it
//!    gives a regularity to the distance of the same plot in different layouts, and allows us to
//!    use an efficient solution where we don't need to compute the distance of each plot in the
//!    reachable area of the expanded layout.
//! 2) Start tile is in the center of the layout. We don't take advantage of this property.
//! 3) The size of the layout is an odd number. We use this property in the solution for
//!    simplicity, but it would be very easy to account for the sitution where the layout size is
//!    an even number.
//!
//! Explanation of the solution for part 2:
//! 
//! We start by computing the distance of all plots in the 3x3 expanded layout. Next is explained
//! why we need the 3x3 grid.
//!
//! Looking at a single target plot in the initial layout, lets sketch the distance from the start
//! of the corresponding plots in the 3x3 grid of layouts (plots that have the same coordinate
//! within a layout). The layout is of size 17.
//!
//!          +----------+-----------+----------+
//!          |  NW: 41  |   N: 24   |  NE: 35  |
//!          +----------+-----------+----------+
//!          |   W: 24  | Center: 7 |   E: 18  |
//!          +----------+-----------+----------+
//!          |  SW: 33  |   S: 16   |  SE: 27  |
//!          +----------+-----------+----------+
//!  
//! We obviously have to compute the distance of the center plot using BFS. Due to the fact that
//! the plot will be closer to one corner of the layout that the others, we also have to compute
//! the distance from the plots in layouts in the opposite corner using BFS. For example, if the
//! plot is at coordinate (2, 2), i.e. close to the NW corner of the initial layout, we also need
//! to use BFS for the corresponding plots in the E, S, and SE layouts (like in the sketch above).
//!
//! For the remaining layouts, we can compute the distance of the plot as the min distance of the
//! plots of the neighbouring layouts plus the layout size. For example, we can compute the
//! distance of the plot in the N layout as: Center + size = 7 + 17 = 24. For another example, the
//! distance of plot in the SW layout: min(S, W) + size = min(16, 24) + 17 = 33. We only need to
//! account for the neighbouring layouts that are the closest to the Center, e.g. N only needs to
//! consider Center, SW needs to consider W and S.
//!
//! Depending on the position of the plot, we may need to run BFS in any of the initial 3x3
//! layouts, so we run BFS for the whole 3x3 grid. We don't need to run BFS for any of the
//! surrounding layouts, since we can just compute the plot distances based on the neighbouring
//! layouts.
//!
//! Layouts of reachable plots expand from the Center in circles. Well, actually they are squares
//! of increasing size, but we call them circles throughout the rest of the comments.
//!
//! Let's add the remaining 4 layouts of distance 2 from the Center layout:
//!
//!                       +-----------+
//!                       |  NN: 41   |
//!            +----------+-----------+----------+
//!            |  NW: 41  |   N: 24   |  NE: 35  |
//! +----------+----------+-----------+----------+----------+
//! |  WW: 41  |   W: 24  | Center: 7 |   E: 18  |  EE: 35  |
//! +----------+----------+-----------+----------+----------+
//!            |  SW: 33  |   S: 16   |  SE: 27  |
//!            +----------+-----------+----------+
//!                       |  SS: 33   |
//!                       +-----------+
//!
//! Here we can see how the layouts expand in circles. The first circle is the Center itself. The
//! second circle is the 4 layouts of distance 1 from the Center: N, E, S, W. Circle 2 has the edge
//! of size 2. The third circle is the 8 layouts of distance 2 from the Center: NN, NE, EE, SE, SS,
//! SW, WW, NW. Circle 3 has the edge of size 3. And so on...
//!
//! Starting from Circle 3, we can compute the distances of the plots in each following circle
//! simply by incrementing the min neighbor by layout size.
//!

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn north(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn south(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn east(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn west(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    plots: HashSet<Coord>,
    start_coord: Coord,
    size: i32,
}

impl Layout {
    pub fn load_from_input(input: &str) -> Layout {
        let mut plots: HashSet<Coord> = HashSet::new();
        let mut start_coord: Option<Coord> = None;

        let size = {
            let size_y = input.lines().count();
            let size_x = input.lines().next().unwrap().chars().count();
            assert!(
                size_x == size_y,
                "Assertion failed: Input is not a square grid"
            );

            size_x as i32
        };

        for (j, line) in input.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                // Skip rocks
                if ch == '#' {
                    continue;
                }

                let coord = Coord::from((i as i32, j as i32));

                if ch == 'S' {
                    start_coord = Some(coord);
                }

                plots.insert(coord);
            }
        }

        Layout {
            plots,
            start_coord: start_coord.expect("Missing start coordinate"),
            size,
        }
    }

    /// For a given coordinate, return it along with 8 copies of itself shifted by the layout size
    /// to each of the 8 directions around it.
    #[rustfmt::skip]
    fn expand_coord(&self, coord: Coord) -> [Coord; 9] {
        let Coord { x, y } = coord;

        [
            Coord::from((x - self.size, y - self.size)),
            Coord::from((x,             y - self.size)),
            Coord::from((x + self.size, y - self.size)),
            Coord::from((x - self.size, y)),
            Coord::from((x,             y)),
            Coord::from((x + self.size, y)),
            Coord::from((x - self.size, y + self.size)),
            Coord::from((x,             y + self.size)),
            Coord::from((x + self.size, y + self.size)),
        ]
    }

    fn expand_3x3(&self) -> Layout {
        // For each coordinate, add 8 more, one in each direction
        let coords_iter = self
            .plots
            .iter()
            .flat_map(|coord| self.expand_coord(*coord));

        Layout {
            plots: HashSet::from_iter(coords_iter),
            start_coord: self.start_coord,
            size: self.size,
        }
    }

    fn generate_neighbors(&self, coord: &Coord) -> Vec<Coord> {
        [coord.north(), coord.south(), coord.east(), coord.west()]
            .into_iter()
            .filter(|coord| self.plots.contains(&coord))
            .collect()
    }

    /// Return the mapping { coordinate -> distance from the start coordinate } for each plot in
    /// the layout.
    fn bfs_distances(&self) -> HashMap<Coord, u32> {
        let mut distances: HashMap<Coord, u32> = HashMap::new();
        let mut queue: VecDeque<(Coord, u32)> = VecDeque::new();
        let mut visited: HashSet<Coord> = HashSet::new();

        // Initialize the algorithm with the start tile
        queue.push_back((self.start_coord, 0));
        visited.insert(self.start_coord);

        while let Some((coord, distance)) = queue.pop_front() {
            distances.insert(coord, distance);

            for neighbor_coord in self.generate_neighbors(&coord) {
                if !visited.contains(&neighbor_coord) {
                    queue.push_back((neighbor_coord, distance + 1));
                    visited.insert(neighbor_coord);
                }
            }
        }

        distances
    }
}

/// Return true if `distance` is reachable in `steps` steps.
///
/// A plot is reachable in N steps if its distance D from the start coord: D <= N and N - D = 2A.
/// In other words, if we can reach the plot in D steps, then we can spend the even remainder of
/// steps going back and forth always returning to it on the even step.
fn is_reachable(distance: u32, steps: u32) -> bool {
    if distance > steps {
        return false;
    }
    (steps - distance) % 2 == 0
}

/// Return the number of plots reachable after N steps from the starting position.
fn count_reachable(layout: &Layout, steps: u32) -> u32 {
    let mut count: u32 = 0;

    for distance in layout.bfs_distances().into_values() {
        if is_reachable(distance, steps) {
            count += 1;
        }
    }
    count
}

/// Return the number of plots reachable after N steps from the starting position if the layout
/// infinitely repeats in each direction.
///
/// We count reachable plots by going through the plots of the initial central layout, and for each
/// plot of the central layout, expanding it to all of its copies in the infinite layouts and
/// counting those that are reachable among them, before moving on the next plot of the central
/// layout.
fn count_reachable_infinite(layout: &Layout, steps: u32) -> u64 {
    // Calculate distance from the start for all plots in the 3x3 expanded layout
    let distances_by_coord = layout.expand_3x3().bfs_distances();

    let layout_size = layout.size as u32;
    let mut count: u64 = 0;

    for &coord in layout.plots.iter() {
        // Distances for the 9 plots in the 3x3 expanded layout for the coord
        let [nw, n, ne, w, center, e, sw, s, se] = layout
            .expand_coord(coord)
            .map(|coord| distances_by_coord.get(&coord).copied().unwrap_or(u32::MAX));

        // Skip unreachable plots (i.e. those surrounded with rocks)
        if center == u32::MAX {
            continue;
        }

        // Distances for the 4 plots of distance 2 from the center in each cardinal direction
        let [nn, ss, ww, ee] = [n, s, w, e].map(|distance| distance + layout_size);

        let circle3 = [nn, ne, ee, se, ss, sw, ww, nw];

        // Add the count of reachable plots in the first 3 circles
        for dist in [center, n, e, s, w].iter().chain(&circle3) {
            if is_reachable(*dist, steps) {
                count += 1;
            }
        }

        // Plots switch reachablity from one circle to the next, meaning that if a plot is
        // reachable in circle i, then it is not reachable in circle i+1. The reason is that in the
        // given input the size of the layout is odd, therefore the distance changes parity from
        // one circle to the next. When we combine this with the reachability condition that
        // `(steps - distance)` must be even, we come to the conclusion that plots switch
        // reachablity from one circle to the next. We mark the whole circle as reachable or not,
        // and skip the unreachable ones in the loop below.
        //
        // TODO: If the size of the input were even, plots would have the same reachability in each
        // circle, since the parity of the distance would not change between circles. Implement the
        // general case where the size of the input can be either odd or even.
        let circle3_reachable = circle3.iter().any(|dist| is_reachable(*dist, steps));
        let mut circle_reachable = !circle3_reachable;

        // Add the count of reachable plots from the fourth to the last reachable circle. We track
        // the circle id and the min and max distance in a circle. Going into each next circle both
        // of these distances increase by the layout size.
        let mut circle: u32 = 4;
        let mut min_distance = *circle3.iter().min().unwrap();
        let mut max_distance = *circle3.iter().max().unwrap();
        loop {
            min_distance += layout_size;
            max_distance += layout_size;

            if min_distance > steps {
                break;
            }

            // In circles 4..N-1, all plots are reachable in the given number of steps
            // `(max_distance <= steps)`. We just add all the plots in these circles.
            //
            // In the final circle, however, some plots will be reachable and some will not,
            // because their distance will be larger than the number of steps.
            if circle_reachable {
                if max_distance <= steps {
                    let circle_size = 4 * (circle - 1);
                    count += circle_size as u64;
                } else {
                    // Corners of the final circle: Each corner of the final circle has the
                    // shortest path to the corresponding corner of circle 3. Therefore, its
                    // distance can be computed as the distance of the corresponding circle 3
                    // corner increased by layout size times the number of circles.
                    for distance in [nn, ss, ww, ee] {
                        let distance = distance + layout_size * (circle - 3);
                        if distance <= steps {
                            count += 1;
                        }
                    }

                    // Sides of the final circle: Each tile on the side of the final circle has the
                    // shortest path to the tile on the corresponding side of circle 3. We compute
                    // the distance the same way as for corners. Each side of the final circle has
                    // `(circle - 2)` tiles.
                    for distance in [nw, ne, sw, se] {
                        let distance = distance + layout_size * (circle - 3);
                        if distance <= steps {
                            count += circle as u64 - 2;
                        }
                    }
                }
            }

            circle += 1;
            circle_reachable = !circle_reachable;
        }
    }

    count
}

/// Return the number of plots reachable after 64 steps from the starting position.
pub fn solve_part1(layout: &Layout) -> u32 {
    count_reachable(layout, 64)
}

/// Return the number of plots reachable after 26501365 steps from the starting position in the
/// infinitely repeating layout.
pub fn solve_part2(layout: &Layout) -> u64 {
    count_reachable_infinite(layout, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {r"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "};

        let layout = Layout::load_from_input(&input);
        assert_eq!(count_reachable(&layout, 1), 2);
        assert_eq!(count_reachable(&layout, 2), 4);
        assert_eq!(count_reachable(&layout, 3), 6);
        assert_eq!(count_reachable(&layout, 6), 16);
        assert_eq!(count_reachable_infinite(&layout, 6), 16);
        assert_eq!(count_reachable_infinite(&layout, 10), 50);
    }

    #[test]
    fn test2() {
        let input = indoc! {r"
            .................
            ..#..............
            ...##........###.
            .............##..
            ..#....#.#.......
            .......#.........
            ......##.##......
            ...##.#.....#....
            ........S........
            ....#....###.#...
            ......#..#.#.....
            .....#.#..#......
            .#...............
            .#.....#.#....#..
            ...#.........#.#.
            ...........#..#..
            .................
        "};

        let layout = Layout::load_from_input(&input);
        assert_eq!(count_reachable_infinite(&layout, 7), 52);
        assert_eq!(count_reachable_infinite(&layout, 8), 68);
        assert_eq!(count_reachable_infinite(&layout, 25), 576);
        assert_eq!(count_reachable_infinite(&layout, 42), 1576);
        assert_eq!(count_reachable_infinite(&layout, 59), 3068);
        assert_eq!(count_reachable_infinite(&layout, 76), 5052);
        assert_eq!(count_reachable_infinite(&layout, 1180148), 1185525742508);
    }
}
