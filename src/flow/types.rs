// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Workflow flow graph, step specification, limits, and budget types.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A validated, instantiated flow graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowGraph {
    pub flow: String,
    pub params: BTreeMap<String, String>,
    pub steps: Vec<StepSpec>,
    #[serde(default)]
    pub limits: Limits,
    #[serde(default)]
    pub budget: Budget,
}

/// Budget caps for a flow run or individual step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Budget {
    /// Maximum tokens allowed. `None` = unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// Maximum cost in USD allowed. `None` = unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cost_usd: Option<f64>,
}

/// Concurrency limits for a flow run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Limits {
    /// Maximum number of steps that may run concurrently. `None` = unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_in_flight: Option<usize>,
    /// Per-agent-kind concurrency limits. Keys are agent names (e.g. "claude", "omp").
    #[serde(default)]
    pub per_agent: BTreeMap<String, usize>,
}

/// One step in a flow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepSpec {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub agent: AgentSelection,
    pub cwd: PathKind,
    pub prompt: String,
    pub deps: Vec<String>,
    pub inputs: Vec<String>,
    pub verify_cmd: Option<Vec<String>>,
    pub rounds_max: Option<u32>,
    pub on_fail: Option<String>,
    pub on_pass: Option<String>,
    pub gate: Option<GateKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<Budget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persona: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_cmd: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_max_wait_secs: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
    #[serde(default, skip_serializing_if = "RunnerKind::is_local")]
    pub runner: RunnerKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentSelection {
    Named(String),
    Auto,
    Unassigned,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PathKind {
    Project,
    Worktree,
    Abs(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GateKind {
    HeimdalApproval,
    OperatorApproval,
}

/// Where a step's attempts run.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunnerKind {
    /// On this host, through the local agent harness.
    #[default]
    Local,
    /// AX Task on Agent Substrate.
    AxTask(AxPlacement),
}

impl RunnerKind {
    pub fn is_local(&self) -> bool {
        matches!(self, RunnerKind::Local)
    }
}

/// Placement configuration for AX runner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxPlacement {
    pub command: Vec<String>,
    #[serde(default = "default_atespace")]
    pub atespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_wait_secs: Option<u64>,
}

fn default_atespace() -> String {
    "default".into()
}

pub fn is_read_only_kind(kind: &str) -> bool {
    matches!(kind, "test" | "review" | "plan" | "note")
}
