use cm_core::graph::{CrateGraph, NameIndex};
use cm_core::models::{SubgraphEdge, SubgraphNode, SubgraphResponse};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::{HashMap, VecDeque};

const MAX_SUBGRAPH_NODES: usize = 500;

pub fn bfs_subgraph(
    graph: &CrateGraph,
    name_index: &NameIndex,
    center_name: &str,
    max_depth: u32,
) -> Option<SubgraphResponse> {
    
    let max_depth = max_depth.min(4);

    let start = *name_index.get(center_name)?;

    let mut visited: HashMap<NodeIndex, u32> = HashMap::new();
    let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::new();

    visited.insert(start, 0);
    queue.push_back((start, 0));

    // BFS outward (dependencies)
    while let Some((node, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }
        if visited.len() >= MAX_SUBGRAPH_NODES {
            break;
        }

        for edge in graph.edges_directed(node, Direction::Outgoing) {
            let target = edge.target();
            if !visited.contains_key(&target) {
                visited.insert(target, depth + 1);
                queue.push_back((target, depth + 1));
                if visited.len() >= MAX_SUBGRAPH_NODES {
                    break;
                }
            }
        }
    }

    // Also include direct dependents (1 hop incoming) for the center node
    for edge in graph.edges_directed(start, Direction::Incoming) {
        let source = edge.source();
        if !visited.contains_key(&source) && visited.len() < MAX_SUBGRAPH_NODES {
            visited.insert(source, 1);
        }
    }

    // Build response
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for (&node_idx, &depth) in &visited {
        let node = &graph[node_idx];
        nodes.push(SubgraphNode {
            id: node.name.clone(),
            name: node.name.clone(),
            description: node.description.clone(),
            downloads: node.downloads,
            version: node.version.clone(),
            depth,
            categories: node.categories.clone(),
        });
    }

    // Collect edges between visited nodes
    for &node_idx in visited.keys() {
        for edge in graph.edges_directed(node_idx, Direction::Outgoing) {
            if visited.contains_key(&edge.target()) {
                let weight = edge.weight();
                edges.push(SubgraphEdge {
                    source: graph[edge.source()].name.clone(),
                    target: graph[edge.target()].name.clone(),
                    kind: weight.kind.clone(),
                    optional: weight.optional,
                });
            }
        }
    }

    // Sort nodes: center first, then by downloads descending
    nodes.sort_by(|a, b| {
        if a.name == center_name {
            std::cmp::Ordering::Less
        } else if b.name == center_name {
            std::cmp::Ordering::Greater
        } else {
            b.downloads.cmp(&a.downloads)
        }
    });

    Some(SubgraphResponse {
        nodes,
        edges,
        center: center_name.to_string(),
        depth: max_depth,
    })
}
