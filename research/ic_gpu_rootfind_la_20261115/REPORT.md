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
L0 measurements, descriptive, not admitted comparisons. No scoreboard or leaderboard row is added. An
independent validator (`VALIDATION_REVIEW.md`) re-verified all 168 certificates and the three curves'
orders with its own code, re-derived every planted scalar, matched all 84 identity checks on every
charged field and counter outside linear algebra (inside it only `row_ops` and `gauss_entries_scanned`
differ), recomputed every table cell, and replayed every worker-free record on another hardware class:
**validated, scoped**; its corrections are applied below (*corrected*). The GPU records replay only
where a CUDA device answers, so their walls rest on the pod's same-host audit.

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
measurement 19:16–19:39Z (the launch command ran 18:57–19:39Z), deleted. *Corrected:* the measured
commit differs from the protocol commit `5d618524e2` also in `src/`: `rootfind_gpu.rs` gained
`worker=native` (`2ec4187be0`; additive, not on the daemon arms' path) and `koblitz_index_calculus.rs`
gained huge-page advice in the binary-field pair table and a test-only rewrite (not used by the prime
path); `gpu/rootfind`, `job.sh` and the specs are unchanged. The `ecbench` binary was copied from
`target/release`, not built from `REV` on the pod, and the sessions record no commit, so the binary is
not bound to `8c5ae6a512`; the validator's binary from a later clean commit reproduces every
worker-free record, and the GPU arms' counts equal the CPU arms', but the GPU arms' wall and device path
in that binary stay unbound. The two failed launches left only exit codes (22 and 1) and walls (28 s,
33 s); that nothing ran is consistent with, not shown by, the evidence. Daemon start 0.242 s. `benchrec` refuses "measured" for a record holding failed
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

(Wall columns: medians of 6 runs; S / rho: ratio of means, the band's rule; the CPU arms are one
thread.)

- **Device work not charged** (*corrected*, as in rounds 3 and 4): every GPU record carries
  `work.complete: false`, with `gpu_batches`, `gpu_hits_confirmed_on_cpu`, `gpu_targets_swept` and
  `gpu_targets_unused` unpriced; S is the CPU arm's by identity.
- **What bounds the solve now:** no single phase. At 4096, set-up (the device pair-table build and the
  probe set, 0.12 s), linear algebra (0.10–0.12 s) and the relation phase (0.04–0.08 s) are of one order;
  at 39.3 bits and 2048 the relation phase is the largest (0.16 s of 0.22 s).
- **On the CPU** (*corrected*) the linear-algebra phase falls 9–11.5× in wall (0.207 → 0.023 s at 2048,
  1.43 → 0.125 s at 4096), which alone would give a solve of 0.84–0.99×; the measured 0.79–0.98× also
  carries relation-phase noise on identical counted work (per pair 0.61–1.12×, and no CPU A/A arm), so
  the CPU whole-solve difference is not resolved here. The relation phase (4–19 s) dominates there.
  On the GPU the measured and LA-only ratios agree within 0.004.
- **§5a band** (`band.json`): S per arm against the matched rho at w = 0 / 0.1 / 1. GPU echelon:
  340 / 342 / 350 (37.4, 2048), 194 / 206 / 261 (37.4, 4096), 737 / 738 / 742 (39.3, 2048), 383 / 389 / 417
  (39.3, 4096); GPU gauss at 4096: 284 / 686 and 430 / 642 at w = 0.1 / 1. The arms tie at w = 0, so the
  band does not rank them; rho is ahead at every weight.
- **Distance to rho in wall time:** rho's median solve is 0.040 s (37.4 bits) and 0.087 s (39.3 bits) on
  one CPU thread; the GPU echelon arm's solve (factor base through verification) is 2.6–7.1× slower
  (0.223 s against 0.087 s at 39.3 bits, 2048). That excludes the daemon's start, 0.242 s once per pod
  (0.225 s of it the CUDA context); a cold single-target reading would be 5.4–13.2× (*corrected*).
  That sets an A100 arm against single-thread rho; it is the distance remaining, not a comparison of
  equals. In counted cost (S) the distance is 194–737×, unchanged by any of this.

## What this changes

The GPU `m = 3` arm's linear algebra falls from most of the solve (0.87–0.89) to a third to two-fifths
(0.34–0.42) at |F_x| = 4096, and its solve to a fifth (*corrected*). The arms measured here and in
`research/ic_la_m3_20261115` give the same answer and S with `incremental-gauss-echelon` (*corrected*:
not shown for every arm). The next bounds are the set-up at 4096 and the relation phase at 2048.

## Not done / next

- The set-up at 4096 (0.12 s: the device pair-table build and probe set) and the relation phase.
- Process (validator): raw data in Git, not S3 (`uploaded: false`; this session's AWS keys are rejected;
  the waiver was granted in conversation and is not a committed object), so CI's cross-host re-audit
  cannot see the sessions; the records' `commit` fields name the commits at record creation and
  analysis (`764d560062`, `1deabdf708`), not the measured `8c5ae6a512`; r3's command points at a
  scratch directory and the `ecbench band` step is not a recorded command; `reports.pdf` names
  REPORT.md and the metrics carry no spread; the upload-retry comment in `run_pod.sh` dates the 404s
  "18:55-19:05Z", but they began 18:54:15Z and 18:55:18Z.
