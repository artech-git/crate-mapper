use cm_core::graph::{CrateGraph, NameIndex};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use serde::Serialize;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Serialize)]
pub struct ImpactResponse {
    pub crate_name: String,
    pub direct_dependents: usize,
    pub total_impacted: usize,
    pub by_depth: Vec<DepthBucket>,
    pub top_dependents: Vec<ImpactedCrate>,
}

#[derive(Debug, Serialize)]
pub struct DepthBucket {
    pub depth: u32,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct ImpactedCrate {
    pub name: String,
    pub downloads: u64,
    pub depth: u32,
}

/// Calculate the blast radius of a crate being yanked/broken.
/// Uses reverse BFS (follows incoming edges = dependents).
pub fn impact_radius(
    graph: &CrateGraph,
    name_index: &NameIndex,
    crate_name: &str,
) -> Option<ImpactResponse> {
    let start = *name_index.get(crate_name)?;

    let mut visited: HashMap<NodeIndex, u32> = HashMap::new();
    let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::new();

    visited.insert(start, 0);
    queue.push_back((start, 0));

    // Reverse BFS - follow incoming edges (things that depend on us)
    while let Some((node, depth)) = queue.pop_front() {
        for edge in graph.edges_directed(node, Direction::Incoming) {
            let dependent = edge.source();
            if !visited.contains_key(&dependent) {
                visited.insert(dependent, depth + 1);
                queue.push_back((dependent, depth + 1));
            }
        }
    }

    // Remove the start node itself
    visited.remove(&start);

    // Build depth buckets
    let mut depth_counts: HashMap<u32, usize> = HashMap::new();
    for &depth in visited.values() {
        *depth_counts.entry(depth).or_insert(0) += 1;
    }

    let mut by_depth: Vec<DepthBucket> = depth_counts
        .into_iter()
        .map(|(depth, count)| DepthBucket { depth, count })
        .collect();
    by_depth.sort_by_key(|b| b.depth);

    // Top dependents by downloads
    let mut all_impacted: Vec<ImpactedCrate> = visited
        .iter()
        .map(|(&idx, &depth)| {
            let node = &graph[idx];
            ImpactedCrate {
                name: node.name.clone(),
                downloads: node.downloads,
                depth,
            }
        })
        .collect();
    all_impacted.sort_by(|a, b| b.downloads.cmp(&a.downloads));

    let direct_dependents = by_depth
        .first()
        .map(|b| if b.depth == 1 { b.count } else { 0 })
        .unwrap_or(0);

    let total_impacted = visited.len();
    let top_dependents: Vec<ImpactedCrate> = all_impacted.into_iter().take(20).collect();

    Some(ImpactResponse {
        crate_name: crate_name.to_string(),
        direct_dependents,
        total_impacted,
        by_depth,
        top_dependents,
    })
}
