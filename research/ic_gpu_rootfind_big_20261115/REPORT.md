---
title: "rootfind on the GPU: m = 3 above 32 bits, a device-built probe table, and a multi-threaded CPU arm"
date: 2026-11-15
---

# rootfind on the GPU: m = 3 above 32 bits, a device-built probe table, and a multi-threaded CPU arm

User request (2026-11-15), "do these", on the open items of PR #4217: build the `m = 3` probe table on
the device; run `m = 3` above 32 bits; a validator check of `research/ic_gpu_rootfind_m3_20261115`;
a multi-threaded CPU comparison.

**Standing.** `[pcc:unverified]`: one A100 pod, measured by the session that wrote the code. An
independent validator (`VALIDATION_REVIEW.md`) re-verified all 240 certificates with its own code,
proved the four curves' group orders, replayed the 51 records that need no GPU on another hardware
class (all identical; the 72 `backend=cuda` records refuse without a device, so they replay only on a
GPU host), and recomputed every number: **validated, scoped**; its corrections are applied below
(*corrected*). ecbench rates the wall comparisons "measurement quality 0/100, admitted 0" (L0). Counted S is unchanged by design and measured unchanged in every pair;
the differences are wall time. Every record is L0 (`--cpus none --allow-busy`) on a shared pod, and
the CPU arm's A/A copy differs from it by up to 16% in solve wall, so differences inside that band are
noise. No scoreboard or leaderboard row is added.

![Solve wall per arm, [pcc:unverified]](fig_big.svg)

## Validator check of the previous report

`research/ic_gpu_rootfind_m3_20261115/VALIDATION_REVIEW.md`: **validated, scoped**. The validator
re-verified all 232 certificates with its own code and replayed all 232 records on a CPU host with an
independently built worker and daemon (a hit-dropping worker fails the `m = 3` replays). Its findings
are applied to that report: one rounding (3.2× → 3.1×), the device relation-phase range (0.010–0.029 s,
slower than the CPU in 3 of 8 cells), the pair table's share (0.4–51× the relation phase in GAE, not
"as much as"), the daemon's gain not being set-up alone, its start being paid per daemon start, an
orphaned daemon left by the aborted first `job_rf3.sh`, and the amendment's heading time. [pcc:unverified]

## What was built (commit `7417c99292`, pushed before the run)

