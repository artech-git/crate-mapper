use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::path::Path;
use tracing::info;

// crates.io DB dumps use PostgreSQL boolean literals ("t"/"f") not Rust's "true"/"false"
fn deserialize_pg_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    match String::deserialize(d)?.as_str() {
        "t" | "true" | "1" => Ok(true),
        "f" | "false" | "0" => Ok(false),
        other => Err(serde::de::Error::custom(format!("invalid bool: {other}"))),
    }
}

#[derive(Debug, Deserialize)]
pub struct CrateRow {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionRow {
    pub id: u32,
    pub crate_id: u32,
    pub num: String,
    pub downloads: u64,
    #[serde(deserialize_with = "deserialize_pg_bool")]
    pub yanked: bool,
    pub features: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct DependencyRow {
    pub id: u32,
    pub version_id: u32,
    pub crate_id: u32,
    pub req: String,
    #[serde(deserialize_with = "deserialize_pg_bool")]
    pub optional: bool,
    pub kind: u32, // 0 = Normal, 1 = Build, 2 = Dev
}

#[derive(Debug, Deserialize)]
pub struct CategoryRow {
    pub id: u32,
    pub category: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct CrateCategoryRow {
    pub crate_id: u32,
    pub category_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct KeywordRow {
    pub id: u32,
    pub keyword: String,
}

#[derive(Debug, Deserialize)]
pub struct CrateKeywordRow {
    pub crate_id: u32,
    pub keyword_id: u32,
}

pub struct ParsedDump {
    pub crates: HashMap<u32, CrateRow>,
    pub latest_versions: HashMap<u32, VersionRow>,
    pub dependencies: HashMap<u32, Vec<DependencyRow>>,
    pub crate_categories: HashMap<u32, Vec<String>>,
    pub crate_keywords: HashMap<u32, Vec<String>>,
    pub total_downloads: HashMap<u32, u64>,
}

pub fn parse_dump(dump_path: &Path) -> Result<ParsedDump> {
    info!("Parsing database dump: {}", dump_path.display());

    let file = std::fs::File::open(dump_path).context("Failed to open dump file")?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    let mut crates: HashMap<u32, CrateRow> = HashMap::new();
    let mut versions: Vec<VersionRow> = Vec::new();
    let mut dependencies_raw: Vec<DependencyRow> = Vec::new();
    let mut categories: HashMap<u32, String> = HashMap::new();
    let mut keywords: HashMap<u32, String> = HashMap::new();
    let mut crate_categories_raw: Vec<CrateCategoryRow> = Vec::new();
    let mut crate_keywords_raw: Vec<CrateKeywordRow> = Vec::new();

    for entry in archive.entries().context("Failed to read archive entries")? {
        let entry = entry.context("Failed to read archive entry")?;
        let path = entry.path().context("Failed to get entry path")?;
        let path_str = path.to_string_lossy().to_string();

        if !path_str.ends_with(".csv") {
            continue;
        }

        let filename = path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();

        match filename.as_str() {
            "crates.csv" => {
                info!("Parsing crates.csv...");
                let mut rdr = csv::Reader::from_reader(entry);
                for result in rdr.deserialize() {
                    let row: CrateRow = result.context("Failed to parse crate row")?;
                    crates.insert(row.id, row);
                }
                info!("Parsed {} crates", crates.len());
            }
            "versions.csv" => {
                info!("Parsing versions.csv...");
                let mut rdr = csv::Reader::from_reader(entry);
                for result in rdr.deserialize() {
                    let row: VersionRow = result.context("Failed to parse version row")?;
                    versions.push(row);
                }
                info!("Parsed {} versions", versions.len());
            }
            "dependencies.csv" => {
                info!("Parsing dependencies.csv...");
                let mut rdr = csv::Reader::from_reader(entry);
                for result in rdr.deserialize() {
                    let row: DependencyRow = result.context("Failed to parse dependency row")?;
                    dependencies_raw.push(row);
                }
                info!("Parsed {} dependencies", dependencies_raw.len());
            }
            "categories.csv" => {
                info!("Parsing categories.csv...");
                let mut rdr = csv::Reader::from_reader(entry);
                for result in rdr.deserialize() {
                    let row: CategoryRow = result.context("Failed to parse category row")?;
                    categories.insert(row.id, row.category);
                }
                info!("Parsed {} categories", categories.len());
            }
            "keywords.csv" => {
                info!("Parsing keywords.csv...");
                let mut rdr = csv::Reader::from_reader(entry);
                for result in rdr.deserialize() {
                    let row: KeywordRow = result.context("Failed to parse keyword row")?;
                    keywords.insert(row.id, row.keyword);
                }
                info!("Parsed {} keywords", keywords.len());
            }
            "crates_categories.csv" => {
                let mut rdr = csv::Reader::from_reader(entry);
                for result in rdr.deserialize() {
                    let row: CrateCategoryRow =
                        result.context("Failed to parse crate_category row")?;
                    crate_categories_raw.push(row);
                }
            }
            "crates_keywords.csv" => {
                let mut rdr = csv::Reader::from_reader(entry);
                for result in rdr.deserialize() {
                    let row: CrateKeywordRow =
                        result.context("Failed to parse crate_keyword row")?;
                    crate_keywords_raw.push(row);
                }
            }
            _ => {}
        }
    }

    // Find latest non-yanked version per crate
    info!("Resolving latest versions...");
    let mut latest_versions: HashMap<u32, VersionRow> = HashMap::new();
    let mut total_downloads: HashMap<u32, u64> = HashMap::new();

    // Sort versions by created_at descending to pick latest
    let mut sorted_versions = versions;
    sorted_versions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    for v in sorted_versions {
        *total_downloads.entry(v.crate_id).or_insert(0) += v.downloads;
        if !latest_versions.contains_key(&v.crate_id) {
            if !v.yanked {
                latest_versions.insert(v.crate_id, v);
            }
        }
    }

    // Build dependencies map: version_id -> Vec<DependencyRow>
    let mut dependencies: HashMap<u32, Vec<DependencyRow>> = HashMap::new();
    for dep in dependencies_raw {
        dependencies.entry(dep.version_id).or_default().push(dep);
    }

    // Resolve category and keyword names per crate
    let mut crate_categories: HashMap<u32, Vec<String>> = HashMap::new();
    for cc in crate_categories_raw {
        if let Some(name) = categories.get(&cc.category_id) {
            crate_categories
                .entry(cc.crate_id)
                .or_default()
                .push(name.clone());
        }
    }

    let mut crate_keywords: HashMap<u32, Vec<String>> = HashMap::new();
    for ck in crate_keywords_raw {
        if let Some(name) = keywords.get(&ck.keyword_id) {
            crate_keywords
                .entry(ck.crate_id)
                .or_default()
                .push(name.clone());
        }
    }

    info!(
        "Parse complete: {} crates, {} versions resolved, {} dependency sets",
        crates.len(),
        latest_versions.len(),
        dependencies.len()
    );

    Ok(ParsedDump {
        crates,
        latest_versions,
        dependencies,
        crate_categories,
        crate_keywords,
        total_downloads,
    })
}
