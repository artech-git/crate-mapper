use serde::{Deserialize, Serialize};

use crate::graph::DepKind;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubgraphNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub downloads: u64,
    pub version: String,
    pub depth: u32,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubgraphEdge {
    pub source: String,
    pub target: String,
    pub kind: DepKind,
    pub optional: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubgraphResponse {
    pub nodes: Vec<SubgraphNode>,
    pub edges: Vec<SubgraphEdge>,
    pub center: String,
    pub depth: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub name: String,
    pub description: String,
    pub downloads: u64,
    pub version: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepInfo {
    pub name: String,
    pub kind: DepKind,
    pub optional: bool,
    pub version_req: String,
    pub downloads: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateInfoResponse {
    pub name: String,
    pub description: String,
    pub downloads: u64,
    pub version: String,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub direct_deps: Vec<DepInfo>,
    pub direct_dependents: Vec<DepInfo>,
    pub dep_count: usize,
    pub dependent_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub nodes: usize,
    pub edges: usize,
}
