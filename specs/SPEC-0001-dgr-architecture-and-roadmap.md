---
status: in-progress
created: 2026-10-12
tags: [dgr, dag, kahn, topological-sort, world-model, trajectories, roadmap, cpm]
priority: critical
scope: dgr-engine
---
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# SPEC-0001: Dgr Mathematical DAG Engine, Kahn Compilation, and Multi-Horizon Roadmap

> **Status**: in-progress · **Priority**: critical · **Created**: 2026-10-12

## 1. Status & Metadata

- **Document ID:** SPEC-0001
- **Title:** Dgr Mathematical DAG Engine, Kahn Compilation, and Multi-Horizon Roadmap
- **Status:** In-Progress & Living
- **Authors:** Volva <volva@loom.unsigned.gg>, Mimir <mimir@unsigned.gg>
- **Champion Inspiration:** Carl Lerche, Aaron Turon, Leslie Lamport, Arthur Kahn
- **Target System:** `dgr` (`unsigned-gg/dgr`, binary `dgr`, crate `dgr`)
- **License:** CC-BY-4.0 / Apache-2.0 OR MIT
- **Predecessor Contract:** [`CONTRACT.md`](../CONTRACT.md) (v3)

---

## 2. Problem Statement

Autonomous multi-agent swarms fail when workflow representation conflates task definition with execution attempts. Conventional task management frameworks treat pipelines as flat queues or linear lists. When retries, failures, and fan-in review gates occur, naive systems rewrite history or lose causal provenance.

Existing workflow engines exhibit four critical architectural flaws:
1. **Conflation of Task and Attempt:** A retry mutates the original task backward, destroying execution history and duration metrics.
2. **Missing Dependency Promotion:** Downstream review steps or sub-tasks do not inherit or gate the parent dependency chain, introducing race conditions.
3. **Heavyweight Runtime Coupling:** Workflow visualizers depend on distributed state stores, external daemons, or heavy web frameworks rather than pure mathematical models.
4. **Lack of Convergence Proofs:** Iterative agent loops lack formal mathematical streak boundaries, causing non-terminating loops and resource exhaustion.

`dgr` resolves these problems. It operates as a **pure mathematical representation kernel and live state engine** in Rust. It decouples work intent from runtime settlement, enforces DAG acyclicity, compiles topological attempt projections via Kahn's algorithm, and delivers instant terminal visibility across arbitrary screen geometries.

---

## 3. Mathematical Foundations & Operational Invariants

```
   ┌──────────────────────────────────────────────────────────────┐
   │                  Invariants of the DAG Engine                │
   ├──────────────────────────────┬───────────────────────────────┤
   │ 1. Strict Acyclicity         │ ∀ v ∈ V, v ∉ Descendants(v)   │
   │ 2. Transitive Minimality     │ G_min = TransitiveReduction(G)│
   │ 3. Projection Monotonicity   │ State(Task) = f(Attempts)     │
   │ 4. Evidence Non-Repudiation  │ Settlement ∈ {◆, ◇, ≈, !}     │
   │ 5. Finite Loop Convergence   │ Streak(Loop) ≤ N_limit        │
   │ 6. Zero-Allocation Domain    │ Traversal in O(1) alloc       │
   └──────────────────────────────┴───────────────────────────────┘
```

### Invariant 1: Strict DAG Acyclicity (Topological Invariant)
Let $G = (V, E)$ be the directed workflow graph where $V$ represents tasks and gates, and $E$ represents declared causal dependencies (`deps` and gate `inputs`).
The graph must remain strictly acyclic:
$$\forall v \in V, \quad v \notin \text{Descendants}(v)$$
Acyclicity is verified via Kahn's topological sort algorithm during validation. Any cycle triggers a deterministic validation error (`E122`).

### Invariant 2: Transitive Reduction Minimality
Let $G = (V, E)$ be a directed acyclic graph. Its transitive reduction $G_{\text{red}} = (V, E_{\text{red}})$ is the unique minimal graph possessing identical reachability:
$$\text{Reach}(G_{\text{red}}) = \text{Reach}(G) \quad \land \quad \forall (u, v) \in E_{\text{red}}, \nexists \text{ path } u \rightsquigarrow v \text{ of length } > 1$$
Visual rendering and critical path calculations operate on the transitive reduction to eliminate visual clutter and redundant constraint tracking.

### Invariant 3: State Projection Monotonicity
A task possesses stable identity across multiple execution attempts. The task state $S(T)$ is a deterministic mathematical projection over its ordered attempt sequence $A(T) = [a_1, a_2, \dots, a_k]$:
$$S(T) = \mathcal{P}(A(T))$$
Projections follow strict monotonicity:
- Retries append new attempts with explicit backward causal links (`sent_back`, `gate_failed`, `followup`, `superseded`).
- Completed work cannot transition to `queued` without creating a new attempt.
- Violations of the projection table trigger validation error `E150`.

