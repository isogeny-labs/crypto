---
title: "A native multi-threaded CPU sweep for rootfind-subtract, on par with the emulation arm on this host"
date: 2026-11-15
---

# A native multi-threaded CPU sweep for rootfind-subtract

User request (2026-11-15): "Keep working on open leads". The open lead from rounds 3 and 4
(`research/ic_gpu_rootfind_big_20261115`, `research/ic_gpu_rootfind_pairs_20261115`): the
multi-threaded CPU arm of the GPU rounds (`backend=emulate`) is the CUDA kernel's per-thread code run
on host threads, and round 3's validator found it sweeping 1/7–1/9 as many candidates per core as the
single-thread Rust arm; a CPU-optimised parallel sweep was wanted as the strongest CPU baseline.

**Standing.** `[pcc:unverified]`: one local run in this container (4-thread Intel Xeon VM, L0,
`--cpus none --allow-busy`), measured by the session that wrote the code; no validator yet. Wall
figures are indicative. Counted S is unchanged by design and measured unchanged. No scoreboard or
leaderboard row is added. An independent validator (`VALIDATION_REVIEW.md`) re-verified all 80
certificates with its own code, matched all 48 pairs on every phase's charge and counter (only the
`gpu_*` counters differ), reproduced every number, and showed by mutation that the tests catch a
dropped first hit: **validated, scoped**; its corrections are applied below (*corrected*). The host was
loaded during the run (1-minute load average 1.3–3.4 on 4 CPUs). The GPU pod for round 5 could not be created (RunPod account balance), so
nothing here is a GPU measurement.

![Relation-phase speed-up over one thread, [pcc:unverified]](fig_native.svg)

## What was built (commit `2ec4187be0`, pushed before the run)

`rootfind-subtract-gpu:worker=native[,threads=N]`: no worker process. Each lane batch's targets are
swept in this process on a rayon pool (default: every thread the process may use), each with
`rootfind-subtract`'s own arithmetic: one Montgomery batched inversion over the base's abscissae and the
two candidates `R ∓ F_i` per abscissa, each probed in an open-addressing set of Montgomery-form
abscissae (`m = 2`: the base's; `m = 3`: the pair table's, built and charged by the CPU oracle as in the
CPU arm). The reported positions, a superset of the CPU lookup's hits, are confirmed and charged by the
oracle's existing path, so the charge is `rootfind-subtract`'s. Not vectorised: the field arithmetic is
the scalar 64-bit Montgomery code. Tests [pcc:unverified]: equal to `rootfind-subtract` over whole
pipelines at `m = 2` and 3, 1 and 3 threads, single targets and lane batches; `threads` refused without
`worker=native`, and `backend`/`probe_build` refused with it.

## Protocol and run

`PROTOCOL.md`, `specs/native2.json` (`m = 2`, 28/30-bit `prime_search` curves, |F_x| = 4096) and
`specs/native3.json` (`m = 3`, 34/36-bit prime_explicit curves, |F_x| = 2048), frozen and pushed with the
code; 2 targets, 2 rounds, alternating. Arms: `rootfind-subtract` (one thread), `worker=native,threads=1`
(the batching control), `worker=native` (4 threads), `worker=spawn,backend=emulate` (the earlier
multi-threaded arm; its pool sized 4 by the CPU budget), `rho.negation`. `benchrec run` of `run.sh`
(record `benchmarks/records/20261115T1740Z-ic-native-sweep.json`, initialised on the clean pushed
commit); `ecbench verify --replay-all` and `ecbench band` per session. *Corrected:* two commits
(session log and analysis script, no source change, same binary hash) landed in the measured checkout
during the native2 session, so native3's `session.json` names commit `6cac7eb73d`. The thread-pool size
(4) follows from the host's CPU count and is not recorded in the evidence. Raw `raw/out_native.tgz` (sha256
`4b8351ca…`); tables `RESULTS.md`, `results.json`, `band-native2.json`, `band-native3.json`.

## Results [pcc:unverified]

