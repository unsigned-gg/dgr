// SPDX-License-Identifier: Apache-2.0 OR MIT
//! `dgr`: Deterministic workflow orchestration engine and live DAG run-state runtime for autonomous agent swarms.

pub mod check;
pub mod contract;
pub mod flow;
pub mod herdr;
pub mod message;
pub mod model;
pub mod picker;
pub mod render;
pub mod scale;
pub mod select;
pub mod stats;
pub mod style;
pub mod view;

pub use check::{check, Finding, Level, Report};
pub use contract::{Attempt, Doc, Project, Run, Task};
pub use flow::{instantiate, FlowGraph, FlowTemplate, Instantiation};