### Invariant 4: Evidence Tier Non-Repudiation
Completion claims are partitioned by cryptographic and structural verification rigor:
$$\text{Tier}(a) \in \{\text{verified } (◆), \text{reported } (◇), \text{heuristic } (≈), \text{asserted } (!)\}$$
- `◆ verified`: Mechanically verified via exit code 0, test suite execution, or cryptographic commit hash.
- `◇ reported`: Structured typed result envelope returned by an authenticated agent.
- `≈ heuristic`: Inferred from runtime signals, timeouts, or process termination.
- `! asserted`: Unverified prose assertion.
The renderer never silently promotes an unverified claim.

### Invariant 5: Finite Loop Convergence
Iterative loops generated by policy rules (`futures[]`) must declare finite convergence bounds:
$$\text{Streak}(P) \le N_{\text{max}} \quad (N_{\text{max}} \in \mathbb{N}^+)$$
Future stubs render in dotted notation (`├┄ ○`) and materialize into concrete tasks only when their triggering condition (`on: pass` or `on: fail`) occurs. Infinite cycles without monotonic termination bounds are rejected by contract rule `E164`.

### Invariant 6: Zero-Allocation Domain Core
Core graph algorithms, topological sorting, and CPM forward/backward passes execute within pre-allocated arena structures without secondary heap allocations or filesystem I/O.

---

## 4. Architecture & Interface

```
                    ┌────────────────────────────┐
                    │ Declarative Flow Template  │
                    │   (template.toml + matrix) │
                    └─────────────┬──────────────┘
                                  │ instantiate
                                  ▼
                    ┌────────────────────────────┐
                    │      FlowGraph Engine      │
                    │   (src/flow/instantiate.rs)│
                    └─────────────┬──────────────┘
                                  │ compiles to
                                  ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   dgr Run-State Document (CONTRACT v3)                 │
│                                                                        │
│   ┌───────────────┐     ┌───────────────┐     ┌───────────────────┐    │
│   │  Projects[]   │     │    Tasks[]    │     │    Attempts[]     │    │
│   │(Visual Scopes)│     │(Work Identity)│     │(Execution Records)│    │
│   └───────────────┘     └───────────────┘     └───────────────────┘    │
└───────────────┬────────────────────────────────────────┬───────────────┘
                │ watches                                │ checks
                ▼                                        ▼
┌───────────────────────────────┐        ┌───────────────────────────────┐
│     dgr view (Terminal TUI)   │        │   dgr check · dgr stats       │
│  • Responsive 20 to 4000 cols │        │  • Strict contract linter     │
│  • Subtree zoom & search (/)  │        │  • Critical Path Method (CPM) │
│  • OSC 52 clipboard export    │        │  • Slack & float analytics    │
│  • Contextual message (m)     │        │  • Sub-millisecond execution  │
└───────────────────────────────┘        └───────────────────────────────┘
```

### CLI Command Matrix

| Command | Status | Purpose | Output |
|---|---|---|---|
| `dgr check <run.json> [--strict] [--json]` | Delivered | Validates document against CONTRACT v3 rules | Exit 0 (clean), 1 (finding), 2 (IO) |
| `dgr view [run.json] [--snapshot] [--compact]` | Delivered | Interactive terminal DAG navigation | TUI or ANSI stdout frame |
| `dgr stats <run.json> [--json]` | Delivered | Computes critical path, WIP, and rework metrics | Formatted metrics table or JSON |
| `dgr flow check <flow.toml>` | In-Progress | Validates declarative workflow template | Syntax and parameter report |
| `dgr flow instantiate <flow.toml>` | In-Progress | Compiles template + parameters into `run.json` | Validated `run.json` document |
| `dgr --skill` | Delivered | Prints bundled agent producer instructions | Markdown agent skill document |

---

## 5. Multi-Horizon System Roadmap

```
Horizon 1 (v0.4.0 Delivered)
  └── Mathematical Representation Kernel, Contract v3, Terminal TUI & CPM Stats
          │
          ▼
Horizon 2 (v0.5.0 Active)
  └── Flow Graph Compilation, Matrix Instantiation & CLI `dgr flow` Integration
          │
          ▼
Horizon 3 (v0.6.0 Planned)
  └── Empirical World-Model Routing, Trajectory Analysis & Speculative Futures
          │
          ▼
Horizon 4 (v0.7.0 Planned)
  └── Streaming Delta Sync, ACP/NATS Transport & High-Throughput Headless Core
          │
          ▼
Horizon 5 (v1.0.0 Planned)
  └── Formal Lean 4 Proof Discharge, SMT Verification & crates.io Release
```

### Horizon 1: Mathematical Representation Kernel (Delivered — v0.4.0)
- Decoupled from `herdr` runtime into standalone, independent mathematical engine.
- Complete CONTRACT v3 specification with full backward compatibility for v1/v2 documents.
- Responsive interactive terminal TUI rendering down to 20 columns and up to 4,000 columns.
- Critical Path Method (CPM) flow analytics with slack calculation and naive ETA.
- Contextual operator message composer with immutable append journaling (`messages.jsonl`).
- Bundled self-onboarding producer skill (`dgr --skill`).
- Comprehensive regression suite with 156 unit and integration tests passing.

