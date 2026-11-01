# Chained-S3 on N83, m = 8, 9: parallel back enumeration (preregistration)

Frozen 2026-11-01 before the measured run. User request: "now parallelize the
back enumeration for m=8 and m=9". Follow-up to
`research/n83_chain_sat_indexed_20261101` (#3216): indexed lookup + peel
rebuild decide every target at m = 8 (≈ 40 s per query) and m = 9 (≈ 54 s),
the time being the single-threaded back enumeration (C(170, 4) ≈ 3.4·10^7
additions and lookups per query).

## Arms (same targets, arm order rotated per target)

| id | arm | d (m = 8 / 9) |
|---|---|---|
| A6 | `all-summands+t2.2+meet<d>+rebuild+indexed` (merged in #3216; control, one thread) | 4 / 5 |
| A7 | A6 `+parallel` (back enumeration split on the multiset's first index across the rayon pool, `RAYON_NUM_THREADS=4`) | 4 / 5 |

The parallel split returns the same hits in the same order as the sequential
enumeration (unit test `n83_parallel_back_hits_match_sequential`); only wall
time can change. The work done is the same.

## Targets, caps, host

Seed 20261031; per m 3 public, 5 planted (the targets of #3216). Per query
300,000 conflicts, 120 s. `run_shard.sh <bin> raw/m8 8 4`, then
`run_shard.sh <bin> raw/m9 9 5`, one after the other so each process has the
4-vCPU host to itself (L0, shared cloud VM). Lookup sets are built once per
process before the first query (single-threaded warmup, not timed).

## Decision rule (frozen)

- Soundness: a refuted planted target or an unverified witness voids the arm;
  A7 must give the same verdict as A6 on every target.
- Improvement (A7 over A6), per m: lower median wall seconds per query on
  paired targets, both arms deciding every query. Reported with the thread
  count; wall-time speed-up on 4 vCPUs, not a reduction in work.

## Limits

L0 host, 8 targets per m; wall time only (no CPU-time accounting).
Decomposition stage only: no DLP, no leaderboard or benchmarks row. EKS not
used (not an ecbench method, no aws CLI).
