<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# Architecture Specifications (LeanSpec)

> Specifications precede implementation. If a system's invariants cannot be
> articulated in plain technical English, compiling it is merely executing an
> unvetted guess.

This directory is the LeanSpec registry of record for `dgr`. Every architectural
decision, boundary rule, and verification gate is specified as an individual,
versioned contract.

| Spec | Title | Status | Priority |
|---|---|---|---|
| [SPEC-0001-dgr-architecture-and-roadmap.md](./SPEC-0001-dgr-architecture-and-roadmap.md) | Dgr Mathematical DAG Engine, Kahn Compilation, and Multi-Horizon Roadmap | in-progress | critical |

## Specification Lifecycle

1. **Draft:** Initial problem statement, mathematical bounds, and EARS requirements.
2. **In-Progress:** Active implementation and algorithmic verification in code.
3. **Approved:** Formal review passed; mathematical invariants and tests verified.
4. **Living:** Operational contract maintained alongside the running codebase.
5. **Superseded:** Replaced by a subsequent specification with explicit backward references.

## Methodology

Specifications adhere to **LeanSpec + EARS** (Easy Approach to Requirements Syntax)
under Controlled Language (ASD-STE100):
- Procedural sentences <= 20 words.
- Descriptive sentences <= 25 words.
- Invariants stated as mathematical formulas and explicit pre/post-conditions.
- Requirements classified by EARS patterns (Ubiquitous, Event-driven, State-driven, Unwanted behavior, Optional feature).
