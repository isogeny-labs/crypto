# Prospective protocol: projected prime-abscissa bases on the volcano panel

**Status:** prospective; no affected data have been taken. **Date:** 2026-11-01.
The machine-readable preregistration is [`protocol.json`](protocol.json). It
must validate, be frozen, and be committed before any cell is submitted.

## Question

The confirmed v2.1 volcano result used `prime-abscissa:size=32` on the 14
ell=2 classes, whose cofactors are 4 or 12. Does the floor/crater IC effect
survive after the factor base is projected into the prime-order subgroup with
`prime-projected-abscissa:size=32`, or was it mediated by raw cofactor-class
dead weight?

This is a 24-bit, split-crater, depth-3 discriminator. It is not a scaling or
security claim. A projected-base win is engineering; it does not change the
index-calculus exponent and remains measured against matched negation rho.

## Frozen population and arms

The population is exactly the 14 ell=2 class exports from
`research/isogeny-volcano-cost-v2_20261017`: 3,904 curve-models, eight
`planted_shared` targets per model, 31,232 workloads. No curve, model, target,
generator, level, or class is regenerated.

Every workload runs, in one ecbench spec:

1. `rho-neg`: `rho.negation`, the matched reference and negative control;
2. `ic-raw`: the old `prime-abscissa:size=32` baseline;
3. `ic-projected`: `prime-projected-abscissa:size=32`, the sole treatment;
4. `rho-neg-aa`: the matching rho A/A arm.

The oracle remains `mitm:negation_folded=1`. Five measured rounds, one warmup,
shuffle order, eight targets, L2, and one thread are fixed. The old raw H1
effect is the positive control. The old ell=3 result remains an already-seen
internal-null calibration and cannot decide the projected result.

## Cells and execution

The EKS request contract caps a job at 3,600 seconds. The original class jobs
took 2,807--3,754 seconds for only two arms and one measured round, so a class
cannot be one current job. [`generate-specs.sh`](generate-specs.sh) slices each
class's existing ordered curve list into consecutive groups of at most 40,
without changing a record. This yields 104 independent cells recorded in
`specs/manifest.json`. All cells are submitted once, concurrently; the existing
32-vCPU L2 pool may queue them. An infrastructure failure is retained and may
be retried once under the same frozen cell identity. A timeout, wrong answer,
or algorithm error is an outcome and is not retried.

No measurement may start until all of these hold:

- `protocol.json` validates and freezes cleanly;
- `generate-specs.sh --check` and every `ecbench plan` pass;
- the source commit and OCI digest are pinned in the requests;
- `scripts/checkpoint.sh gate` passes on a pushed branch;
- EKS readiness, capacity, request identity, L2 admission, and A/A admission
  pass without lowered thresholds.

## Primary analysis and decision

[`analyse.sh`](analyse.sh) is frozen before data. It combines all cell records
against the original class sidecars and runs the class-stratified
`volcano-stats` analysis with 2,000 bootstraps and 10,000 permutations.

The primary statistic is projected IC floor/crater S with its 90% interval.

- **Mediated / close the representative-selection lead:** the projected 90%
  interval is wholly inside `[0.97, 1.03]` and projected model tau is at most
  the already-recorded null p95, 0.047.
- **Persists / proceed to selection arms:** the projected interval excludes 1,
  its upper endpoint is below 0.97, and the one-sided permutation p is below
  0.05.
- **Inconclusive:** every other complete, valid result.
- **Invalid:** any non-verified row, a failed raw positive control, a failed rho
  negative/A/A control, incomplete cells, identity mismatch, or failed replay.

The raw positive control must again have a floor/crater ratio below 0.97 with
one-sided p below 0.05. Rho must be equivalent within +/-3%. Candidate/raw
relation yield and per-cell S comparisons are secondary. Candidate, raw, and
rho are priced at 0, 0.1, and 1 GAE per emitted uncharged native operation; no
ordering is claimed unless it holds throughout that band.

## Inadmissible moves

- changing the population, shard membership, arms, seeds, rounds, margins,
  direction, or thresholds after any affected data;
- dropping or rerunning a cell because of its scientific value;
- joining a cell to a different source/image/spec identity;
- treating wall time below L2 as evidence;
- treating a stage-only change, a raw/projected accounting difference, or the
  gap to rho as an exponent improvement;
- publishing a leaderboard or benchmark row before the independent validation
  artifacts in [`validation/README.md`](validation/README.md) pass.
