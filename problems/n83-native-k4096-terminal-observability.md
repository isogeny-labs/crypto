# Native N83 K4096 terminal-receipt observability

- **Status:** investigating
- **Kind:** empirical bottleneck with unknown internal phase; instrumentation gap
- **Affected work:** [native K2048/K4096 continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/cloud-matched-k2048-k4096-20261018/RESULT.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-18

## Problem

This native host/worker's primary a=0, GF(2^83), K4096 five-summand path has no terminal
solver-counter receipt within either the 100000-conflict/600-second attempt
or the separate 1000-conflict/180-second control. This prevents a completed
matched-query comparison with the K2048 short control and does not identify
which internal phase consumed the budget.

## Evidence and uncertainty

- **Observed:** exact K4096 input reconstruction, generic replay of all
  679936 points, fresh setec S3 custody and guarded model construction pass.
  The constructed all-affine model has 131958 variables, 2021197 clauses
  and a 635953152-byte cgroup peak. Both natural-query attempts stop at
  their outer wall caps without terminal summaries.
- **Observed:** K2048's separate short control returns exactly 1000
  conflicts, zero models, zero relations and rank zero. Its original-group
  query replay passes. This establishes a complete bounded receipt on that
  arm; it is not an estimate of natural yield or complete cold runtime.
- **Observed:** the K4096 events reach `FactorBaseReady` and
  `RelationCollectionStarted`. They do not bracket domain closure, internal
  encoding, SAT entry or SAT preprocessing. The separate construction gate
  does not measure all work inside the relation driver's decomposition call.
- **Source:** `sat_decompose_chained_s3_with_domain` in
  [the driver](../src/cryptanalysis/koblitz_index_calculus.rs) performs domain
  closure checks, encoding and solver work before returning its statistics.
- **Unknown:** final conflicts, models, relation count, rank and worker peak
  memory for each wall-censored attempt. SAT entry on those attempts is not
  independently established by the retained events alone.
- **Separate upstream evidence:** the [emulated K4096 gate](../research/koblitz_n83_factor_base_sweep_20261008/verification/k4096-orbit-model-20261018/RESULT.md)
  returned a 5,000-conflict solver-cap receipt in 373.290 seconds with zero
  models, relations and rank. This native-host gap does not invalidate that
  result or establish a cross-host throughput comparison.
- **Not established:** UNSAT, absent decompositions, a mathematical
  obstruction, the fastest base, or a complete IC-vs-rho comparison.

## What would unblock it

- [ ] Emit flushed phase markers and actual query coefficients/point before
  the blocking work; distinguish closure, model construction, solver entry
  and terminal disposition on the same exact retained input.
- [ ] Validate those markers on a public positive control and preserve
  capped/failed outcomes under a protocol frozen before execution.
- [ ] Obtain terminal, independently group-replayed query receipts for both
  retained bases under a matched declared envelope, or identify the measured
  blocking phase without treating censoring as a conflict or relation count.

Meeting these conditions would resolve observability and permit a stage
comparison. Natural useful rank, verified column logs, individual-log work,
matched references and complete cold runtime would still be required to
select a factor base.

## Next discriminating step

Use the repository's now-required native harness for further comparisons.
The present receipts preserve a historical Python supervisor around a Rust
worker; they are not a supported future execution path. Add opt-in native
phase receipts to the decomposition path, test on the existing
small-field verified solve and a public known-witness control, then freeze
one retained K4096 diagnostic with its exact source, inputs and resource
limits. Stop at the declared wall/memory/conflict bounds. A trace that stops
before SAT entry points to admission/construction work; a trace that reaches
SAT but lacks a terminal result localizes the remaining solver-stage gate.
Neither outcome alone proves nonexistence or an asymptotic obstruction.
This step is proposed, not launched by the present rung.

## Resolution and history

2026-10-18: Docker and S3 access are restored, two construction gates pass,
three search attempts are outer-wall censored, and the K2048 short control
has a complete solver-cap receipt. The K4096 phase/counter gap remains open.
