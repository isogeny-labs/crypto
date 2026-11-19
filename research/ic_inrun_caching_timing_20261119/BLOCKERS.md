# Blockers and gaps for a faithful run

Status at drafting (2026-11-19). The **hard** items stop a launch. The
**scope** items limit what the run can claim. The **gap** items are
disclosed, not cured.

## Hard (resolve before dispatch)

- **B5. The spec must exist at both tested commits.** `isolab-experiment.yml`
  reads `spec_path` from `source_revision` via the contents API. `ad2a014`
  (main) does not contain this directory. The fix is in `DISPATCH.md` §0c:
  a branch `ic-inrun-caching-timing-baseline` whose one commit adds only this
  directory on top of `ad2a014`. Pushing it, and the NEW commit, needs push
  rights. This package does not push.
- **B6. Freeze the NEW code.** The PR branch is still moving: the lead is
  working on a `lanczos` fix and CI cleanup (`2dfce5f7d`, `a0b215b0d`). The
  NEW revision must be a single named head whose `src/`, `crates/` and
  `Cargo.lock` match the head named in the protocol. A later code commit
  means a new protocol amendment, or a new study. The arms avoid `lanczos`,
  but `filter+wiedemann` shares `SparseCado` with the code being changed.
- **B10. Old binary accepts `batch=0`.** The protocol's negative control and
  the shared spec assume that `ad2a014`'s plain `subtract`/`mitm` oracles
  ignore unknown oracle parameters. They read only `m`, `negation_folded`
  and `layout`, with no key check; this was checked in the `ad2a014`
  `plugins.rs` source, but not by running it. If the old binary refuses the
  key, job O-P24-1 fails at plan or child time and costs only that job.
  Then drop the `*-b0` arms from a v2 spec for both revisions. Do not use
  different specs per revision.
- **B11. Budget and access are unread.** No AWS call was made. Before each
  dispatch, record the daily and monthly budget headroom, the pool check and
  the caller identity. Also confirm that the dispatching GitHub identity can
  run `workflow_dispatch` on `main` (OIDC role restricted to `main`).

## Scope (changed defaults that ecbench cannot reach, so no EKS timing)

- **B1. C1, ecdl-tensor-ic `MitmTable`.** `ic.tensor_contraction_v1` (and
  `ic.tensor_cofactor`) build `Solver` with `mitm = false` (the default). The
  MITM path is reachable only from `src/bin/ic/tensor.rs` and the crate's
  own bins. The workflow builds only `ecbench`. Options: (a) a new ecbench
  parameter `mitm=1` on `ic.tensor_contraction_v1`, which is a code change
  needing its own PR, and would itself be missing from the old binary; or
  (b) a local counted-only comparison. The m = 4 count drop (5278 → 2818
  `relation_search.adds`) is deterministic and needs no clock.
- **B2. C2a, incremental dense retries in `ec_index_calculus.rs`.** These are
  called only by `icx prime_external`, `research_bench` and the `ic` binary,
  not by ecbench.
- **B3. C3, sweep x-index hoist.** It is in the same unreachable pipeline.
  The `IC_HOIST_SWEEP=0` toggle is an environment variable, and neither the
  workflow nor `ecbench.spec/v1` can set one for the pod. The PR calls this
  "uncounted hash work only", so it can only be seen in wall time.
- **B4. Round 2 #3, `petit_quisquater` `PqLookup`.** It is not reached from
  ecbench.

For B1–B4 the only faithful timing route would be the ephemeral runner
(§9b) or an isolab custom command job (`isolab_k8s_run`) running the `ic` or
`icx` binary under its own frozen protocol. That is a separate study; this
package does not cover it.

## Gaps (disclosed)

- **B7. No wall positive control.** No arm has a known, planted wall
  slowdown. D5's in-binary pair is the only decided wall reading. The
  cross-binary wall is descriptive (`ecbench compare --b-dir` is
  operations-only).
- **B8. No `w_cal`.** C5 moves field inversions, which are field-arithmetic
  counters. §5a asks for a per-host calibrated weight, and these jobs cannot
  run `ecbench wcal` alongside. The band (0, 0.1, 1) governs, and the
  calibrated reading is omitted and labelled.
- **B9. Frozen regression suite (§8).** The suite
  `research/index_calculus_baseline_20260914/regression` was not run. It is
  not needed to launch, but a pipeline claim for C4 (descent system
  construction) needs it.
- **B12. Wall reference.** `rho.prime_fast`, the §12 prime wall reference,
  is not registered as an ecbench method in either binary.
  `rho.negation` stands in, and this is labelled.
- **B13. Counted S for `descent-algebraic` is a lower bound.** `ic.pipeline`
  leaves the algebraic solver term unpriced. C4 is therefore decided in
  bitop/v1 by phase, and its S is reported as a lower bound.
- **B14. Feasibility is not piloted.** The per-child times (`subtract` about
  r = 2^24 additions; F4 on K0-13 with dimension 6) are estimates. Disk and
  memory limits here forbid a local build. If a job's children time out,
  that job is censored, and its rerun at a changed size is a protocol
  amendment for both revisions.
- **B15. Unreproducible builds.** Each dispatch rebuilds its revision.
  Compare `ecbench_sha256` across the two builds of one revision, and
  report any difference.
- **B16. Experiment-protocol record.** `protocol.json`
  (`experiment.protocol/v1`) needs the `crypto-autoresearcher` checker, which
  is not on this machine. `PROTOCOL.md` is the frozen design until the
  checker is run.
