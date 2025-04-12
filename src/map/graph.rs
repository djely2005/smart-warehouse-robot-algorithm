use std::collections::HashMap;

use super::{edge::Edge, node::Node};

#[derive(Debug)]
pub struct Map {
    pub nodes: HashMap<&'static str, Node>,
    pub edges: Vec<Edge>,
}

impl Map {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();

        nodes.insert("home", Node::new("home", 0.0, 1.2));
        nodes.insert("tag1", Node::new("tag1", 0.675, 1.2));
        nodes.insert("tag2", Node::new("tag2", 1.2, 0.6));
        nodes.insert("tag3", Node::new("tag3", 0.9, 0.0));
        nodes.insert("tag4", Node::new("tag4", 0.45, 0.0));

        // Generate edges
        let mut edges = vec![];

        let pairs = [
            ("home", "tag1"),
            ("tag1", "tag2"),
            ("tag2", "tag3"),
            ("tag3", "tag4"),
            ("tag4", "home"),
        ];

        for (a, b) in pairs {
            let node_a = nodes.get(a).unwrap();
            let node_b = nodes.get(b).unwrap();
            let distance = node_a.distance(node_b);

            edges.push(Edge::new(a, b, distance));
            edges.push(Edge::new(b, a, distance)); // if bidirectional
        }

        Self { nodes, edges }
    }

    pub fn print_map(&self) {
        for edge in &self.edges {
            println!(
                "{} <--> {} = {:.2}m",
                edge.from, edge.to, edge.distance
            );
        }
    }
}
