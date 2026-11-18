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

## 2026-11-16T11:20Z — First actual-curve parity result

### What was done (commands, files, features)

- The corrected source-bound Cargo harness at `4589d65ea6` exited zero:
  eight tests passed, including complete K13/K19/K23 insertion, collision
  and full-group parity. Raw output is in `curve-locked.stdout`,
  `curve-locked.stderr` and `curve-locked.meta`. The library release build
  took 2m58s; this is compilation, not a collector timing.
- K13/K19/K23 had 161/4,159/18,158 singleton pages and 43/314/340 spill
  pages. The candidate requested 11,332/163,844/2,129,924 bytes of
  directory plus reserved spill backing; the prior 16-word paged index
  requested 39,172/758,404/4,534,916 bytes. These are storage requests,
  not allocator RSS or speed measurements. [pcc:unverified]

### Decisions made (and why)

- Add a distinct synthetic control that fills all 16 valid slots of one
  page. The existing control inserted keys 1..15 into the first page and
  key 16 into the next, so its name overstated that coverage. Preserve
  this passing first run, source-pin the additional control, then rerun.

### Open threads (unfinished, pick up next time)

- Commit the additive control, rerun both test binaries, derive a bounded
  result record, and create the required visual report.

### Errors (what went wrong, how it was handled)

- The original synthetic test checked 15 rather than 16 occupied slots
  in one page. Actual-curve parity was unaffected; the control is being
  strengthened before the final source-bound result.

## 2026-11-16T11:26Z — Final source-bound parity and visual record

### What was done (commands, files, features)

- From source commit `c3ad96f431`, the final dependency-free unit binary
  passed 4/4 tests and the final Cargo library harness passed 9/9 tests.
  The complete K13/K19/K23 `INLINE_PARITY_JSON` records equal the earlier
  passing curve run. Final UTC seconds, raw stdout/stderr and process-level
  `time -l` counters are retained in `unit-final.*` and `curve-final.*`.
- `derive_results.py` asserts source and binary hashes, all fixed counts,
  prior paged-result identity, complete collision/group-check counts,
  page accounting and equality of the two passing curve records.
  `verify_storage.sh` separately checked the arithmetic with `jq` and
  exited zero.
- Wrote a bounded observation report, editable Graphviz diagram, SVG
  backing chart and two-page Typst PDF. Opened and visually checked both
  rendered PDF pages. [pcc:unverified]

### Decisions made (and why)

- Preserve the result as a standalone representation screen. It has no
  collector timing, process RSS or complete F4/F5/F6 comparison.
- Run the repository's required release library suite before publishing
  the branch. Keep its output distinct from the experiment records.

### Open threads (unfinished, pick up next time)

- Run the release library validation, inspect failures, finish source and
  artifact checks, commit and open a ready stacked PR.

### Errors (what went wrong, how it was handled)

- None in the final source-bound tests or report rendering.

## 2026-11-16T11:48Z — Release suite partial failure and fixture diagnosis

### What was done (commands, files, features)

- Ran `cargo test --release --lib -- --test-threads 4` using Rust 1.99.1,
  with stdout, stderr, UTC timestamps and `/usr/bin/time -l` counters in
  `lib-validation.*`. The release build completed in 8m36s; the test
  runner announced 6,567 tests. Before interruption, 2,796 tests had
  reported `ok` and five ecbench band/compare tests had reported `FAILED`.
  The run exited 101 after SIGINT to the test process at 11:44:17Z.
- Reran one failing test directly from the built release binary with
  `--exact ... --nocapture`. It failed in `band.rs:1191` because the
  artifact-managed `sessions/k20-s0/records.jsonl` was absent. The
  manifest lists its SHA-256 and 2,293,097-byte expected size.
- Checked Conductor before hydrating the fixture. T-6701 holds its exact
  path with exclusive write scope, so no hydration or fixture edit was
  attempted. `lib-validation-diagnostic.*` preserves the panic and
  command outcome. [pcc:unverified]

### Decisions made (and why)

- Classify the broad suite as incomplete and failing. The isolated
  diagnosis establishes the missing fixture for one failure only; do not
  infer that it explains all five.
- Preserve the focused 4/4 and 9/9 parity checks as the source-bound
  correctness evidence. They do not depend on the missing fixture.

### Open threads (unfinished, pick up next time)

- Once T-6701 releases the path, hydrate the artifact and rerun the
  affected band/compare tests. Complete a broad release suite in a
  separately recorded run if needed for a passing repository-wide gate.
- Finish record validation, commit and open the ready stacked PR.

### Errors (what went wrong, how it was handled)

- Five ecbench tests failed during the broad run. The single isolated
  diagnostic named a missing artifact; the held path prevented repair in
  this task. The red suite was interrupted after unrelated long tests
  sustained high CPU use; the output and stop were preserved.