- **Device-built probe table** (`RF_LOOKUP_HASH_DEVICE`): the probe set's packed hash slots are filled
  on the device, one `atomicCAS` insert per key; nothing is sorted on the host any more (Rust stops
  sorting the pair table's abscissae). `rootfind-subtract-gpu` parameter `probe_build=device|host`
  (default `device` for `m = 3`; `host` is the previous round's path, kept as an arm).
- **`backend=any|cuda|emulate`**: `cuda` refuses to run without a device (checked locally: status −2),
  so the backend is now part of each record's method identity; `emulate` runs the kernel's per-thread
  code on every host thread, which is the multi-threaded CPU arm.
- **Curves above 32 bits**: ecbench's `prime_search` stops at 32 bits, so `make_curves.py` generates
  `prime_explicit` prime-order curves at 34, 36, 38 and 40 bits (order by baby-step giant-step on a
  random point over the Hasse interval, kept only when a single multiple fits and it is prime; checked
  by a brute-force count at 20 bits and by [N]Q = O for 20 random points at each size).
- Tests (host worker): `m = 2`, `m = 3`, daemon, `backend=emulate` and both probe builds equal
  `rootfind-subtract` over whole pipelines [pcc:unverified].

## Run

One A100 80GB PCIe pod (`t0bqkj2h4g2dgo`, Intel Xeon Gold 6240R host, 96 threads), CUDA 12.8,
04:43Z–06:12Z 2026-11-15, about $3.10, deleted. `ecbench` built locally from `7417c99292` (the benchmark
record was initialised on that commit with a clean tree; the sessions record no git commit and the
pod-built worker's sources are not hashed, so the build provenance is not verifiable from the artifacts), `rootfind_serve_cuda` built on the pod
from the same tree; one daemon started once (context 0.382 s) with `RF_SERVE_TIMING=1`; the job
required the CUDA backend in the daemon and the worker. Sessions as frozen: `specs/smoke.json`,
`specs/rf3big.json` (34–40 bits; \|F_x\| 2048 and 4096; arms CPU 1 thread, multi-threaded CPU
(`backend=emulate`), GPU daemon with the device-built table, GPU daemon with the host-built table at
2048, CPU A/A at 2048; rho and rho A/A), `specs/rf2mt.json` (28/30/32 bits, \|F_x\| 4096: CPU,
multi-threaded CPU, GPU daemon; rho). Raw `raw/out_rbig.tgz` (sha256 `4144d049…`); tables
`RESULTS.md`, `results.json`.

## Results [pcc:unverified]

- **Identity:** smoke 32/32, rf3big 160/160, rf2mt 48/48 verified; `ecbench verify --replay-all`
  reproduced all 240. 136 checks of every other IC arm against the CPU arm (by workload and round):
  equal S, total GAE, recovered logarithm, relations, targets and relation-phase additions, 0
  mismatches. The device-built table's first CUDA run is in the smoke session and matches. [pcc:unverified]

### `m = 3` above 32 bits (median of 4)

| log2 r | \|F_x\| | S / rho | CPU solve | multi-thread solve | GPU solve | GPU vs CPU | GPU vs multi-thread | relation phase CPU → GPU |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 33.4 | 2048 | 133 | 2.74 s | 2.75 s | 2.56 s | 1.07× | 1.08× | 0.586 → 0.044 s |
| 35.2 | 2048 | 152 | 4.22 s | 3.41 s | 2.81 s | 1.50× | 1.21× | 1.77 → 0.054 s |
| 37.4 | 2048 | 188 | 10.5 s | 4.49 s | 2.91 s | 3.59× | 1.54× | 7.83 → 0.113 s |
| 39.3 | 2048 | 521 | 32.6 s | 7.74 s | 2.93 s | 11.1× | 2.64× | 30.2 → 0.245 s |
| 33.4 | 4096 | 230 | 8.66 s | 11.6 s | 8.56 s | 1.01× | 1.35× | 0.398 → 0.070 s |
| 35.2 | 4096 | 129 | 15.5 s | 17.6 s | 16.0 s | 0.97× | 1.10× | 1.26 → 0.077 s |
| 37.4 | 4096 | 107 | 25.3 s | 19.5 s | 17.3 s | 1.47× | 1.13× | 8.09 → 0.089 s |
| 39.3 | 4096 | 274 | 48.0 s | 21.3 s | 17.6 s | 2.72× | 1.20× | 31.5 → 0.152 s |

- Above 32 bits the `m = 3` relation phase grows (8–32 s on the CPU at 37–39 bits) and the device takes
  it to 0.09–0.25 s there (69–208×; 5.7–33× at 33–35 bits, *corrected*; at 33.4 bits, 4096, every run splits its batches four times and the multi-threaded arm is slower than one thread). At \|F_x\| = 2048 the solve is then 3.6× (37.4 bits) and 11.1× (39.3
  bits) faster than one CPU thread, and 1.5× and 2.6× faster than the multi-threaded arm. [pcc:unverified]
- At \|F_x\| = 4096 what remains is the pair table (≈ 16.8M sums built on the CPU: oracle set-up 5.1–5.8 s
  on the CPU arm, 5.8–6.4 s on the GPU arm and 7.2–7.5 s on the multi-threaded arm, *corrected*) and linear algebra (2.8–11 s, the same code on every arm), so the solve gains only
  0.97–2.72×.
- **Device-built against host-built table** (\|F_x\| = 2048, same session): the GPU arm's oracle set-up
  falls by 0.46–0.77 s (1.92–2.06 s → 1.28–1.46 s). On the worker, creating the session takes a median
  0.044 s (0.028–0.066) with the device-built hash against 0.780 s with the host build (daemon timing
  lines, CUDA, \|F_x\| = 2048; *corrected*: 0.185 s pooled emulation arms, both sizes and replays). The
  host arm's table is a sorted Eytzinger table, so the comparison also changes the table type. The GPU
  set-up is now 0.09–0.80 s above the CPU arm's (which builds the same pair table), against 0.12–0.57 s
  on the smaller curves of the previous round with sets four times smaller.
- **Distance to rho** (at w = 0): S is 107–521× the matched rho's mean S; in wall time the best GPU
  arm is 7–254× slower than rho's median solve (37–417 ms). The validator's §5a band: rho is ahead at
  w = 0, 0.1 and 1 (`m = 3` 107–946×, `m = 2` 8 468–22 867×).

### `m = 2`: multi-threaded CPU against the GPU (\|F_x\| = 4096, median of 4)

| log2 r | S / rho | CPU solve | multi-thread solve | GPU solve | multi-thread vs CPU | GPU vs multi-thread |
|--:|--:|--:|--:|--:|--:|--:|
| 27.5 | 8 468 | 3.07 s | 0.623 s | 0.153 s | 4.9× | 4.1× |
| 30.0 | 10 858 | 14.4 s | 2.88 s | 0.202 s | 5.0× | 14.2× |
| 31.3 | 22 784 | 39.9 s | 7.81 s | 0.323 s | 5.1× | 24.2× |

**The multi-threaded arm is a weak baseline.** It is the worker's host emulation: the kernel's
per-thread function on 96 host threads, and it reaches only about 5× one thread at `m = 2` (where
"one thread" is the separate Rust implementation). *Corrected:* creating a thread pool per batch costs
only 3.4–4.4 ms of a 36–47 ms batch (≤ 10%); the main cause is that the emulation code sweeps about
1/7–1/9 as many candidates per core as the single-thread Rust arm. A tuned parallel CPU sweep would
close part of the 4–24× gap; how much is not measured here.

**Device work not charged** (*corrected*, previously unnamed): 18–986 swept targets per `m = 3` run that
the run never asked for, plus the split batches, as `gpu_*` counters.

## Not done / next

- The `m = 3` pair table is still built on the CPU (5–6 s at \|F_x\| = 4096, charged on every arm); at
  that size it and linear algebra now bound the solve.
- A tuned multi-threaded CPU sweep (persistent thread pool) for a fair parallel baseline.
- Process (validator): raw data in Git, not S3 (`uploaded: false`); a code comment saying CPU hosts
  can replay every record is wrong for `backend=cuda` records.
