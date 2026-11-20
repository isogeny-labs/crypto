# Inherited SAT progress-phase syntax blocks integrated validation

- **Status:** investigating
- **Kind:** implementation gap in the base branch
- **Affected work:** [algebraic IC publication](../research/ic_algebraic_crossover_20261023/PUBLISH_REVIEW.md), current PR #2655 (following merged #2550)
- **Next action owner:** upstream SAT implementation maintainer
- **Last updated:** 2026-10-26

## Problem

Earlier SAT syntax was corrected upstream and its later library gate timed out,
as recorded below. A different inherited delimiter in
`binary_semaev_chain_sat.rs` blocks the23:22current-base projected-query build
at533437155. Upstream main8eef12527 and externally updated PR head562f7075
already contain its fix. Incorporation and fresh source gates remain required;
there is no current-base example/library pass from the failed attempt.

## Evidence and uncertainty

- **Observed:** integrated commit `1d71daac7` merges PR head `b1ab8a4e5` and
  main `37f7615588fcb1866bf7863829660fffa21e1dac`. The isolated busy wrapper's
  `cargo test --release --lib` exits 101, reporting `sat.rs:3706:3`, with the
  unmatched `impl Solver` / `progress_phase` at lines 709/800. The
  [receipt](../research/ic_algebraic_crossover_20261023/pipeline-receipts/final-merged-library-gate-20261026/status.json)
  retains command, timestamps, source revision and raw-log hash. Its complete
  17-line compiler diagnostic is retained alongside the receipt.
- **Derived:** this file has Git blob
  `ca306e19654aeb0f92c9ed79352e20615a182d91` in the integrated tree, its main
  parent and freshly fetched main `942caf7e49edcebf1f157222c8a5a3b61a3d85ef`.
  `git diff origin/main -- src/cryptanalysis/sat.rs` is empty. This change
  does not introduce or modify the faulty file.
- **Missing capability:** a compiling upstream SAT implementation on the base.
- **What this evidence does not establish:** the older 4,635-test pass applies
  to the frozen narrow-constructor source, not this integrated head. This
  failure says nothing about the finite research results, and clearing it
  alone will not establish current-head CI, artifact delivery or full IC cost.

## What would unblock it

- [x] Incorporate the upstream SAT correction by a normal base merge.
- [ ] Run the release library suite and affected example tests on the new
  integrated source, retaining all outcomes and addressing task-caused failures.

These conditions unblock the source gate only; storage, review and research
obligations remain separate.

## Next discriminating step

- **Action:** report the exact unchanged blob on PR #2550 and wait for an
  upstream change to this file; do not patch unrelated SAT code or skip tests.
- **Inputs and controls:** the same integrated tree and required library command.
- **Execution limits:** the recorded 2,700-second outer timeout, one Cargo build
  job and two test threads. No repeated run is useful while the blob is unchanged.
- **Outcomes:** a changed base permits a fresh compile/test gate; a repeated
  unchanged blob leaves this problem open.
- **Recheck trigger:** a new upstream SAT revision, at the PR check-in cadence.

## 2026-10-26T23:22:02Z current-base recurrence

The integration receipt at
`research/ic_algebraic_crossover_20261023/pipeline-receipts/projected-query-integration-source-20261026/`
retains test/build exits101 for an unclosed impl/function delimiter at lines
304/628, reported at line1648. Git blob3803524139ef5382d85f835107339d66d5909a6d
is identical in integration533437155, pre-change mergefd0f39a53 and the then-local
main03a001c9b. This task added no change to that file. Default Clippy exits0;
the separate scoped Lean gate passes, while the dependent certificate check
is not run125 because the release build failed. These mixed outcomes are
reported individually, not called a successful integration gate.

The actual default-branch file now has blob428cd4d17de548a34bf7aaec25be35fc86b3b6fb
and a closing brace after verify_external_assignment. Native fetch confirms
main8eef1252714097e0b8279c0bf55b82ed9e49943d; the existing regularPR has been
normally updated to562f7075b4a6ad0a2c5de918c5d7e4c0a0d51d24, parentsda951869
and8eef1252. Continue from that updated head and rerun the affected gates,
preserving this failed attempt. Do not patch unrelated SAT code, skip tests,
change frozen scientific source or infer a mathematical obstruction.

## Resolution and history

Unresolved. Reported with blob evidence in PR comment 6421864767 on 2026-10-26.
The later integrated source `69cf0253e` merges PR head `ac16317e5` and main
`942caf7e4`; the SAT blob remains identical. A fresh source-specific attempt is
retained under `pipeline-receipts/final-integrated-library-gate-20261026-69cf/`
in the study, preserving the first failed gate separately.
No bypass, current-head test pass or merge is claimed.

On2026-10-26, main e8fcadf62 supplies SAT blob
`f1bac71439f5835e94c8d8f12071b86600a9a73f`, merged as d3c9f02c7.
The first post-fix attempt stopped on four omitted tracked fixture files; those
were materialized and the unchanged library tree now runs4,991tests. Both failed
receipts remain preserved. Do not call the full gate green before completion.
