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

## 2026-11-16T11:15Z — Dependency-free unit result

### What was done (commands, files, features)

- Built the source committed at `9f5e09d0ea` using Rust 1.99.1 and ran its
  dependency-free unit binary. All three tests passed, including full K13
  key lookup, first-witness preservation on promotion and explicit failure
  controls. `unit.meta`, `unit-build.*` and `unit-test.*` retain UTC seconds,
  exits, stdout/stderr and `/usr/bin/time -l` process counters. Source SHA-256
  is `289e49654ec0343f344f3ce5c5367332913273a7891d546feefa4c62e1c83638`;
  binary SHA-256 is
  `7544b9bd4fb7e52f2ff782295254cd6c27aa8abf01477e08545725cbcea057a0`.

### Decisions made (and why)

- Proceed to the pinned actual-curve parity test on the unchanged source.
  The earlier `11:17Z` worklog heading was rounded forward before checking
  the clock; this entry records the accurate 11:15Z unit run window.

### Open threads (unfinished, pick up next time)

- Run the local Cargo harness, inspect exact K13/K19/K23 results and record
  both successful and failed test outputs.

### Errors (what went wrong, how it was handled)

- None in the unit run.

## 2026-11-16T11:16Z — Cargo lock admission correction

### What was done (commands, files, features)

- The first actual-curve harness command exited 101 before compilation:
  `--locked` found the copied lockfile's old harness package name.
  `curve.meta`, `curve.stdout` and `curve.stderr` retain that refusal.
- Replaced only the lockfile's harness package name with the new package
  name. A diff against the prior harness lock shows this is the sole
  change; `cargo metadata --locked --offline --no-deps` exits zero.

### Decisions made (and why)

- Commit the corrected lockfile before rerunning the unchanged parity
  test, preserving the original admission refusal separately.

### Open threads (unfinished, pick up next time)

- Run the same source-bound test after the lock correction and preserve
  its own command outputs.

### Errors (what went wrong, how it was handled)

- No parity assertions executed in the refused launch.
