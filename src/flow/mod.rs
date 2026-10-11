// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Declarative workflow flow templates -> FlowGraph -> dgr v3 `run.json`.

pub mod instantiate;
pub mod matrix;
pub mod template;
pub mod types;

pub use instantiate::{instantiate, FlowError, Instantiation};
pub use matrix::{load_matrix, parse_matrix, MatrixError};
pub use template::{load_template, BudgetTemplate, FlowMeta, FlowTemplate, LimitsTemplate, ParamSpec, StepTemplate};
pub use types::{AgentSelection, Budget, FlowGraph, GateKind, Limits, PathKind, RunnerKind, StepSpec};
