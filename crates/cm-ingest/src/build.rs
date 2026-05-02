use anyhow::Result;
use cm_core::graph::{CrateGraph, CrateNode, DepEdge, DepKind, NameIndex};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::path::Path;
use tracing::info;

use crate::parse::ParsedDump;

pub struct BuiltGraph {
    pub graph: CrateGraph,
    pub name_index: NameIndex,
}

pub fn build_graph(dump: ParsedDump) -> Result<BuiltGraph> {
    info!("Building dependency graph...");

    let mut graph = CrateGraph::new();
    let mut name_index: NameIndex = HashMap::new();
    let mut id_to_node: HashMap<u32, NodeIndex> = HashMap::new();

    // Add all crate nodes
    for (crate_id, krate) in &dump.crates {
        let version_info = dump.latest_versions.get(crate_id);
        let total_dl = dump.total_downloads.get(crate_id).copied().unwrap_or(0);

        let node = CrateNode {
            id: *crate_id,
            name: krate.name.clone(),
            description: krate.description.clone().unwrap_or_default(),
            downloads: total_dl,
            version: version_info.map(|v| v.num.clone()).unwrap_or_default(),
            repository: krate.repository.clone(),
            keywords: dump
                .crate_keywords
                .get(crate_id)
                .cloned()
                .unwrap_or_default(),
            categories: dump
                .crate_categories
                .get(crate_id)
                .cloned()
                .unwrap_or_default(),
            created_at: krate.created_at.clone(),
            updated_at: krate.updated_at.clone(),
        };

        let idx = graph.add_node(node);
        name_index.insert(krate.name.clone(), idx);
        id_to_node.insert(*crate_id, idx);
    }

    info!("Added {} nodes", graph.node_count());

    // Add dependency edges
    let mut edge_count = 0u64;
    for (crate_id, version) in &dump.latest_versions {
        let source_idx = match id_to_node.get(crate_id) {
            Some(idx) => *idx,
            None => continue,
        };

        if let Some(deps) = dump.dependencies.get(&version.id) {
            for dep in deps {
                let target_idx = match id_to_node.get(&dep.crate_id) {
                    Some(idx) => *idx,
                    None => continue,
                };

                let kind = match dep.kind {
                    0 => DepKind::Normal,
                    1 => DepKind::Build,
                    2 => DepKind::Dev,
                    _ => DepKind::Normal,
                };

                graph.add_edge(
                    source_idx,
                    target_idx,
                    DepEdge {
                        kind,
                        optional: dep.optional,
                        version_req: dep.req.clone(),
                    },
                );
                edge_count += 1;
            }
        }
    }

    info!(
        "Graph built: {} nodes, {} edges",
        graph.node_count(),
        edge_count
    );

    Ok(BuiltGraph { graph, name_index })
}

pub fn save_snapshot(built: &BuiltGraph, path: &Path) -> Result<()> {
    info!("Saving graph snapshot to {}", path.display());
    let encoded = bincode::serialize(&(&built.graph, &built.name_index))?;

    // Write to a sibling .tmp file first, then rename into place atomically.
    // A crash or kill between write and rename leaves the previous snapshot (or
    // no file) intact — the .tmp is never mistaken for a valid snapshot.
    let tmp_path = path.with_file_name(
        format!("{}.tmp", path.file_name().unwrap_or_default().to_string_lossy()),
    );
    std::fs::write(&tmp_path, &encoded)?;
    std::fs::rename(&tmp_path, path)?;

    info!("Snapshot saved: {:.1} MB", encoded.len() as f64 / 1_048_576.0);
    Ok(())
}

pub fn load_snapshot(path: &Path) -> Result<BuiltGraph> {
    info!("Loading graph snapshot from {}", path.display());
    let data = std::fs::read(path)?;
    let (graph, name_index): (CrateGraph, NameIndex) = bincode::deserialize(&data)?;

    // A zero-node graph means the file deserialized without a hard error but is
    // structurally empty — treat it as corrupt so the caller rebuilds.
    if graph.node_count() == 0 {
        anyhow::bail!("snapshot is empty (0 nodes) — likely corrupt or truncated");
    }

    info!(
        "Snapshot loaded: {} nodes, {} edges",
        graph.node_count(),
        graph.edge_count()
    );
    Ok(BuiltGraph { graph, name_index })
}
