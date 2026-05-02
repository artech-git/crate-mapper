use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateNode {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub downloads: u64,
    pub version: String,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DepKind {
    Normal,
    Dev,
    Build,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepEdge {
    pub kind: DepKind,
    pub optional: bool,
    pub version_req: String,
}

pub type CrateGraph = DiGraph<CrateNode, DepEdge>;
pub type NameIndex = HashMap<String, NodeIndex>;

/*
    dynamic memory optimized graph node required for managing the nodes 
    in a order where memory is not impacted and is also performant at scale

    objectives: 
        fetch data from nodes
        
    
    functional requirments: 
        memory optimize
        supports infinite nodes fetching and saving
*/