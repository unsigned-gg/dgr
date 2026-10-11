// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Workflow matrix parsing and combination generator.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum MatrixError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("empty matrix")]
    Empty,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatrixConfig {
    #[serde(default)]
    pub dimensions: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub include: Vec<BTreeMap<String, String>>,
    #[serde(default)]
    pub exclude: Vec<BTreeMap<String, String>>,
}

/// Parse a matrix definition from TOML string.
pub fn parse_matrix(text: &str) -> Result<Vec<BTreeMap<String, String>>, MatrixError> {
    let cfg: MatrixConfig = toml::from_str(text)?;
    let mut combinations = vec![BTreeMap::new()];

    for (dim, values) in &cfg.dimensions {
        let mut next = Vec::new();
        for comb in combinations {
            for val in values {
                let mut c = comb.clone();
                c.insert(dim.clone(), val.clone());
                next.push(c);
            }
        }
        combinations = next;
    }

    // Append explicit includes
    for inc in cfg.include {
        if !combinations.contains(&inc) {
            combinations.push(inc);
        }
    }

    // Filter out excludes
    combinations.retain(|comb| {
        !cfg.exclude.iter().any(|ex| {
            ex.iter().all(|(k, v)| comb.get(k) == Some(v))
        })
    });

    Ok(combinations)
}

/// Load and parse a matrix from a file path.
pub fn load_matrix<P: AsRef<Path>>(path: P) -> Result<Vec<BTreeMap<String, String>>, MatrixError> {
    let text = std::fs::read_to_string(path)?;
    parse_matrix(&text)
}