### Horizon 2: Flow Compilation & CLI Command Integration (Active — v0.5.0)
- **CLI Subcommand Promotion:** Expose `src/flow/` via `dgr flow` CLI subcommands (`check`, `instantiate`, `matrix`).
- **Combinatorial Matrix Expansion:** Parse multi-dimensional parameter matrices (`dimensions`, `include`, `exclude`) into parallel execution graphs.
- **Budget and Concurrency Gates:** Enforce flow-level limits (`max_in_flight`, `per_agent`) and token/cost caps (`max_tokens`, `max_cost_usd`) in compiled run plans.
- **Transitive Edge Pruning:** Implement zero-allocation transitive reduction preprocessing before graph rendering.

### Horizon 3: Empirical World-Model Routing & Trajectories (Planned — v0.6.0)
- **Empirical Rework Profiling:** Compute historical variance and failure probabilities from attempt causes (`sent_back`, `gate_failed`).
- **Dynamic Critical Path Re-weighting:** Update remaining duration estimates dynamically using empirical agent completion histories.
- **Trajectory Branch Prediction:** Evaluate future policy trees and dynamically select agent models or personas based on historical step success rates.
- **Speculative Task Pre-warming:** Issue advance warm-up signals to agent pools when upstream tasks enter high-probability completion phases.

### Horizon 4: Streaming Delta Sync & Headless Protocol (Planned — v0.7.0)
- **Delta Event Streaming:** Replace whole-file polling with RFC 6902 JSON Patch or append-only event log streaming.
- **Zed ACP & NATS Integration:** Expose headless streaming endpoints over Agent Communication Protocol (ACP) and NATS subjects.
- **High-Capacity In-Memory Ring Buffer:** Scale event retention to support swarms with 100,000+ tasks without memory degradation.
- **Headless Cockpit Daemon:** Run `dgr` as an ambient telemetry sidecar publishing continuous terminal frames and metrics to webhooks.

### Horizon 5: Formal Proof Discharge & Ecosystem Release (Planned — v1.0.0)
- **Lean 4 Mathematical Formalization:** Formalize Kahn topological compilation and state projection monotonicity in Lean 4 theorem files.
- **SMT-LIB2 Solver Integration:** Discharge workflow acyclicity and loop termination proofs directly into Z3.
- **Crates.io Namespace Release:** Publish clean, documented crates: `dgr` (CLI) and `dgr-core` (pure mathematical library).
- **Public Specification Conformity:** Validate complete conformance against LeanSpec and EARS standards.

---

## 6. Requirements (EARS)

- **[REQ-0001-01]** (Ubiquitous): The `dgr` engine shall enforce mathematical directed acyclic graph invariants across all tasks, dependencies, and fan-in gates.
- **[REQ-0001-02]** (Event-driven): WHEN `dgr check` executes against a run document, the validator shall verify all structural rules and return exit code 0 for clean runs or exit code 1 when findings occur.
- **[REQ-0001-03]** (State-driven): WHILE a task has multiple attempts, the projection engine shall compute task state monotonically from attempt records without mutating historical attempts.
- **[REQ-0001-04]** (Unwanted behavior): IF a cycle exists in declared dependencies or gate inputs, THEN the engine shall abort topological sorting and emit error `E122` identifying the cycle path.
- **[REQ-0001-05]** (Event-driven): WHEN a terminal window resize event occurs, `dgr view` shall recompute visual geometry dynamically across all columns between 20 and 4,000.
- **[REQ-0001-06]** (Optional feature): WHERE flow template files specify combinatorial matrices, `dgr flow` shall generate orthogonal workflow permutations while filtering declared exclusions.
- **[REQ-0001-07]** (Ubiquitous): All file operations, graph traversals, and test suites shall execute under mandatory host throttling (`-j 2`).
- **[REQ-0001-08]** (State-driven): WHILE computing flow statistics, `dgr stats` shall calculate the critical path, total slack, rework frequency, and WIP counts from attempt timestamps.

---

## 7. Verification & Testing Discipline

1. **Host Throttling (`-j 2`):** Every `cargo` invocation must enforce `-j 2` to prevent memory exhaustion on multi-agent workstation `ceres`.
2. **Deterministic Test Suite:** All 156 unit, integration, and contract tests must execute cleanly:
   ```bash
   cargo test -j 2
   ```
3. **Contract Conformance:** The bundled reference scene [`samples/run.json`](../samples/run.json) must validate with zero warnings:
   ```bash
   cargo run -j 2 -- check samples/run.json --strict
   ```
4. **Self-Run Acceptance Demo:** The self-run verification artifact must pass strict validation:
   ```bash
   cargo run -j 2 -- check demos/selfrun/run.json --strict
   ```
5. **Zero-Warning Compiler Gate:** Compilation must complete with zero warnings under release flags:
   ```bash
   cargo check -j 2 --release
   ```
6. **Controlled Language (ASD-STE100):** Documentation and architecture specifications must adhere strictly to Systems Architect register with concise, active phrasing.
