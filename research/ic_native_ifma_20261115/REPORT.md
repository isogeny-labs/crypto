---
title: "An AVX-512 IFMA native sweep for rootfind-subtract: 1.26–1.67× the scalar native sweep"
date: 2026-11-15
---

# An AVX-512 IFMA native sweep for rootfind-subtract

User request (2026-11-15): "Keep working on open leads", then "All". The open lead from
`research/ic_native_sweep_20261115`: a SIMD field kernel for the CPU sweep (the native sweep there is
scalar 64-bit Montgomery and on par with the emulation arm).

**Standing.** `[pcc:unverified]`: one local run in this container (4-thread Intel Xeon VM with AVX-512
IFMA, L0, `--cpus none --allow-busy`), measured by the session that wrote the code; no validator yet.
Wall figures are indicative. Counted S is unchanged by design and measured unchanged. No scoreboard or
leaderboard row is added. An independent validator (`VALIDATION_REVIEW.md`) re-verified all 96
certificates and the planted targets with its own code, matched all 64 identity checks, reviewed the
kernel, ran a randomized test against u128 arithmetic for primes up to just below 2^52, and checked 13
mutations: **validated** at the report's scope, with prediction 2 corrected below (*corrected*). The
host was loaded during the run (load average 1.4–2.7); the thread-pool size and a counter showing the
IFMA kernel ran are not in the evidence.

![Relation-phase speed-up over one thread, [pcc:unverified]](fig_ifma.svg)

## What was built (commit `1deabdf708`, pushed before the run)

`rootfind-subtract-gpu:worker=native,simd=ifma`: the native sweep with eight targets per step, one per
AVX-512 lane. Montgomery multiplication with `R = 2^52` from `vpmadd52luq`/`vpmadd52huq` (the product's
low and high 52 bits, the reduction's quotient, the carry, one conditional subtraction by `min`);
additions and subtractions mod `p` by `min` as well. Each lane runs its own batched inversion over the
base's abscissae (prefix products, one scalar inversion per lane, the backward pass), then the two
candidates per abscissa, converted out of Montgomery form and probed per lane in the open-addressing
set of canonical abscissae. The candidates and their order are the scalar sweep's; hits are confirmed
and charged by the oracle's existing path. Refused without IFMA or for `p ≥ 2^52`. Tests
[pcc:unverified]: the lanes equal the scalar sweep hit for hit (targets in and out of the base,
negatives, chunks of 1, 8, 9 and 31 targets); whole pipelines equal `rootfind-subtract` at `m = 2` and 3;
`simd` refused outside `worker=native` and for unknown values.

## Protocol and run

`PROTOCOL.md`, `specs/ifma2.json` and `specs/ifma3.json`, frozen and pushed before the run (`219b1378f8`):
the workloads of the native-sweep study, 2 targets, 2 rounds, alternating; arms `rootfind-subtract`
(one thread), `worker=native` at `threads=1` and all threads, each scalar and `simd=ifma`; `rho.negation`.
`benchrec run` of `run.sh` (record `benchmarks/records/20261115T2010Z-ic-native-ifma.json`, initialised
on the clean pushed commit); `ecbench verify --replay-all` and `ecbench band` per session. Raw
`raw/out_ifma.tgz` (sha256 `77afcb94…`); tables `RESULTS.md`, `results.json`, `band-ifma2.json`,
`band-ifma3.json`.

## Results [pcc:unverified]

- **Identity (prediction 1 held):** 96/96 records verified; 96 replays reproduced, 0 problems. 64 checks
  of every IC arm against the one-thread arm (per workload and round): equal trials, relations,
  recovered logarithm, status, S, total GAE and every phase's charged additions, doublings, scalar
  multiplications and GAE; 0 mismatches. [pcc:unverified]

| m | log2 r | \|F_x\| | one thread: relation phase | scalar ×1 | IFMA ×1 | scalar ×4 | IFMA ×4 | IFMA / scalar, 1 thread | IFMA / scalar, 4 threads |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 27.5 | 4096 | 2.01 s | 0.79× | 1.32× | 3.09× | 4.62× | 1.67× | 1.49× |
| 2 | 30.0 | 4096 | 11.4 s | 0.94× | 1.49× | 3.47× | 5.63× | 1.60× | 1.62× |
| 3 | 33.4 | 2048 | 0.436 s | 0.52× | 0.75× | 2.09× | 2.63× | 1.45× | 1.26× |
| 3 | 35.2 | 2048 | 1.74 s | 0.95× | 1.29× | 3.49× | 4.54× | 1.36× | 1.30× |

(Speed-ups over the one-thread `rootfind-subtract` arm's median relation-phase wall, median of 4; the
IFMA / scalar columns are ratios of the unrounded medians, *corrected*: three cells were first divided
from medians rounded to three decimals.)
[pcc:unverified]

- **Prediction 2 failed narrowly at one of its four cells** (*corrected*; first stated as "held at its
  threshold"): at `m = 2` the IFMA sweep is 1.60–1.67× the scalar native sweep on one thread and
  1.49–1.62× on four; the 4-thread cell at 27.5 bits is 1.4945, below the predicted 1.5×. Paired per
  target and round that cell is 1.41–1.62× (median 1.56); with 4 runs, L0, a loaded host and no A/A
  arm, the run cannot place it on either side of 1.5×.
- **Prediction 3 held:** at `m = 3` the gain is smaller, 1.26–1.45× (*corrected* from 1.25–1.44×). A
  likely cause, not measured (no performance counters on this host): the probe of the pair table's
  abscissae (a 64 MiB set) is a random memory access per candidate, which the lanes do not speed up.
- **Against one thread:** on four threads the IFMA sweep's relation phase is 4.6–5.6× the plain Rust arm
  at `m = 2` and 2.6–4.5× at `m = 3`; the solve 4.0–5.4× at `m = 2` and 1.2–2.0× at `m = 3` (where the
  CPU-built pair table and linear algebra are unchanged).
- **Distance to rho** (§5a band, IFMA arm, w = 0 / 0.1 / 1): `m = 2` 6 690 / 6 706 / 6 781 and 16 809 /
  16 830 / 16 922; `m = 3` 213 / 296 / 668 and 150 / 171 / 264; the same on every IC arm; rho is ahead at
  every weight.

## What this changes

The strongest CPU baseline for the GPU comparisons is now the IFMA native sweep on hosts that have IFMA:
on this VM, 1.49–1.62× the scalar native sweep at `m = 2` on four threads (the emulation arm was not in
this run; the earlier study measured it on par with the scalar native sweep). How much it narrows the
A100's relation-phase lead is not measured (*corrected*: the earlier "by that factor per CPU core" was
an extrapolation, and the factor was the four-thread one); a many-core IFMA host beside the GPU is not
measured either.

## Not done / next

- The IFMA arm on a many-core host beside the GPU, and a vectorised probe (gather) for `m = 3`.
- Process (validator): raw data in Git, not S3 (this session's AWS keys are rejected; the same waiver as
  earlier rounds); the record's PDF field names REPORT.md; session ifma3 records commit `e96ec1b771`
  (the open record's commit during the run, same binary). The record's summary keeps the first
  "1.50–1.62×" and "1.25–1.44×" (a closed record cannot be amended); this report supersedes them.
- Code (validator): the tests did not cover primes near 2^52, where dropping the Montgomery product's
  final reduction goes wrong (the run's primes are below 2^36). A test on primes up to just below 2^52
  now covers it and fails under that mutation (*corrected*). Unreachable: an empty base would panic,
  and the inversion assumes a prime `p`.
