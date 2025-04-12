mod map;

use map::graph::Map;

fn main() {
    let map = Map::new();

    let start = "home";
    let end = "tag3";

    match map.shortest_path(start, end) {
        Some((distance, path, edges)) => {
            println!(
                "Shortest path from {} to {} is {:.2}m via nodes: {:?}",
                start, end, distance, path
            );

            println!("Traversed edges:");
            for edge in edges {
                println!(
                    " - {} -> {} = {:.2}m",
                    edge.from, edge.to, edge.distance
                );
            }
        }
        None => println!("No path found from {} to {}", start, end),
    }
}
