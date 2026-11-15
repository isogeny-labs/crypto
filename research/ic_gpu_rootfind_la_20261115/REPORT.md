---
title: "m = 3 rootfind on the GPU with incremental-gauss-echelon: the solve at 0.18–0.20× at |F_x| = 4096"
date: 2026-11-15
---

# m = 3 rootfind on the GPU with incremental-gauss-echelon (round 5)

User request (2026-11-15): "Keep working on open leads" (then "All"). Lead from
`research/ic_la_m3_20261115`: `incremental-gauss-echelon` does the same job as `incremental-gauss`
(trials, relations, logarithm, S) with a seventh to a tenth of the row operations; round 4 left
linear algebra as most of the GPU arm's solve at |F_x| = 4096.

**Standing.** `[pcc:unverified]`: one A100 pod, measured by the session that wrote the protocol; no
validator yet. Every record is L0 (`--cpus none --allow-busy`); the GPU echelon arm's A/A copy is within
0.98–1.07× of it per pair. Counted S is unchanged by design and measured unchanged. The wall ratios are
L0 measurements, descriptive, not admitted comparisons. No scoreboard or leaderboard row is added.

![GPU solve by phase, [pcc:unverified]](fig_la_gpu.svg)

## Protocol and run

`PROTOCOL.md`, `specs/rf3la.json` and `specs/smoke.json`, frozen and pushed before the run (`5d618524e2`,
merged with #4310): the prime_explicit curves of 38 and 40 bits, |F_x| = 2048 and 4096, 3 targets, 2
rounds; `rootfind-subtract:m=3` (CPU, one thread) and `rootfind-subtract-gpu:m=3,probe_build=pairs`
(daemon, cuda), each with `linalg=incremental-gauss` and `incremental-gauss-echelon`; the GPU echelon arm
again as an A/A copy at 4096; `rho.negation` and its A/A.

The first launch (17:07Z) failed at pod creation (account balance; record
`20261115T1707Z-ic-gpu-rootfind-la` closed censored). Under the new record
`20261115T1854Z-ic-gpu-rootfind-la-r2`, two launches failed at the upload (HTTP 404 just after the pod
reported ready; each pod deleted at once, nothing ran); the launcher then waited and retried
(`8c5ae6a512`, measured commit), and the third ran: one A100 80GB PCIe pod, cgroup quota 13.6 CPUs,
19:16–19:39Z, deleted. Daemon start 0.242 s. `benchrec` refuses "measured" for a record holding failed
commands, so `20261115T1854Z-ic-gpu-rootfind-la-r2` is closed censored and the measurement is recorded
in `20261115T2002Z-ic-gpu-rootfind-la-r3`, whose command is the analysis of the sha256-checked tarball
(r2's summary names that record "1955Z", which is wrong; it cannot be amended). Raw `raw/out_rla.tgz` (sha256 in `raw/out_rla.tgz.sha256`);
tables `RESULTS.md`, `results.json`; `band.json` (`ecbench band`, default reading); `analyze.py`.

## Results [pcc:unverified]

- **Identity (prediction 1 held):** smoke 36/36 and rf3la 132/132 records verified;
  `ecbench verify --replay-all` reproduced all 168. 84 checks of every IC arm against the CPU gauss arm
  (per workload and round): equal trials, relations, recovered logarithm, status, S, total GAE and every
  phase's charged additions, doublings, scalar multiplications and GAE; 0 mismatches. [pcc:unverified]
- **Prediction 2 held:** at |F_x| = 4096 the GPU arm's median solve with echelon is 0.18× (37.4 bits) and
  0.20× (39.3 bits) its solve with gauss (predicted at most 0.5×); at 2048, 0.41× and 0.58×.
- **Prediction 3 held:** linear-algebra `row_ops` echelon/gauss 0.129–0.142 (predicted 0.10–0.20).

| log2 r | \|F_x\| | S / rho | CPU gauss solve | CPU echelon | GPU gauss | GPU echelon | GPU echelon vs gauss | GPU echelon vs CPU gauss | GPU echelon: set-up / relations / LA |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 37.4 | 2048 | 340 | 5.66 s | 5.39 s | 0.287 s | **0.118 s** | 0.41× | 48× | 0.038 / 0.055 / 0.023 s |
| 37.4 | 4096 | 194 | 8.09 s | 6.40 s | 1.551 s | **0.280 s** | 0.18× | 29× | 0.122 / 0.037 / 0.117 s |
| 39.3 | 2048 | 737 | 20.2 s | 19.7 s | 0.387 s | **0.223 s** | 0.58× | 90× | 0.032 / 0.162 / 0.024 s |
| 39.3 | 4096 | 383 | 20.0 s | 17.8 s | 1.541 s | **0.305 s** | 0.20× | 66× | 0.121 / 0.076 / 0.104 s |

(Medians of 6 runs; the CPU arms are one thread.)

- **What bounds the solve now:** no single phase. At 4096, set-up (the device pair-table build and the
  probe set, 0.12 s), linear algebra (0.10–0.12 s) and the relation phase (0.04–0.08 s) are of one order;
  at 39.3 bits and 2048 the relation phase is the largest (0.16 s of 0.22 s).
- **On the CPU** the same solver change is 0.79–0.98× in solve: there the relation phase (4–19 s) dominates.
- **§5a band** (`band.json`): S per arm against the matched rho at w = 0 / 0.1 / 1. GPU echelon:
  340 / 342 / 350 (37.4, 2048), 194 / 206 / 261 (37.4, 4096), 737 / 738 / 742 (39.3, 2048), 383 / 389 / 417
  (39.3, 4096); GPU gauss at 4096: 284 / 686 and 430 / 642 at w = 0.1 / 1. The arms tie at w = 0, so the
  band does not rank them; rho is ahead at every weight.
- **Distance to rho in wall time:** rho's median solve is 0.040 s (37.4 bits) and 0.087 s (39.3 bits) on
  one CPU thread; the GPU echelon arm is 2.6–7.1× slower (0.223 s against 0.087 s at 39.3 bits, 2048).
  That sets an A100 arm against single-thread rho; it is the distance remaining, not a comparison of
  equals. In counted cost (S) the distance is 194–737×, unchanged by any of this.

## What this changes

The GPU `m = 3` arm's linear algebra falls from most of the solve to about a third at |F_x| = 4096, and
its solve to a fifth; every arm, CPU and GPU, can use `incremental-gauss-echelon` with the same answer
and S. The next bounds are the set-up at 4096 and the relation phase at 2048.

## Not done / next

- A validator check of this report.
- The set-up at 4096 (0.12 s: the device pair-table build and probe set) and the relation phase.
- Raw data in Git, not S3 (this session's AWS keys are rejected; the same waiver as earlier rounds).
