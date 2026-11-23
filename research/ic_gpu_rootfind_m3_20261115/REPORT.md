---
title: "rootfind on the GPU: m = 3 against the pair table, and a daemon worker"
date: 2026-11-15
---

# rootfind on the GPU: m = 3 against the pair table, and a daemon worker

User request (2026-11-15), "do these", on the open items of PR #4178: (1) a validator check of
`research/ic_gpu_rootfind_ic_20261114/REPORT.md`; (2) `m = 3` on the GPU; (3) remove the ~0.3 s GPU
start-up each solve paid.

**Standing.** `[pcc:unverified]`: one A100 pod, measured by the session that wrote the code. An
independent validator (`VALIDATION_REVIEW.md`) re-verified all 232 certificates with its own code,
replayed all 232 records on a CPU host with an independently built emulation worker and daemon
(positive control: a hit-dropping worker fails the m = 3 replays), and recomputed every number:
**validated, scoped**, no number refuted beyond one rounding; its corrections are applied below
(*corrected*). ecbench rates every wall comparison here "measurement quality 0/100" (L0, no
admitted pairs): the wall figures are descriptive. Counted S is unchanged by design and measured unchanged in every pair;
the gains and losses below are wall time, every record is L0 (`--cpus none --allow-busy`), and no
scoreboard or leaderboard row is added.

![Solve wall per arm, [pcc:unverified]](fig_m3.svg)

## (1) Validator check of the previous report

`research/ic_gpu_rootfind_ic_20261114/VALIDATION_REVIEW.md`: **validated, scoped**. The validator
re-verified all 88 certificates with its own curve code, replayed every record on a CPU host through
the host-emulation worker (72/72 and 16/16 reproduced), and recomputed every number; none was
refuted. Ten statements were corrected or qualified in that report (the spread of
`gpu_targets_unused`, per-solve ranges, that the worker's backend is not recorded, that the `j ≥ i`
argument rests on the kernel missing nothing, the r-scaling being a heuristic, L0 and no IC A/A arm,
the wall-time distance to rho, the start time, the prefetch cause being a hypothesis) and its
`RESULTS.md` ratio label was fixed. This round's protocol took up its process findings: IC A/A arms,
specs pushed to GitHub before the run (`219333b6ab`), the benchmark record initialised on that clean
commit, and the job aborting unless the daemon and the worker report the CUDA backend. [pcc:unverified]

## What was built (commit `219333b6ab`)

- **Kernel probe mode** (`gpu/rootfind/rootfind.cuh`): the candidates `x(R ∓ F_i)` are looked up in an
  arbitrary abscissa set instead of the base, every hit is reported (no `j ≥ i` filter), and the lookup
  table is built on that set. The session API and the worker protocol (RFS2) carry the set.
- **`rootfind-subtract-gpu:m=3`**: the set is the abscissae of the negation-folded pair table the CPU
  oracle builds and charges (`PairTable::map_abscissae`; the table is keyed by the sum's abscissa).
  A position the device does not report is one where the CPU probe misses its key, so the reported
  positions, confirmed in order through the CPU probe, give `rootfind-subtract:m=3`'s first hit and
  charge.
- **`worker=daemon`**: `rootfind_serve --daemon SOCKET` creates its CUDA context once and serves one
  connection at a time; a run connects instead of starting a worker. `worker=spawn` (the default) is
  the previous behaviour. The mode is a method parameter, so every record names it. The daemon's
  start is a one-time host cost outside every record and is reported below.
- Overflowing batches are split; the worker connection is per thread.

**Checks before the run** [pcc:unverified]: whole-pipeline equality to `rootfind-subtract` for `m = 2`,
`m = 3` and the daemon (host worker, four curves of 14–20 bits); positive control: a worker that drops
the probe hits with `j mod 7 = 3` fails the `m = 3` test (relation-phase additions 4 545 against 4 579);
local ecbench smoke 28/28 verified and replayed.

## Run

One A100 80GB PCIe pod (`zydj9olc9q3d6w`), CUDA 12.8, 02:48Z–03:14Z 2026-11-15, about $1.10, deleted.
Binaries: `ecbench` built locally from `219333b6ab` on a clean tree (stated by the job's `REV` file;
the sessions record no git commit and the binary embeds none, so this is not verifiable from the
artifacts), `rootfind_serve_cuda` built on the pod from the same tree (sources not hashed in the
tarball). `job.sh` ran `specs/smoke.json` and `specs/rf2d.json` as frozen; the frozen
`specs/rf3.json` was refused by ecbench before any record (`prime_search` takes at most 32 bits), so
`AMENDMENT_1.md` (pushed before the rerun) moved its curves to 26/28/30/32 bits, nothing else changed,
and `job_rf3.sh` ran it on the same pod and binaries. The first try of `job_rf3.sh` aborted on a stale
socket file (its wait loop is fixed in `4961ee3cd1`, pushed at 03:11:41Z, after rf3 had started); no
record was taken, but that try exited without killing the daemon it had started, which was probably
left running and unreachable (its socket was replaced) during rf3 (*corrected*: undisclosed before). Raw `raw/out_rm3.tgz`
(sha256 `1742922a…`); tables `RESULTS.md`, `results.json`.

## Results [pcc:unverified]

- **Identity:** smoke 28/28, rf3 144/144, rf2d 60/60 verified; `ecbench verify --replay-all`
  reproduced all 232. 128 GPU-vs-CPU pair checks (spawn, daemon and the IC A/A arms against the CPU
  arm, by workload and round): equal S, total GAE, recovered logarithm, relations, targets and
  relation-phase additions, 0 mismatches. [pcc:unverified]
- **The daemon:** CUDA context ready 0.376 s after launch, socket listening at 0.383 s (second start
  for the amendment run: 0.291 s). *Corrected:* it is paid once per daemon start (three starts in
  this run), not per solve, and part of it lands in the first record after each start (about 0.055 s
  more relation-phase wall).

### (3) Start-up: daemon against a worker per run (`m = 2`, \|F_x\| = 4096, median of 4)

| log2 r | S / rho | CPU solve | spawn solve (A/A copy) | spawn set-up | daemon solve | daemon set-up | daemon vs CPU | daemon vs spawn |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 27.5 | 5 847 | 2.17 s | 0.367 s (0.322 s) | 0.208 s | 0.105 s | 0.004 s | 20.8× | 3.5× |
| 30.0 | 11 393 | 9.67 s | 0.382 s (0.393 s) | 0.209 s | 0.121 s | 0.002 s | 79.6× | 3.1× |
| 31.3 | 22 198 | 27.4 s | 0.424 s (0.506 s) | 0.156 s | 0.214 s | 0.004 s | 127.8× | 2.0× |

The per-solve set-up medians fall from 0.16–0.25 s to 2–4 ms; the spawn arm's A/A copy differs from it
by 3–19% in solve wall (CUDA start varies), so the daemon's 2.0–3.5× is well outside that noise.
*Corrected:* the daemon arm's relation phase is also 0.054–0.058 s shorter (the spawned worker loads
its kernels lazily on first launch), so the 2.0–3.5× is not set-up alone. The
daemon's one-time 0.38 s equals about two spawned solves' set-up. [pcc:unverified]

