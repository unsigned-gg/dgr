// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Flow template TOML schema and file loader.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use super::types::RunnerKind;

#[derive(Debug, Clone, Deserialize)]
pub struct FlowTemplate {
    pub flow: FlowMeta,
    #[serde(default)]
    pub params: BTreeMap<String, ParamSpec>,
    #[serde(default)]
    pub limits: LimitsTemplate,
    #[serde(default)]
    pub budget: BudgetTemplate,
    #[serde(rename = "step", default)]
    pub steps: Vec<StepTemplate>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BudgetTemplate {
    #[serde(default)]
    pub max_tokens: Option<u64>,
    #[serde(default)]
    pub max_cost_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LimitsTemplate {
    #[serde(default)]
    pub max_in_flight: Option<usize>,
    #[serde(default)]
    pub per_agent: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlowMeta {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParamSpec {
    #[serde(default)]
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StepTemplate {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub agent: Option<String>,
    pub cwd: Option<String>,
    pub prompt: String,
    #[serde(default)]
    pub deps: Vec<String>,
    #[serde(default)]
    pub inputs: Vec<String>,
    pub verify_cmd: Option<Vec<String>>,
    pub rounds_max: Option<u32>,
    pub on_fail: Option<String>,
    pub on_pass: Option<String>,
    pub gate: Option<String>,
    #[serde(default)]
    pub budget: Option<BudgetTemplate>,
    #[serde(default)]
    pub persona: Option<String>,
    #[serde(default)]
    pub ready_cmd: Option<Vec<String>>,
    #[serde(default)]
    pub ready_max_wait_secs: Option<u64>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub runner: Option<RunnerKind>,
}

pub fn load_template<P: AsRef<Path>>(path: P) -> Result<FlowTemplate, Box<dyn std::error::Error + Send + Sync>> {
    let text = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&text)?)
}
