// SPDX-License-Identifier: Apache-2.0 OR MIT
//! `dgr`: Mathematical and machine-sympathetic directed acyclic graph (DAG) execution runtime and live state engine in pure Rust.

pub mod check;
pub mod contract;
pub mod flow;
pub mod liveness;
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
