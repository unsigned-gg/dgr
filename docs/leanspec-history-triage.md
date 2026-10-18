# LeanSpec history triage (2026-10-18)

This applies the estate standard's "check previous commits" step (Linear EST-1169, parent CER-2890). It covers the whole first-parent history of `main` at `5fb4a38`, which is 6 commits, three of them merged PRs.

| Commit or PR | Title | Disposition | Reason |
|---|---|---|---|
| `b0f33db` (direct push) | feat(dgr): extract dagr runtime and flow representation kernel as dgr 0.4.0 | Retro-linked to SPEC-0001 | SPEC-0001 Horizon 1, "Mathematical Representation Kernel (Delivered — v0.4.0)", and REQ-0001-01 to 08 specify the kernel this commit delivered. |
| `dbcaf28` (direct push) | refactor(dgr): decouple from herdr and dagr into standalone mathematical DAG engine | Retro-linked to SPEC-0001 | SPEC-0001 describes the standalone engine that results from this decoupling. |
| PR 1 (`5dec57c`) | docs: add license badge and principles link per conformity directive | Exempt | Documentation only. |
| PR 3 (`524fdcf`) | chore(package): exclude dev workflows and scripts from crates.io tarball | Exempt | Packaging metadata only; no behaviour change. |
| PR 4 (`50eac9e`) | docs(specs): add LeanSpec SPEC-0001 initial architecture and roadmap | Linked | It adds SPEC-0001 itself. |
| `5fb4a38` (direct push) | docs(dgr): codify workflow execution as compiled microcode on silicon | Exempt | Documentation only. |

**Counts:** 1 linked, 2 retro-linked, 3 exempt, 0 needing a new retro spec.

**Note:** three of these commits reached `main` as direct pushes, not PRs. The branch rules on `main` block force-push and deletion, but they do not require a PR.
