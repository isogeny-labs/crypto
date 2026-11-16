# Inline-singleton pair index worklog

The shared `docs/session-logs/2026-11-16.md` is reserved by Conductor
T-6720. This task keeps its timestamped notes here and will not edit that
held file. [pcc:unverified]

## 2026-11-16T11:10Z — Frozen representation screen

### What was done (commands, files, features)

- Read the primary rootfinding paper, current binary torsion builder,
  paged-index source and actual-curve parity result. Queried the shared KB;
  its 2026-11-14 refresh predates the current study, so local pinned source
  controls the exact mechanism. Checked Conductor: T-6708 still holds the
  production collector and ecbench adapter; T-6342 holds `.resume`.
- Read the current AWS monthly budget under account `104186811093`:
  USD 2,094.914 actual against USD 1,300 limit. No EKS submission was
  attempted. Claimed disjoint Conductor task T-6733 and created branch
  `codex/rootfind-inline-page-index-20261116` from `235001fbb8`.
- Froze `PROTOCOL.md` before code or result. [pcc:unverified]

### Decisions made (and why)

- Screen an inline singleton directory because the prior actual pair
  streams have only 57/327/344 keys beyond one per occupied 16-key page.
  This could change probe and allocation costs; it has no measured speed
  status. Leave the reserved source files alone.

### Open threads (unfinished, pick up next time)

- Commit the frozen protocol, implement the bounded index and exact parity
  checks, then run and report only at the tested scope.

### Errors (what went wrong, how it was handled)

- No implementation or run attempted at this freeze.

## 2026-11-16T11:17Z — Source implementation before execution

### What was done (commands, files, features)

- Implemented a 32-bit inline singleton directory, promoted spill pages,
  first-witness preservation, explicit bounds and source-local synthetic
  controls in `inline_page_index.rs`. Added a new exact K13/K19/K23
  integration test comparing each insertion with a hash map and the prior
  paged prototype, then the complete collision vector and group equations
  with the production torsion builder.
- Added a local Cargo library harness pointing at the new test source.
  Copied the pinned dependency lock and resolved its package entry offline.
  Rustfmt completed with no remaining format changes. No test has run yet.

### Decisions made (and why)

- Preallocate at most 512 spill pages initially and enforce the folded
  builder's `floor((a²+1)/2)` spill-page limit. A spill needs at least two
  retained keys, so the limit is a worst-case bound rather than an observed
  value. Record actual capacity separately from occupied spill pages.

### Open threads (unfinished, pick up next time)

- Pin this source in a commit, then execute the source-bound unit and
  actual-curve parity tests with raw output retention.

### Errors (what went wrong, how it was handled)

- None before execution.
