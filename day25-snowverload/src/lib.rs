//! https://adventofcode.com/2023/day/25
//!
//! Compute the minimum cut of a graph, knowing that the size of the minimum cut is equal to 3.
//! <https://en.wikipedia.org/wiki/Minimum_cut>
//!
//! Input assumptions: All nodes in the input have a degree of at least 4. The min cut of size 3 is
//! unique in the input.
//!
//! Run Karger's algorithm (which is a randomized algorithm) until it returns with a min cut of
//! size 3. Karger's runs in O(|V|^2). It succeeds with polynomial probability.
//! <https://en.wikipedia.org/wiki/Karger%27s_algorithm>

use std::collections::HashMap;
use std::fmt;

use rand::{rngs::ThreadRng, Rng};

pub mod parser;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
struct Component {
    ident: [char; 3],
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.ident[0], self.ident[1], self.ident[2])
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    adj_list: HashMap<Component, Vec<Component>>,
}

impl Graph {
    pub fn to_undirected(self) -> Graph {
        let mut adj_list: HashMap<Component, Vec<Component>> = HashMap::new();

        for (node, neighbors) in self.adj_list.into_iter() {
            adj_list.entry(node).or_default().extend(&neighbors);
            for neighbor in neighbors {
                adj_list.entry(neighbor).or_default().push(node);
            }
        }

        Graph { adj_list }
    }

    fn random_edge(&self, rng: &mut ThreadRng) -> (Component, Component) {
        let nodes: Vec<Component> = self.adj_list.keys().copied().collect();

        let from_index = rng.gen_range(0..nodes.len());
        let from = nodes[from_index];

        let neighbors = &self.adj_list[&from];

        let to_index = rng.gen_range(0..neighbors.len());
        let to = neighbors[to_index];

        (from, to)
    }

    /// Contract the edge between nodes `from` and `to`. The edge itself is removed. Nodes `from`
    /// and `to` are replaced by a single node that keeps the name of `from`. All neighbors of `to`
    /// have `to` replaced by `from` in their adjacency lists.
    fn contract(&mut self, from: &Component, to: &Component) {
        let neighbors_of_from = self.adj_list.remove(from).unwrap();
        let neighbors_of_to = self.adj_list.remove(to).unwrap();

        let new_neighbors: Vec<Component> = neighbors_of_from
            .into_iter()
            .chain(neighbors_of_to)
            .filter(|n| n != to && n != from)
            .collect();

        // For each neighbor of the merged node, which we now refer to as `from`, we need to
        // replace `to` in their neighbor list with `from`.
        for node in new_neighbors.iter() {
            let neighbors_of_node = self.adj_list.get_mut(node).unwrap();
            neighbors_of_node.iter_mut().for_each(|nn| {
                if nn == to {
                    *nn = *from
                }
            });
        }

        // Insert the merged node into the adjacency list
        self.adj_list.insert(*from, new_neighbors);
    }
}

/// Modified Karger's algorithm: While the graph has more than 2 vertices, pick one edge at random
/// and contract the graph along that edge. The outcome is the cut of the graph along the edges
/// that connect the last 2 vertices.
///
/// Return the number of edges that define the cut, as well as the product of the sizes of the two
/// subgraphs defined by the cut.
///
/// In order to know the sizes of the subgraphs, we track the node sizes as we contract the graph;
/// i.e. the size of the merged node is equal to the sum of the sizes of the two nodes that formed
/// the merged node.
fn kargers(graph: &mut Graph) -> (u32, u32) {
    let mut rng = rand::thread_rng();

    let mut node_sizes: HashMap<Component, u32> =
        HashMap::from_iter(graph.adj_list.keys().map(|node| (*node, 1)));

    while graph.adj_list.len() > 2 {
        let (from, to) = graph.random_edge(&mut rng);
        graph.contract(&from, &to);
        *node_sizes.get_mut(&from).unwrap() += node_sizes[&to];
    }

    let [(node_a, neighbors_a), (node_b, neighbors_b)] = graph
        .adj_list
        .iter()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Expected two elements in adj_list");

    assert_eq!(neighbors_a.len(), neighbors_b.len());

    let result = node_sizes[node_a] * node_sizes[node_b];

    (neighbors_a.len() as u32, result)
}

/// Partition the graph along the min-cut into two subgraphs. Min-cut has three edges. Return the
/// product of the sizes of subgraphs.
///
/// Run Karger's algorithm until it returns with a cut of size 3 (because we know that is the size
/// of the min cut).
pub fn solve_part1(graph: &Graph) -> u32 {
    let mut iters = 0;
    loop {
        iters += 1;
        let (cut_length, result) = kargers(&mut graph.clone());

        if cut_length == 3 {
            println!("Finished in {} iterations of Karger's algorithm", iters);
            return result;
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
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        "};

        let graph = parser::parse_input(&input).unwrap();
        let result = solve_part1(&graph);
        assert_eq!(result, 54);
    }
}
