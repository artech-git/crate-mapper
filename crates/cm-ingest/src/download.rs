use anyhow::{Context, Result};
use std::{io::{BufWriter, Write}, path::Path};
use tracing::info;
use futures::stream::StreamExt;

const DUMP_URL: &str = "https://static.crates.io/db-dump.tar.gz";

pub async fn download_dump(data_dir: &Path) -> Result<std::path::PathBuf> {
    let dump_path = data_dir.join("db-dump.tar.gz");

    if dump_path.exists() {
        let metadata = std::fs::metadata(&dump_path)?;
        let age = metadata.modified()?.elapsed().unwrap_or_default();
        if age < std::time::Duration::from_secs(24 * 60 * 60) {
            info!("DB dump is fresh ({}h old), skipping download", age.as_secs() / 3600);
            return Ok(dump_path);
        }
    }

    info!("Downloading crates.io database dump from {}", DUMP_URL);
    std::fs::create_dir_all(data_dir)?;

    let response = reqwest::Client::builder()
        .user_agent("CrateMapper/0.1.0 (https://github.com/cratemapper)")
        .build()?
        .get(DUMP_URL)
        .send()
        .await?;

    let total_size = response.content_length().unwrap_or(0);
    info!("Download size: {:.1} MB", total_size as f64 / 1_048_576.0);

    // Download to a temp file; rename to final path only on full success so a
    // failed/interrupted download never leaves a partial file that the age-check
    // would treat as fresh on the next startup.
    let tmp_path = data_dir.join("db-dump.tar.gz.tmp");
    let file = std::fs::File::create(&tmp_path).context("Failed to create temp download file")?;
    let mut buffer_writer = BufWriter::new(file);

    let mut bytes = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = bytes.next().await {
        let chunk = chunk.context("Error reading download chunk")?;
        downloaded += chunk.len() as u64;
        buffer_writer.write_all(&chunk).context("Failed to write download chunk")?;
    }
    buffer_writer.flush().context("Failed to flush download buffer")?;

    std::fs::rename(&tmp_path, &dump_path).context("Failed to finalize download file")?;
    info!("Download complete: {:.1} MB written to {}", downloaded as f64 / 1_048_576.0, dump_path.display());
    Ok(dump_path)
}
