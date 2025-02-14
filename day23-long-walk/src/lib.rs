//! https://adventofcode.com/2023/day/23
//!
//! The input layout has a special shape: All paths are 1 tile wide. All tiles on the path have two
//! neighbors, except for:
//! - the start and finish tiles, which have one neighbor
//! - inner tiles where the path forks, which have three or four neighbors
//!
//! Because of this shape of layout, we can transform it into a graph, where the inner fork tiles,
//! as well as the start and finish tile, are vertices of the graph, while the paths between these
//! vertices are the edges of the graph.
//!
//! In part1, the graph is a DAG. This is because of the slopes, which exist only around the inner
//! fork tiles, essentially defining directed edges by forcing the direction of movement at the
//! fork.
//!
//! In part2, however, we ignore the slopes, so the graph becomes an undirected cyclic graph.
//!
//! The solution uses a recursive DFS implementation to discover all the paths without loops in the
//! graph that start at the start vertex. The implementation remembers the longest path ending at
//! each vertex.
//!
//! This implementation is exponential in the depth of the DFS tree. However, since the problem
//! size is rather small (the input graph has 36 vertices after the transformation), this
//! implementation still works fast enough. Run it in release mode though.
//!
//! For part1, we could use a linear-time solution. However, there is no known linear time solution
//! for part2 (i.e. an undirected graph with cycles), so we use the same solution for both parts.
//!
//! The linear solution would work as such:
//! - Create a topological ordering of vertices using a marking recursive DFS algorithm
//! - For each vertex in the topological ordering, compute the longest path by taking the max of
//! paths from the incoming neighbors
//!
//! The problem description: https://en.wikipedia.org/wiki/Longest_path_problem
//! The linear time solution for DAGs: https://en.wikipedia.org/wiki/Longest_path_problem#Acyclic_graphs
//! The marking DFS algorithm: https://en.wikipedia.org/wiki/Topological_sorting#Depth-first_search

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn neighbors(&self) -> [Coord; 4] {
        [self.north(), self.east(), self.south(), self.west()]
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Coord {
        Coord { x, y }
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    path: HashSet<Coord>,
    slope_n: HashSet<Coord>,
    slope_s: HashSet<Coord>,
    slope_e: HashSet<Coord>,
    slope_w: HashSet<Coord>,
    start_coord: Coord,
    end_coord: Coord,
    size: i32,
}

impl Layout {
    pub fn load_from_input(input: &str) -> Layout {
        let mut path: HashSet<Coord> = HashSet::new();
        let mut slope_n: HashSet<Coord> = HashSet::new();
        let mut slope_s: HashSet<Coord> = HashSet::new();
        let mut slope_e: HashSet<Coord> = HashSet::new();
        let mut slope_w: HashSet<Coord> = HashSet::new();
        let mut start_coord: Option<Coord> = None;
        let mut end_coord: Option<Coord> = None;

        let size = {
            let size_y = input.lines().count();
            let size_x = input.lines().next().unwrap().chars().count();
            assert!(
                size_x == size_y,
                "Assertion failed: Input is not a square grid"
            );

            size_x
        };

        for (j, line) in input.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                // Skip forest tiles
                if ch == '#' {
                    continue;
                }

                let coord = Coord::from((i as i32, j as i32));
                path.insert(coord);

                if ch == '^' {
                    slope_n.insert(coord);
                } else if ch == 'v' {
                    slope_s.insert(coord);
                } else if ch == '>' {
                    slope_e.insert(coord);
                } else if ch == '<' {
                    slope_w.insert(coord);
                }

                if j == 0 {
                    start_coord = Some(coord);
                } else if j == size - 1 {
                    end_coord = Some(coord);
                }
            }
        }

        Layout {
            path,
            slope_n,
            slope_s,
            slope_e,
            slope_w,
            start_coord: start_coord.expect("Missing start coordinate"),
            end_coord: end_coord.expect("Missing end coordinate"),
            size: size as i32,
        }
    }

    fn get_edge_starts(&self, vertex: Coord) -> Vec<Coord> {
        let mut edge_starts = Vec::new();

        let n = vertex.north();
        if self.slope_n.contains(&n) {
            edge_starts.push(n);
        }

        let e = vertex.east();
        if self.slope_e.contains(&e) {
            edge_starts.push(e);
        }

        let s = vertex.south();
        if self.slope_s.contains(&s) {
            edge_starts.push(s);
        }

        let w = vertex.west();
        if self.slope_w.contains(&w) {
            edge_starts.push(w);
        }

        edge_starts
    }

    fn next_path_tile(&self, prev_path_tile: Coord, curr_path_tile: Coord) -> Vec<Coord> {
        curr_path_tile
            .neighbors()
            .into_iter()
            .filter(|coord| *coord != prev_path_tile)
            .filter(|coord| self.path.contains(coord))
            .collect()
    }

    /// Starting from `vertex` search in the direction of `edge_start` until we reach another
    /// vertex. Return the edge between the two vertices and all the path tiles on that edge.
    fn complete_edge(&self, vertex: Coord, edge_start: Coord) -> (Edge, HashSet<Coord>) {
        let mut edge_tiles = HashSet::from_iter(std::iter::once(vertex));
        let mut length = 0;

        let mut prev_path_tile = vertex;
        let mut curr_path_tile = edge_start;

        loop {
            edge_tiles.insert(curr_path_tile);
            length += 1;

            match self
                .next_path_tile(prev_path_tile, curr_path_tile)
                .as_slice()
            {
                // If there is only one tile available, we continue along the edge.
                &[next_path_tile] => {
                    prev_path_tile = curr_path_tile;
                    curr_path_tile = next_path_tile;
                },
                // Otherwise, we reached a vertex. Here, we have either two/three tiles available
                // (for inner vertices), or zero tile available (for the end vertex).
                _ => {
                    let edge = Edge {
                        from: vertex,
                        to: curr_path_tile,
                        length,
                    };
                    return (edge, edge_tiles);
                },
            }
        }
    }

    fn print_path(&self, graph: &Graph, path: &Vec<Coord>) {
        use colored::Colorize;

        let mut path_tiles: HashSet<Coord> = HashSet::new();

        for (u, v) in path.iter().zip(path.iter().skip(1)) {
            path_tiles.extend(graph.get_path_segment(u, v));
        }

        for j in 0..self.size {
            for i in 0..self.size {
                let coord = Coord::from((i as i32, j as i32));

                let symbol = if graph.vertices.contains(&coord) {
                    "X"
                } else if self.path.contains(&coord) {
                    "."
                } else {
                    "#"
                };

                if path_tiles.contains(&coord) {
                    print!("{}", symbol.on_blue());
                } else {
                    print!("{}", symbol);
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    from: Coord,
    to: Coord,
    length: u32,
}

impl Edge {
    pub fn reverse(self) -> Edge {
        Edge {
            from: self.to,
            to: self.from,
            length: self.length,
        }
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} --> {:?}", self.from, self.to)
    }
}

#[derive(Debug, Clone)]
struct Path {
    vertices: Vec<Coord>,
    length: u32,
}

impl Default for Path {
    fn default() -> Path {
        Path {
            vertices: Vec::new(),
            length: 0,
        }
    }
}

impl Path {
    pub fn new(vertex: Coord) -> Path {
        Path {
            vertices: vec![vertex],
            length: 0,
        }
    }

    pub fn contains(&self, vertex: &Coord) -> bool {
        self.vertices.contains(vertex)
    }

    pub fn extend(&self, vertex: Coord, length: u32) -> Path {
        let mut path = self.clone();
        path.vertices.push(vertex);
        path.length += length;
        path
    }
}

#[derive(Debug)]
struct Graph {
    vertices: HashSet<Coord>,
    adj_list: HashMap<Coord, Vec<Edge>>,
    path_segments: HashMap<(Coord, Coord), HashSet<Coord>>,
}

impl Graph {
    pub fn from_layout(layout: &Layout) -> Graph {
        let mut queue: VecDeque<(Coord, Coord)> = VecDeque::new();
        let mut vertices: HashSet<Coord> = HashSet::new();
        let mut adj_list: HashMap<Coord, Vec<Edge>> = HashMap::new();
        let mut path_segments: HashMap<(Coord, Coord), HashSet<Coord>> = HashMap::new();

        let start_vertex = layout.start_coord;
        vertices.insert(start_vertex);

        // The first edge always goes south from the start vertex
        queue.push_back((start_vertex, start_vertex.south()));

        while let Some((vertex, edge_start)) = queue.pop_front() {
            let (edge, path_segment) = layout.complete_edge(vertex, edge_start);
            let next_vertex = edge.to;

            // If we encounter this vertex for the first time, we expand from here
            if !vertices.contains(&next_vertex) {
                vertices.insert(next_vertex);

                for edge_start in layout.get_edge_starts(next_vertex) {
                    queue.push_back((next_vertex, edge_start));
                }
            }

            adj_list.entry(vertex).or_default().push(edge);
            path_segments.insert((edge.from, edge.to), path_segment);
        }

        Graph {
            vertices,
            adj_list,
            path_segments,
        }
    }

    pub fn to_undirected(self) -> Graph {
        let mut adj_list: HashMap<Coord, Vec<Edge>> = HashMap::new();

        for edge in self.adj_list.values().flatten() {
            adj_list.entry(edge.from).or_default().push(*edge);
            adj_list.entry(edge.to).or_default().push(edge.reverse());
        }

        Graph {
            vertices: self.vertices,
            adj_list,
            path_segments: self.path_segments,
        }
    }

    pub fn neighbors(&self, vertex: &Coord) -> Vec<(Coord, u32)> {
        self.adj_list
            .get(vertex)
            .map(|edges| edges.iter().map(|edge| (edge.to, edge.length)).collect())
            .unwrap_or_default()
    }

    pub fn get_path_segment(&self, vertex1: &Coord, vertex2: &Coord) -> &HashSet<Coord> {
        // The graph stores only one direction of the path segment, so we need to try both
        // directions for undirected graphs.
        let s1 = self.path_segments.get(&(*vertex1, *vertex2));
        let s2 = self.path_segments.get(&(*vertex2, *vertex1));
        s1.or(s2).unwrap()
    }

    /// Computes the longest paths from the given vertex to all other vertices in the graph.
    pub fn longest_paths(&self, vertex: Coord) -> HashMap<Coord, Path> {
        let mut longest_paths: HashMap<Coord, Path> = HashMap::new();
        self.dfs_longest_paths(vertex, Path::new(vertex), &mut longest_paths);

        longest_paths
    }

    /// Computes the longest paths from the given vertex to all other vertices in the graph using a
    /// recursive DFS implementation. Stores the paths in the `longest_paths` parameter.
    fn dfs_longest_paths(
        &self,
        vertex: Coord,
        path: Path,
        longest_paths: &mut HashMap<Coord, Path>,
    ) {
        let saved_path = longest_paths.entry(vertex).or_default();
        if saved_path.length < path.length {
            *saved_path = path.clone();
        }

        for (neighbor, edge_length) in self.neighbors(&vertex) {
            // Disallow loops
            if path.contains(&neighbor) {
                continue;
            }

            self.dfs_longest_paths(neighbor, path.extend(neighbor, edge_length), longest_paths);
        }
    }
}

/// Return the length of the longest path from the start coordinate to the end coordinate.
pub fn solve_part1(layout: &Layout) -> u32 {
    let graph = Graph::from_layout(layout);
    let longest_paths = graph.longest_paths(layout.start_coord);

    let path = &longest_paths[&layout.end_coord];
    layout.print_path(&graph, &path.vertices);
    path.length
}

/// Return the length of the longest path from the start coordinate to the end coordinate if the
/// slopes don't matter (i.e. if the graph is undirected).
pub fn solve_part2(layout: &Layout) -> u32 {
    let graph = Graph::from_layout(layout).to_undirected();
    let longest_paths = graph.longest_paths(layout.start_coord);

    let path = &longest_paths[&layout.end_coord];
    layout.print_path(&graph, &path.vertices);
    path.length
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let input = indoc! {r"
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        "};

        let layout = Layout::load_from_input(&input);
        let result = solve_part1(&layout);
        assert_eq!(result, 94);
        let result = solve_part2(&layout);
        assert_eq!(result, 154);
    }
}
