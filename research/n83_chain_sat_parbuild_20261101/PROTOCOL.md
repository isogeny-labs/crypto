# Chained-S3 on N83, m = 9: parallel build of the depth-5 fingerprint set (preregistration)

Frozen 2026-11-01 before the measured run. User request: "now parallelize the
depth-5 build for m=9". Follow-up to `research/n83_chain_sat_parallel_20261101`
(#3247): m = 9 queries take ≈ 14 s on 4 threads, but the one-off,
single-threaded build of the depth-5 set (1.15·10^9 multisets, 372,406,144
distinct fingerprints) took 1,023 s.

## Arms

| id | arm (m = 9, d = 5, `RAYON_NUM_THREADS=4`) |
|---|---|
| S | `all-summands+t2.2+meet5+rebuild+indexed+parallel` (merged in #3247; sequential build) |
| P | S `+parbuild`: `chain_s3_build_fingerprints` with the enumeration split on the multiset's first index (each worker fills its exact-size slice of one array), compaction over identity sums, parallel sort |

Both builds produce the same set (unit test `n83_parallel_build_matches_sequential`,
depths 1–3); `front_fingerprints` must match between S and P at every depth.

## Design

The build happens once per process, so each repetition is a separate process:
S1, P1, S2, P2 in that order (`run_all.sh`), one at a time on the 4-vCPU host
(L0, shared cloud VM). Each process builds depths 2…5 (warmup lines, the
measured quantity) and then answers the same 3 public + 5 planted m = 9
targets (seed 20261031), 300,000 conflicts and 120 s per query, as a check
that the set built is usable.

## Measurement and decision rule (frozen)

- Primary: wall seconds of the depth-5 build per process (warmup line, meet
  depth 5); secondary: depths 2–4 and the total.
- Soundness: equal `front_fingerprints` per depth across all four processes;
  every query decided with the verdict of #3247 (public refuted, planted
  verified); 0 soundness flags.
- Improvement (P over S): both P builds faster than both S builds at depth 5.
  Reported as the ratio of means with both values; wall-clock on 4 vCPUs, not
  less work.

## Limits

Two repetitions per arm, one host; wall time only; peak memory not measured.
Decomposition stage only: no DLP, no leaderboard or benchmarks row. EKS not
used (not an ecbench method, no aws CLI).
