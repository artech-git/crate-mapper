use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrateMapperError {
    #[error("Crate not found: {0}")]
    CrateNotFound(String),

    #[error("Graph operation failed: {0}")]
    GraphError(String),

    #[error("Data ingestion failed: {0}")]
    IngestError(String),

    #[error("Invalid depth: {0} (max: 4)")]
    InvalidDepth(u32),
}
