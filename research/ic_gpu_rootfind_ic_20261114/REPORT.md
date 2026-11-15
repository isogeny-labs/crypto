---
title: "rootfind in a complete index-calculus solve (rootfind-subtract-gpu), and Eytzinger prefetch"
date: 2026-11-14
---

# rootfind in a complete index-calculus solve, and Eytzinger prefetch

User request (2026-11-14), "do these": (1) an independent replay of the rootfind lookup measurement;
(2) hook rootfind into a measured index-calculus run; (3) try prefetching the next level of the
sorted search.

**Standing.** `[pcc:unverified]`: one A100 pod, measured by the session that wrote the code. An
independent validator (`VALIDATION_REVIEW.md`, 2026-11-15) re-verified all 88 certificates with its own
curve code, replayed every record on a CPU host through the host-emulation worker (72/72 and 16/16
reproduced) and recomputed every number: disposition **validated, scoped**, no number refuted; its
wording corrections and disclosures are applied below (marked *corrected*). The counted cost S is unchanged by design and measured unchanged; the
gain is wall time on a GPU, not a counted-cost improvement, and it adds no scoreboard or leaderboard
row. The method measured (two-summand subtraction, `m = 2`) is far behind Pollard rho in S, and its
gap grows with the curve (below).

## (1) Validator check of the lookup report

An independent validator agent recomputed every headline number of
`research/ic_gpu_rootfind_lookup_20261114/REPORT.md` from its raw tarball (sha256 matches): disposition
**validated, scoped** (`research/ic_gpu_rootfind_lookup_20261114/VALIDATION_REVIEW.md`). No number was
refuted; four wording errors were corrected in that report (paired lookups alone speed the split-array
hash up by 1.5–3.3% at |F| = 1024; 15–20% rather than 15–21%; packed slots alone 0.67–0.71×; the
measured revision is `c308625f57`, whose `gpu/` tree equals `1e757ab551`'s).

## (2) `rootfind-subtract-gpu`: rootfind inside a complete IC solve

![CPU against GPU arm, median wall per workload, [pcc:unverified]](fig_ic.svg)

### What was built (commit `aeeb30b883`)

- `gpu/rootfind`: a persistent Fp64 session (`rf64_session_create/decompose/destroy`, CUDA and host
  emulation): the factor base and its lookup table stay on the device; a batch of targets is two
  copies in, one launch, the relations out. A worker `rootfind_serve` (`make serve`, host emulation;
  `make serve-cuda`, the CUDA backend) speaks a small binary protocol on stdin/stdout.
- `src/cryptanalysis/ic_framework/rootfind_gpu.rs`: the decomposition oracle `rootfind-subtract-gpu`
  (`m = 2`), registered in `ic bench` and `ecbench`'s `ic.pipeline`. It is `rootfind-subtract` with the
  sweep done on the worker: every candidate `R ∓ F_i` of every target in a batch is computed by the
  kernel; each target's hits are sorted into the CPU sweep's order and confirmed one by one through the
  CPU oracle's own lookup and group re-addition. A base that is not negation-closed is refused (that
  is what guarantees the kernel's `j ≥ i` hits contain the CPU's first hit); targets whose abscissa is
  in the base go to the CPU oracle whole. No worker means `prepare` fails. *Corrected:* that does not
  stop a `-gpu` arm from running on the host-emulation build of the worker (a multi-threaded CPU run of
  the same kernel code), and the backend is not in the record; in this run every GPU record's worker
  stderr names the A100, so the figures below are device figures. The `j ≥ i` argument also needs the
  kernel to miss no hit, which is supported by the tests, not proven.
- **Charged exactly as `rootfind-subtract`** (the `fes-f2-gpu` precedent): the batched inversion, one
  addition per candidate up to the first hit, every lookup, the confirming addition. What the device
  computes beyond that (the rest of a decomposing target's sweep, lane targets swept but never asked
  for: `gpu_targets_unused`, *corrected:* 3–984 a run, median 454) is reported as `gpu_*` native
  counters, not charged. The validator puts the device's computed additions at 1.01–4.88× the charged
  ones per run (1.17× overall).
- Batches come from the lane walks: new target sources `walk-prime-batch-256` and `-1024` (lane count
  widened from u8 to u16); one lane round is one kernel launch.

### Checks before the run

- `cargo test --lib rootfind_gpu` with the host worker: 48 whole-pipeline runs (four prime curves,
  14–20 bits; lane and walk targets; two seeds, two planted logs) equal to `rootfind-subtract` in
  relations, recovered logarithm, every phase's ledger and S. Positive control: flipping the ε
  mapping makes the test fail (`recovered: Some(11)` against `None`) [pcc:unverified].
- ecbench smoke (20-bit curve, 16 records) locally with the host worker and on the pod with the CUDA
  worker: 16/16 verified, replay audit OK, CPU and GPU arms equal S per workload.

### Run

