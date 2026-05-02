use cm_core::graph::{CrateGraph, NameIndex};
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ChainResponse {
    pub path: Vec<ChainNode>,
    pub length: usize,
    pub found: bool,
}

#[derive(Debug, Serialize)]
pub struct ChainNode {
    pub name: String,
    pub downloads: u64,
}

/// Find the shortest dependency chain between two crates.
/// Uses BFS (unweighted shortest path) following outgoing edges.
pub fn shortest_chain(
    graph: &CrateGraph,
    name_index: &NameIndex,
    source_name: &str,
    target_name: &str,
) -> ChainResponse {
    let source = match name_index.get(source_name) {
        Some(&idx) => idx,
        None => {
            return ChainResponse {
                path: vec![],
                length: 0,
                found: false,
            }
        }
    };

    let target = match name_index.get(target_name) {
        Some(&idx) => idx,
        None => {
            return ChainResponse {
                path: vec![],
                length: 0,
                found: false,
            }
        }
    };

    // BFS to find shortest path
    let predecessors = dijkstra(graph, source, Some(target), |_| 1u32);

    if !predecessors.contains_key(&target) {
        // Try reverse direction (target depends on source)
        let rev_predecessors = dijkstra(graph, target, Some(source), |_| 1u32);
        if rev_predecessors.contains_key(&source) {
            // Reconstruct reverse path using BFS with parent tracking
            return bfs_path(graph, target, source);
        }

        return ChainResponse {
            path: vec![],
            length: 0,
            found: false,
        };
    }

    bfs_path(graph, source, target)
}

fn bfs_path(graph: &CrateGraph, source: NodeIndex, target: NodeIndex) -> ChainResponse {
    use std::collections::VecDeque;

    let mut visited: HashMap<NodeIndex, Option<NodeIndex>> = HashMap::new();
    let mut queue: VecDeque<NodeIndex> = VecDeque::new();

    visited.insert(source, None);
    queue.push_back(source);

    while let Some(current) = queue.pop_front() {
        if current == target {
            break;
        }

        for edge in graph.edges_directed(current, Direction::Outgoing) {
            let next = edge.target();
            if !visited.contains_key(&next) {
                visited.insert(next, Some(current));
                queue.push_back(next);
            }
        }
    }

    if !visited.contains_key(&target) {
        return ChainResponse {
            path: vec![],
            length: 0,
            found: false,
        };
    }

    // Reconstruct path
    let mut path = Vec::new();
    let mut current = target;
    loop {
        let node = &graph[current];
        path.push(ChainNode {
            name: node.name.clone(),
            downloads: node.downloads,
        });
        match visited.get(&current) {
            Some(Some(prev)) => current = *prev,
            _ => break,
        }
    }
    path.reverse();

    let length = path.len().saturating_sub(1);
    ChainResponse {
        path,
        length,
        found: true,
    }
}