### (2) `m = 3` on the GPU: correct, but slower end to end at these sizes

Median of 4; CPU arm A/A within 2% (`rf3-cpuaa-F1024`, \|F_x\| = 1024 only; there is no A/A copy of
any GPU or daemon m = 3 arm).

| log2 r | \|F_x\| | S / rho | CPU solve | CPU set-up | daemon solve | daemon set-up | daemon vs CPU | relation phase CPU → daemon |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 25.2 | 1024 | 179 | 0.107 s | 0.084 s | 0.247 s | 0.215 s | 0.43× | 0.013 → 0.021 s |
| 27.8 | 1024 | 178 | 0.116 s | 0.084 s | 0.232 s | 0.208 s | 0.50× | 0.019 → 0.010 s |
| 29.2 | 1024 | 52 | 0.135 s | 0.084 s | 0.259 s | 0.229 s | 0.52× | 0.030 → 0.010 s |
| 31.2 | 1024 | 43 | 0.214 s | 0.084 s | 0.249 s | 0.215 s | 0.86× | 0.107 → 0.012 s |
| 25.2 | 2048 | 672 | 0.507 s | 0.450 s | 1.23 s | 0.948 s | 0.41× | 0.031 → 0.263 s |
| 27.8 | 2048 | 618 | 0.501 s | 0.432 s | 1.08 s | 0.997 s | 0.47× | 0.037 → 0.049 s |
| 29.2 | 2048 | 143 | 0.517 s | 0.430 s | 1.06 s | 0.994 s | 0.49× | 0.045 → 0.029 s |
| 31.2 | 2048 | 54 | 0.602 s | 0.432 s | 1.10 s | 0.994 s | 0.55× | 0.077 → 0.016 s |

- `m = 3` is much closer to rho in S (43–672× against `m = 2`'s 5 800–22 000×, at w = 0; the validator
  finds rho ahead at w = 0, 0.1 and 1, and the rho reference is a 4-run mean that varies 4.1× across
  curves), which makes it the better method to accelerate in our reading (an interpretation). Its
  relation phase is small at these sizes: the pair table (≈ \|F_x\|² additions, CPU, charged, the same
  on both arms) costs 0.4–51× the relation phase in GAE and 0.8–14.5× in wall (*corrected*: not "as
  much as"). On the CPU the relation phase is 0.013–0.107 s.
- The device takes the relation phase to 0.010–0.029 s at log2 r ≥ 27.8 (2.0–8.7× at \|F_x\| = 1024;
  1.5–4.8× at ≥ 29.2, 2048), *corrected:* it is slower than the CPU's in 3 of 8 cells (at 25.2 bits,
  2048, 0.12× the CPU's speed, with 126 batch splits a run: dense hits overflow the relation buffer),
  but the set-up grows by 0.12–0.15 s (1024) and
  0.50–0.57 s (2048). The probe sets hold 1.02–1.05M and 3.77–4.19M abscissae (worker logs); they are
  collected and sorted in Rust, sent over the socket, sorted again and copied into Eytzinger order by
  the worker on the host, then uploaded. Which of those steps costs most was not profiled (a
  hypothesis to test next). The set-up outweighs the saving everywhere here (0.41–0.86× the CPU
  solve); a worker per run is slower still (0.21–0.44×).
- In wall time rho is far ahead of either: the `m = 3` daemon arm is 28–2 040× rho's median solve and
  the `m = 2` daemon arm 25–72×.

## Not done / next

- `m = 3` set-up: build the probe table on the device from the abscissae (or keep it on the device
  across targets of one curve) instead of sorting twice on the host and shipping it; until then the
  device does not pay at `m = 3` on these sizes.
- Larger curves for `m = 3` need `prime_explicit` curves (ecbench's `prime_search` stops at 32 bits).
- Process (validator): raw data in Git, not S3 (the S3 waiver); the sessions only inside the tarball,
  so CI cannot re-audit them; the record's `reports.pdf` field names `REPORT.md`.
- A validator check of this report; a multi-threaded CPU arm; the relation guard the GPU arm switches
  (cache-line set) is still not separated from the device effect.