One A100 80GB PCIe pod (`1ku6389hniumbb`, AMD EPYC 9454 host), CUDA 12.8, 23:00Z–23:42Z 2026-11-14
(*corrected*: the job's first command is stamped 23:00:22), about $1.50, deleted. `ecbench run` of
`specs/main.json` (byte-identical to the session's `spec.json`; committed locally in `aeeb30b883` before
the run, but first pushed at 23:26Z, after it, so the freeze rests on the local commit date): three
prime-order curves (`prime_search`, 28/30/32 bits, log2 r = 27.5, 30.0, 31.3), two targets each, two
rounds; arms `rho-neg` (reference), `rf2-cpu-F{4096,16384}` and `rf2-gpu-F{4096,16384}` (prime-abscissa
base of 4096 or 16384 abscissae, `walk-prime-batch-1024` targets, `incremental-gauss`), `rho-neg-aa`.
The CPU arm is one thread; the GPU arm is one thread plus the device. Every record is L0
(`--cpus none --allow-busy`), and there is no A/A copy of an IC arm, so `ecbench compare` admits none of
the wall comparisons (quality 0/100); the wall figures are descriptive. The GPU arm also switches the
relation loop's repeat guard to the cache-line set (the one that touches lane rounds ahead); its counts
are identical and its wall effect is not separated. The session records no git commit; the measured
revision rests on the job's `REV` file. `job.sh`; raw `raw/out_ric.tgz` (sha256 `8ab25665…`); tables
`RESULTS.md`, `results.json`.

### Results

- **Identity:** 72/72 records verified; `ecbench verify --replay-all`: 72 replays reproduced, 0
  problems. All 24 CPU/GPU pairs have equal S, total GAE, recovered logarithm, relations, targets and
  relation-phase additions (0 mismatches) [pcc:unverified].
- **Wall time** (median of 4 paired runs each) [pcc:unverified]:

| log2 r | \|F_x\| | mean S | S / rho | CPU solve | GPU solve | solve | CPU relations | GPU relations | relations |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 27.5 | 4096 | 6 259 | 6 403 | 1.88 s | 0.489 s | 3.9× | 1.81 s | 0.070 s | 25.8× |
| 27.5 | 16384 | 3 639 | 3 723 | 2.72 s | 1.07 s | 2.5× | 1.96 s | 0.091 s | 21.4× |
| 30.0 | 4096 | 18 007 | 13 154 | 12.4 s | 0.507 s | 24.5× | 12.3 s | 0.146 s | 84.4× |
| 30.0 | 16384 | 14 371 | 10 498 | 12.1 s | 1.39 s | 8.7× | 11.1 s | 0.143 s | 77.8× |
| 31.3 | 4096 | 26 010 | 22 986 | 28.0 s | 0.643 s | 43.5× | 27.9 s | 0.246 s | 113.2× |
| 31.3 | 16384 | 23 265 | 20 560 | 28.6 s | 1.51 s | 18.9× | 27.5 s | 0.227 s | 121.2× |

- **Where the GPU solve's time goes now:** the worker start and CUDA context (oracle set-up, medians
  0.25–0.35 s per workload, single solves 0.215–0.391 s, because ecbench runs each record in a fresh
  process) and, at \|F_x\| = 16384, linear algebra (medians 0.69–0.94 s on the GPU arm and 0.71–1.03 s on
  the CPU arm; the two arms run the same code). The relation phase is 0.07–0.25 s.
- **Distance to rho** (unchanged by the GPU): S is 3 700–23 000× the matched rho's mean S (at w = 0;
  the validator finds IC above rho at w = 0, 0.1 and 1 GAE per native operation on every curve), growing
  with r, as expected of a two-summand subtraction search (relation work heuristically ∝ r against
  rho's √r; *corrected:* the three sizes fit r^1.05 and r^1.22, a heuristic, not a measured law). In
  wall time the GPU arm is still 115–787× slower than rho (median 1.4–4.5 ms a solve) [pcc:unverified].

## (3) Eytzinger prefetch: a negative result

`RF_EYT_PREFETCH = d` prefetches `eyt[k << d]` (the node d levels down; d = 4 is one 128-byte line of 16
keys) at every step of the descent; `RF_EYT_PREFETCH_L2 = 1` into L2 only. Six builds of one tree
(default D, its A/A copy D2, d = 3, 4, 5, and d = 4 into L2), every one past the host tests and the three
device selftests, five rotated rounds of `./bench throughput` [pcc:unverified]:

- A/A within 0.12% (\|F\| = 65536) and 0.16% (\|F\| = 1024).
- The sorted path gets **slower** with every prefetch variant: 1.31–1.92× at \|F\| = 65536 and
  1.21–1.81× at \|F\| = 1024 (per configuration in `RESULTS.md`). The hash path does not use it and is
  unchanged (within 0.5%).
- At each workload's best configuration the default stays best (sorted/C16 360.27 ms at \|F\| = 65536;
  hash/C16 17.71 ms at \|F\| = 1024, untouched by the flag).

A hypothesis, without profiler evidence (*corrected*): the extra instruction and the divergent
per-thread prefetch addresses cost more than the latency they hide, and the paired lookups already
overlap two descents. (`RESULTS.md` first printed its "best configuration per arm" ratios inverted (0.883 for an arm 1.13×
slower); `analyze.py` is fixed and `RESULTS.md` regenerated from the same tarball.) The flag stays in the tree, default off, so the
negative result can be reproduced.

## Not done / next

- `m = 3` (the pair-table probe, the stronger CPU arm of the bitop ladder) on the GPU: the kernel looks
  candidates up only in the base [pcc:unverified].
- The per-solve worker start (0.3 s) dominates small solves; a worker shared across ecbench records
  would remove it but would tie records together.
- A multi-threaded CPU arm for a fairer wall comparison; the pipeline's relation loop is
  single-threaded.
- §15 (the fastest external implementation) is not addressed; the sessions live only inside the
  tarball (no S3 manifest), so CI cannot re-audit them.
