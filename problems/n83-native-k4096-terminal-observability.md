# Native N83 K4096 terminal-receipt observability

- **Status:** resolved for one frozen K4096 terminal diagnostic; matched comparison open
- **Kind:** empirical bottleneck localized to SAT search; bounded receipt
- **Affected work:** [native K2048/K4096 continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/cloud-matched-k2048-k4096-20261018/RESULT.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-19

## Problem

The earlier native host/worker's primary a=0, GF(2^83), K4096 five-summand
path had no terminal solver-counter receipt within either the
100000-conflict/600-second attempt or the separate 1000-conflict/180-second
control. A later frozen diagnostic now identifies the blocking phase and
provides a terminal 1000-conflict receipt. A matched source/host multi-query
comparison with K2048 and complete runtime selection remain open.

## Evidence and uncertainty

- **New observed terminal receipt:** the
  [frozen native diagnostic](../research/koblitz_n83_factor_base_sweep_20261008/verification/k4096-native-phases-20261018/RESULT.md)
  built the exact 131958-variable/2021197-clause/4980-XOR model in
  780.945 ms after decomposition entry, entered SAT, and returned
  `UNKNOWN_solver_cap` at exactly 1000 conflicts after 137.457 seconds in
  the solver. One public query was independently group-replayed. It yielded
  no model or relation within that cap, and its observed matrix rank was
  zero. Peak cgroup memory was 751669248 bytes under 4 GiB and zero swap.
- **New inference:** model admission and domain construction are not the
  180-second short-control bottleneck on this exact source/input. The
  conflict-bounded SAT call dominates the completed diagnostic. This does
  not retroactively assign terminal counters to the earlier wall-censored
  attempts or establish a throughput comparison across their source/host.
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
- **Unknown for the earlier attempts:** final conflicts, models, relation
  count, rank and worker peak memory for each wall-censored attempt. SAT
  entry on those attempts is not independently established by their retained
  events alone.
- **Separate upstream evidence:** the [emulated K4096 gate](../research/koblitz_n83_factor_base_sweep_20261008/verification/k4096-orbit-model-20261018/RESULT.md)
  returned a 5,000-conflict solver-cap receipt in 373.290 seconds with zero
  models, relations and rank. This native-host gap does not invalidate that
  result or establish a cross-host throughput comparison.
- **Not established:** UNSAT, absent decompositions, a mathematical
  obstruction, the fastest base, or a complete IC-vs-rho comparison.

## What would unblock it

- [x] Emit flushed phase markers and actual query coefficients/point before
  the blocking work; distinguish closure, model construction, solver entry
  and terminal disposition on the same exact retained input.
- [x] Validate those markers on a public positive control and preserve
  capped/failed outcomes under a protocol frozen before execution.
- [ ] Obtain terminal, independently group-replayed query receipts for both
  retained bases under a matched declared envelope, or identify the measured
  blocking phase without treating censoring as a conflict or relation count.

Meeting these conditions would resolve observability and permit a stage
comparison. Natural useful rank, verified column logs, individual-log work,
matched references and complete cold runtime would still be required to
select a factor base.

## Next discriminating step

The first frozen K4096 diagnostic now has a complete terminal bounded
receipt. Use the native harness for a same-source, matched-resource K2048
control and repeated public queries, then charge relation yield, useful
rank, LA, target work, verification and rho. The historical Python
supervisor receipts remain provenance, not a supported future execution
path. A 1000-conflict unknown on one query is not evidence of nonexistence
or an asymptotic obstruction.

## Resolution and history

2026-10-18: Docker and S3 access are restored, two construction gates pass,
three search attempts are outer-wall censored, and the K2048 short control
has a complete solver-cap receipt. The K4096 phase/counter gap remains open.

2026-10-19: A pinned native `c7i.xlarge` run on the same retained K4096 object
passed exact source/input replay, emitted 12 phase markers, independently
replayed its query, and completed with `UNKNOWN_solver_cap` at 1000 conflicts.
Four forged audit inputs were rejected. The former terminal-observability
gap is resolved for this bounded trial; matched total-cost selection is open.
