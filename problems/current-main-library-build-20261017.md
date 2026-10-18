# Current main-derived library build blocks N83 m=4 validation

- **Status:** `open`
- **Kind:** inherited implementation/build gap; current-head validation blocker
- **Affected work:** [N83 chained-S3 m=4 validation](../research/n83_chained_s3_m4_20261017/VALIDATION.md), [K=16,627 m=4 capacity validation](../research/n83_k16627_m4_capacity_20261017/VALIDATION.md), PR #2100 and its dependent m=4 PRs
- **Next action owner:** maintainers of the current `main` build and PR #2100 CI
- **Last updated:** 2026-10-18

## Problem

After Copilot merged `main` parent
`c6d64436a33359320ad9d5b42c4acebee2968b9a` into N83 review PR #2100
at `dd1f7585fa7c7e56749580afb561d59f78f9f624`, the dependent m=4 branch
cannot compile its release library suite. The compiler stops in unrelated
ecbench and quartic modules before m=4 tests execute on the merged head.
This blocks the repository's required current-head validation and therefore
merging the dependent factor-base change. It does not constitute evidence
about relation yield or mathematical feasibility at degree 83.

## Evidence and uncertainty

- **Observed:** The [merged-head focused build log](../research/n83_chained_s3_m4_20261017/final-focused.log)
  from `cargo test --release --lib chained_s3_` exits 101 with 26 compile
  errors. The first errors are duplicate `Fp4` variants in
  `src/cryptanalysis/ecbench/workload.rs`, unresolved `ecbench::fbm` imports,
  missing `OnbWeightBase` imports, and undefined `sparsest` in
  `src/cryptanalysis/gaudry_quartic.rs`. The [exact full command log](../research/n83_chained_s3_m4_20261017/final-release-lib.log)
  is retained separately. The pre-merge m=4 code passed 4,216 release library
  tests, with 117 ignored, in [its complete log](../research/n83_chained_s3_m4_20261017/cargo-release-lib.log).
- **Observed:** The [K=16,627 capacity stack](../research/n83_k16627_m4_capacity_20261017/VALIDATION.md) independently reran `cargo test --release --lib -- --test-threads=2` after merging its #2124 base. It exited 101 with 26 errors in the same ecbench and Gaudry source area; [the raw log](../research/n83_k16627_m4_capacity_20261017/cargo-release-lib-stacked.log) is retained. The five error-location source files are byte unchanged from #2124. Before the stack merge, its source passed the complete library suite: 4,216 passed and 117 ignored.
- **Derived from source identity:** The three affected files above have
  identical Git blob IDs in main parent `c6d64436a33359320ad9d5b42c4acebee2968b9a`
  and PR #2100 head `dd1f7585fa7c7e56749580afb561d59f78f9f624`.
  The dependent m=4 branch does not change those files. This establishes
  that the reported source errors were inherited, not introduced by the m=4
  diff. A direct build of a clean main checkout is not recorded here.
- **Suspected explanation:** Concurrent main-branch integrations left
  ecbench and quartic source references inconsistent. This is a source-level
  hypothesis, not an attribution to a particular PR or author.
- **Missing capability:** A compiling current main-derived source revision
  and current-head release validation.
- **What this evidence does not establish:** It does not show that all of
  main is broken on every toolchain, nor that the m=4 branch passes after
  the inherited errors are corrected. The earlier pass covered the
  pre-merge source only.

## What would unblock it

- [ ] A base-branch fix or subsequent merge removes the duplicate variant,
  resolves the `fbm` and `OnbWeightBase` paths and `sparsest` references,
  and makes `cargo test --release --lib -- --test-threads=2` compile and pass
  at the final PR head without skipping tests.
- [ ] Re-run the touched m=4 tests and inspect the final diff after merging
  that fix; keep PR #2100 and the dependent PR at their passing current heads.

Meeting these conditions would unblock code validation. It would not itself
establish degree-83 natural relation yield, useful rank or complete cost.

## Next discriminating step

- **Action or proof obligation:** Compare PR #2100 CI's compile failure, if
  any, with the local log; test a clean base revision after the maintainer's
  source fix, then rerun the dependent m=4 release gate.
- **Inputs and controls:** The two pinned parents named above, the exact
  local build command and logs, and identical inherited blob IDs.
- **Execution limits and stop condition:** No research search budget is
  involved; stop any new validation attempt at a compiler failure and retain
  its diagnostics. Do not patch unrelated base code in the factor-base PR.
- **Outcomes and next actions:** Matching CI confirms the shared blocker;
  a clean base build with different errors requires checkout/toolchain
  diagnosis; a fixed base permits current-head validation and PR review.
- **Recheck trigger:** A new `main` or PR #2100 head with relevant source
  changes or a completed CI report.

A planned step is not a completed experiment.

## Resolution and history

Unresolved. This record names a validation dependency and does not resolve
the N83 factor-base quality problem.
