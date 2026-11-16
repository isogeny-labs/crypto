---
title: "Batched inversions in the device pair-table key kernel (round 6): set-up only 7% lower"
date: 2026-11-15
---

# The device pair-table key kernel with batched inversions (round 6)

User request (2026-11-15): "keep working on open leads". After round 5
(`research/ic_gpu_rootfind_la_20261115`) the GPU `m = 3` arm's set-up at |F_x| = 4096 (0.12 s) was one
of three equal bounds, and pairs-mode session creation took a median 0.112 s, linear in the number of
pairs. The hypothesis was the key kernel's two Fermat inversions per cell.

**Standing.** `[pcc:unverified]`: one A100 pod, measured by the session that wrote the code; no
validator yet. L0; walls descriptive. Counted S is unchanged by design and measured unchanged. No
scoreboard or leaderboard row is added.

![Pair-build stages, [pcc:unverified]](fig_stages.svg)

## What was built (commit `0c8c431185`, pushed before the run)

`rf_pair_run` (`gpu/rootfind/rootfind.cuh`): one thread per run of 16 partners of a row; the run's
denominators (2y on the diagonal, x_b − x_a elsewhere, shared by both signs) are inverted together
(prefix products, one inversion, backward pass), and both signs are written from one inverse. The keys
are `rf_pair_x`'s bit for bit: a host test compares every cell (rows not a multiple of 16, a
duplicated abscissa through the per-cell fallback), and an off-by-one in the backward pass fails 1 230
checks. The host (emulation) build uses the same runs; the Rust pairs tests pass on it. The old kernel
is kept as `k_pair_keys_cell` for `rootfind_serve --bench-pairs`, and `RF_SERVE_TIMING` now prints the
build's stages. The `ecbench` binary's sha256 was recorded at dispatch (`ecbench.sha256`).

## Run

`PROTOCOL.md`; one A100 80GB PCIe pod, 22:39–22:48Z (the launch command 22:26–22:49Z), deleted;
`benchrec run` of `run_pod.sh` (record `20261115T2225Z-ic-gpu-rootfind-setup`). Raw `raw/out_rsu.tgz`;
tables `RESULTS.md`, `results.json`, `band.json`.

## Results [pcc:unverified]

- **Prediction 1 held:** `--bench-pairs` on the device, 0 cells differ at n = 1024, 2048 and 4096; the
  batched kernel is 4.0×, 5.1× and 5.7× faster (4.7 ms → 0.8 ms at 4096).
- **Prediction 2 held:** smoke 36/36 and rf3setup 84/84 verified; `verify --replay-all` reproduced all
  120; 36 checks of the GPU arms against the CPU echelon arm equal on trials, relations, logarithm, S,
  total GAE and every phase's charged fields; 0 mismatches. [pcc:unverified]
- **Prediction 3 failed:** the GPU arm's median set-up at |F_x| = 4096 is 0.109–0.116 s, against
  round 5's 0.121–0.122 s (cross-session), not at most half; session creation median 0.105 s (round 5:
  0.112 s).
- **Why:** the key kernel was never the bottleneck (4.7 ms of ~0.1 s). The stage timings at
  |F_x| = 4096 (median of 48): keys 1.9 ms, dedupe 6.9 ms, compact 5.1 ms, probe insert 2.9 ms, and the
  copy of the 16.8M-entry probe-index → build-index map (67 MB) to a fresh pageable host buffer
  **47 ms**. At 2048: 1.3 / 2.0 / 2.0 / 1.6 ms and the copy 15.5 ms.
- GPU echelon solve: 0.115 s and 0.220 s at 2048, 0.255 s and 0.292 s at 4096 (37.4 and 39.3 bits);
  A/A per pair 0.87–1.13. S / rho at w = 0 / 0.1 / 1: 220–685 / 233–685 / 292–689; rho ahead at every
  weight. Rho's median solve 0.033 s and 0.069 s on one thread.

## What this changes

The key kernel is 5.7× faster [pcc:unverified] and the hypothesis that it bounded set-up is refuted by the stage
timings, which name the 67 MB host copy instead. Round 7 (`research/ic_gpu_rootfind_devmap_20261115`)
keeps that map on the device.

## Not done / next

- Round 7 (the map on the device): frozen; its first launch found no A100 available.
- A validator check of this report.
- Raw data in Git, not S3 (this session's AWS keys are rejected; the same waiver as earlier rounds).