- **Identity:** 80/80 records verified; 80 replays reproduced, 0 problems. 48 checks of every IC arm
  against the one-thread arm (per workload and round): equal trials, relations, recovered logarithm,
  status, S, total GAE and every phase's charged additions, doublings, scalar multiplications and
  GAE; 0 mismatches. [pcc:unverified] (*Corrected:* the first analysis script read two ledger keys the
  records do not carry, so it compared phase names and additions only; it now reads the four charged
  fields, with the same result, and the validator's stricter check agrees.)

| m | log2 r | \|F_x\| | S / rho | one thread: relation phase | native ×1 thread | emulation (4) | native (4) | native / emulation |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 27.5 | 4096 | 5 967 | 1.85 s | 0.86× | 2.84× | 3.15× | 1.11× |
| 2 | 30.0 | 4096 | 24 637 | 11.2 s | 0.93× | 3.33× | 3.32× | 1.00× |
| 3 | 33.4 | 2048 | 106 | 0.465 s | 0.56× | 2.27× | 2.24× | 0.99× |
| 3 | 35.2 | 2048 | 162 | 1.54 s | 0.77× | 2.79× | 2.88× | 1.03× |

(Speed-ups are the one-thread arm's median relation-phase wall over each arm's, median of 4.) [pcc:unverified]

- **Prediction 2 failed:** at `m = 2`, the prediction's scope, the native sweep is 1.00–1.11× the
  emulation arm's relation-phase speed (paired per workload 0.95–1.15×; predicted at least 1.5×); with
  the `m = 3` cells, 0.99–1.11× (*corrected*: the range had mixed in `m = 3`). On this host the emulation arm is not the weak baseline round 3 described;
  both reach 2.2–3.3× one thread on 4 threads. Round 3's 1/7–1/9 per-core figure came from a pod with
  96 visible threads and an unrecorded quota; this run does not explain the difference.
- **Prediction 3 held:** all threads against `threads=1`, 3.6–4.0× in relation-phase wall.
- **Prediction 1 held** (identity above).
- **The batching cost:** with one thread the native sweep is 0.56–0.93× the plain Rust arm, consistent
  with its sweeping 1.02–2.16× the candidates (wall 1.05–1.85×; no run isolates the cause, *corrected*):
  it sweeps every candidate of every target a lane round presents (the CPU arm stops at a target's first
  hit and sweeps only the targets the run asks for). That uncharged work is reported as `gpu_*`
  counters, as for the GPU arms.
- **Solve wall:** at `m = 2` the native arm's solve is 2.9–3.3× one thread; at `m = 3` and 2048 only
  1.18–1.52×, because the CPU-built pair table and linear algebra are unchanged, and the native arm
  also builds a 64 MiB probe set of the pair table's abscissae that no counter records (its set-up wall
  is 0.13–0.21 s above the one-thread arm's; *corrected*).
- **Distance to rho:** the same on every IC arm: 106–24 637× the matched rho at w = 0, 151–24 680× at
  w = 0.1 and 293–24 875× at w = 1 (§5a band, *corrected*: previously w = 0 only); rho is ahead at every
  weight.

## What this changes

The strongest CPU baseline for the GPU comparisons is now measured on one host: the in-process Rust
sweep and the emulation arm are equal within 11% on 4 threads. The GPU rounds' multi-threaded arm was
therefore not handicapped by its code, at least on this host; whether it was by oversubscription on the
96-thread pod stays untested. A vectorised (SIMD) field kernel was not attempted.

## Not done / next

- Round 5 (`research/ic_gpu_rootfind_la_20261115`, GPU arms with `incremental-gauss-echelon`): frozen;
  blocked on RunPod funds.
- A SIMD field kernel for the CPU sweep, and the native arm on a many-core host beside the GPU.
- Process (validator): raw data in Git, not S3 (`uploaded: false`; this session's AWS keys are
  rejected, the same waiver as earlier rounds); the record's PDF field names REPORT.md; the replays ran
  on the same host and binary.
