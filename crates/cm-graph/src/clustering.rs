use cm_core::graph::{CrateGraph, NameIndex};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct Cluster {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub crate_count: usize,
    pub top_crates: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ClustersResponse {
    pub clusters: Vec<Cluster>,
    pub assignments: HashMap<String, u32>,
}

// Stable color palette for clusters
const CLUSTER_COLORS: &[&str] = &[
    "#3b82f6", "#6366f1", "#8b5cf6", "#ec4899", "#ef4444", "#f97316", "#f59e0b", "#84cc16",
    "#10b981", "#14b8a6", "#06b6d4", "#0ea5e9", "#a855f7", "#d946ef", "#e11d48", "#eab308",
    "#22c55e", "#2563eb", "#7c3aed", "#fb923c",
];

/// Category-based clustering: assign each crate a cluster based on its primary category.
pub fn compute_clusters(graph: &CrateGraph, _name_index: &NameIndex) -> ClustersResponse {
    let mut category_map: HashMap<String, Vec<(String, u64)>> = HashMap::new();

    // Group crates by their primary category
    for idx in graph.node_indices() {
        let node = &graph[idx];
        let category = node
            .categories
            .first()
            .cloned()
            .unwrap_or_else(|| "Uncategorized".to_string());

        category_map
            .entry(category)
            .or_default()
            .push((node.name.clone(), node.downloads));
    }

    // Sort categories by total downloads
    let mut sorted_cats: Vec<(String, Vec<(String, u64)>)> = category_map.into_iter().collect();
    sorted_cats.sort_by(|a, b| {
        let total_a: u64 = a.1.iter().map(|(_, d)| d).sum();
        let total_b: u64 = b.1.iter().map(|(_, d)| d).sum();
        total_b.cmp(&total_a)
    });

    let mut clusters = Vec::new();
    let mut assignments: HashMap<String, u32> = HashMap::new();

    for (i, (category, mut crates)) in sorted_cats.into_iter().enumerate() {
        let cluster_id = i as u32;
        let color = CLUSTER_COLORS[i % CLUSTER_COLORS.len()].to_string();

        // Sort crates by downloads descending
        crates.sort_by(|a, b| b.1.cmp(&a.1));

        let top_crates: Vec<String> = crates.iter().take(5).map(|(n, _)| n.clone()).collect();
        let crate_count = crates.len();

        // Assign cluster to each crate
        for (name, _) in &crates {
            assignments.insert(name.clone(), cluster_id);
        }

        clusters.push(Cluster {
            id: cluster_id,
            name: category,
            color,
            crate_count,
            top_crates,
        });
    }

    ClustersResponse {
        clusters,
        assignments,
    }
}
