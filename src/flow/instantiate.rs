// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Instantiation: workflow template + parameters -> FlowGraph + dgr v3 run document.

use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::json;
use sha2::{Digest, Sha256};

use super::template::FlowTemplate;
use super::types::{AgentSelection, FlowGraph, GateKind, PathKind, StepSpec};

#[derive(Debug)]
pub struct Instantiation {
    pub graph: FlowGraph,
    pub dgr_document: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("flow has no steps")]
    EmptyFlow,
    #[error("step id {0:?} invalid: {1}")]
    InvalidStepId(String, String),
    #[error("duplicate step id {0}")]
    DuplicateStepId(String),
    #[error("dependency {0} not found")]
    MissingDep(String),
    #[error("input {0} not found")]
    MissingInput(String),
    #[error("cycle in dependency graph")]
    Cycle,
    #[error("unknown param {0}")]
    UnknownParam(String),
    #[error("missing required param {0}")]
    MissingParam(String),
    #[error("gate step {0} missing gate field")]
    MissingGate(String),
    #[error("non-gate step {0} has gate field")]
    UnexpectedGate(String),
    #[error("verify_cmd must be non-empty")]
    EmptyVerifyCmd,
    #[error("step {0}: command must be non-empty")]
    EmptyCommand(String),
    #[error("dgr validation failed: {0}")]
    ValidationFailed(String),
}

/// Instantiate a workflow template with parameters into a validated DAG run document.
pub fn instantiate(
    template: &FlowTemplate,
    params: BTreeMap<String, String>,
) -> Result<Instantiation, FlowError> {
    if template.steps.is_empty() {
        return Err(FlowError::EmptyFlow);
    }

    // Check parameters
    for k in params.keys() {
        if !template.params.contains_key(k) {
            return Err(FlowError::UnknownParam(k.clone()));
        }
    }
    for (k, spec) in &template.params {
        if spec.required && spec.default.is_none() && !params.contains_key(k) {
            return Err(FlowError::MissingParam(k.clone()));
        }
    }

    let mut ids = HashMap::new();
    let mut seen = HashSet::new();
    for st in &template.steps {
        if !seen.insert(&st.id) {
            return Err(FlowError::DuplicateStepId(st.id.clone()));
        }
        ids.insert(st.id.clone(), st);
    }

    // Cycle detection
    for st in &template.steps {
        for dep in &st.deps {
            if dep == &st.id {
                return Err(FlowError::Cycle);
            }
            if !ids.contains_key(dep) {
                return Err(FlowError::MissingDep(dep.clone()));
            }
        }
    }

    // Render dgr v3 document
    let mut tasks = Vec::new();
    for st in &template.steps {
        let task_id = st.id.clone();
        let title = st.title.clone().unwrap_or_else(|| st.id.clone());
        let kind = st.kind.clone();
        let deps: Vec<serde_json::Value> = st
            .deps
            .iter()
            .map(|d| json!({ "task": d, "rule": "promote" }))
            .collect();

        tasks.push(json!({
            "id": task_id,
            "title": title,
            "kind": kind,
            "state": "queued",
            "owner": st.agent.clone().unwrap_or_else(|| "unassigned".to_string()),
            "deps": deps,
            "attempts": []
        }));
    }

    let run_hash = {
        let mut hasher = Sha256::new();
        hasher.update(template.flow.name.as_bytes());
        for (k, v) in &params {
            hasher.update(k.as_bytes());
            hasher.update(v.as_bytes());
        }
        format!("{:x}", hasher.finalize())[..12].to_string()
    };

    let dgr_document = json!({
        "dgr": 3,
        "run": {
            "id": format!("{}-{}", template.flow.name, run_hash),
            "title": template.flow.name.clone(),
            "state": "working"
        },
        "tasks": tasks
    });

    let graph = FlowGraph {
        flow: template.flow.name.clone(),
        params,
        steps: template
            .steps
            .iter()
            .map(|s| StepSpec {
                id: s.id.clone(),
                title: s.title.clone().unwrap_or_else(|| s.id.clone()),
                kind: s.kind.clone(),
                agent: match s.agent.as_deref() {
                    Some("auto") => AgentSelection::Auto,
                    Some("none") => AgentSelection::None,
                    Some(name) => AgentSelection::Named(name.to_string()),
                    None => AgentSelection::Unassigned,
                },
                cwd: match s.cwd.as_deref() {
                    Some("worktree") => PathKind::Worktree,
                    Some(path) => PathKind::Abs(path.to_string()),
                    None => PathKind::Project,
                },
                prompt: s.prompt.clone(),
                deps: s.deps.clone(),
                inputs: s.inputs.clone(),
                verify_cmd: s.verify_cmd.clone(),
                rounds_max: s.rounds_max,
                on_fail: s.on_fail.clone(),
                on_pass: s.on_pass.clone(),
                gate: s.gate.as_deref().map(|g| match g {
                    "operator" => GateKind::OperatorApproval,
                    _ => GateKind::HeimdalApproval,
                }),
                budget: None,
                persona: s.persona.clone(),
                ready_cmd: s.ready_cmd.clone(),
                ready_max_wait_secs: s.ready_max_wait_secs,
                timeout_secs: s.timeout_secs,
                runner: s.runner.clone().unwrap_or_default(),
            })
            .collect(),
        limits: Default::default(),
        budget: Default::default(),
    };

    Ok(Instantiation {
        graph,
        dgr_document,
    })
}
