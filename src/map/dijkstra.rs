use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::map::edge::Edge;
use super::graph::Map;

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    node: &'static str,
}

// For BinaryHeap (min-heap with reversed ordering)
impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    pub fn shortest_path(
        &self,
        start: &'static str,
        goal: &'static str,
    ) -> Option<(f64, Vec<&'static str>, Vec<&Edge>)> {
        let mut dist: HashMap<&'static str, f64> = HashMap::new();
        let mut prev: HashMap<&'static str, &'static str> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for node_id in self.nodes.keys() {
            dist.insert(*node_id, f64::INFINITY);
        }

        dist.insert(start, 0.0);
        heap.push(State { cost: 0.0, node: start });

        while let Some(State { cost, node }) = heap.pop() {
            if node == goal {
                // Reconstruct node path
                let mut path = vec![goal];
                let mut current = goal;

                while let Some(&p) = prev.get(current) {
                    path.push(p);
                    current = p;
                }

                path.reverse();

                // Reconstruct edge path
                let mut edge_path = vec![];
                for pair in path.windows(2) {
                    let from = pair[0];
                    let to = pair[1];
                    if let Some(edge) = self.edges.iter().find(|e| e.from == from && e.to == to) {
                        edge_path.push(edge);
                    }
                }

                return Some((cost, path, edge_path));
            }

            if cost > dist[&node] {
                continue;
            }

            for edge in self.edges.iter().filter(|e| e.from == node) {
                let next = edge.to;
                let next_cost = cost + edge.distance;

                if next_cost < dist[&next] {
                    dist.insert(next, next_cost);
                    prev.insert(next, node);
                    heap.push(State { cost: next_cost, node: next });
                }
            }
        }

        None
    }

}
