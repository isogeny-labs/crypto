# PS-8: the meet-in-the-middle pair table against the pairs-and-solve kernels

Protocol: [`../protocols/PS-8.md`](../protocols/PS-8.md) (frozen before any measurement; added after PS-0…PS-7 as lead L1 of [`../LEADS.md`](../LEADS.md)).  Plan: [`../PLAN.md`](../PLAN.md).
Kernels compared: [`../ps2_scalar/`](../ps2_scalar/README.md), [`../ps3_bitsliced/`](../ps3_bitsliced/README.md).  Run report with graphs and PDF:
[`../reports/run-20261018T1804Z/`](../reports/run-20261018T1804Z/).  Class: **engineering measurement**.  Commands: [`run_ps8.sh`](run_ps8.sh); raw records: [`raw/`](raw/).
Arm: `examples/pdp_speedup/mitm.rs` (`--arm mitm`).

## 1. Requested and executed

Host: one host class, **host B of PS-3** (Xeon 2.10 GHz KVM guest, 4 vCPU, kernel `6.18.44-fc-v139`; `raw/manifest.json`), run 2026-10-18 18:04-18:32 UTC, binary SHA-256 in `raw/binary_hashes.txt`.
150 timed rows requested (5 arms x 5 rounds x 6 rungs), 150 completed, **0 contended**, 0 refused starts.

| PS-8 requirement | status | evidence |
|:--|:--|:--|
| H1: verdict digest and every root set equal PS-0's on the whole corpus | **verified complete (held)**: `mitm` and `group-scalar` on `l = 8…12` (80 targets each, verdicts and full root sets) and `baseline`, `group-scalar`, `mitm` on the 3-target `l = 13` sample; 13 records, **0 mismatches** | `raw/verify_arms.jsonl`, `raw/verify_l13.jsonl` |
| H2: at `l >= 10` the probe time is at most 1/10 of `group-scalar`'s | **held**: probe / group-scalar = 1/110, 1/137, 1/255, 1/361 at `l = 10…13` (derived from the medians in §2) | §2 |
| H3: break-even `N* = build / (t_group − t_mitm)` at most 100 at every rung `l = 8…12` | **held**: 3.0, 3.3, 4.9, 5.4, 5.5 at `l = 8…12` (7.8 at `l = 13`); against `bs512`: 1.0, 1.7, 3.5, 4.8, 5.4 (6.7 at `l = 13`) | §2 |
| H4: at most 40 bytes per key | **held**: 24.0-29.2 bytes per key | §2 |
| ABAB, five rounds, A/A beside every difference | **verified complete**; A/A spread of the baseline 5.4%, 8.0%, 6.7%, **10.2%**, 3.1%, 3.5% at `l = 8…13` (the `l = 11` spread is above 10%) | `raw/ab_summary.md` |
| Probe time never reported without build time and memory | respected (§2 columns) | §2 |
| Targets of PS-0, not base-point sums | respected (the first 16 refuted PS-0 targets; 6 at `l = 12`; 3 at `l = 13`) | `raw/times.jsonl` |

## 2. Measured

Probe stage, wall nanoseconds per refuted target on one reserved core, medians of five interleaved rounds (`tools/isolated_bench.py run --cpus 3`); build is `setup_secs` of the same runs, outside the timed region and **reported here**:

| l | baseline ms | group-scalar ms | bs512 ms | **mitm probe, µs** | **mitm build, s** (median of 5 runs) | keys | table MiB | bytes/key | probe / group-scalar (B/A median, min...max) | `N*` vs group-scalar | `N*` vs bs512 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|--:|--:|
| 8 | 8.241 | 0.226 | 0.653 | 2.4 | 0.0007 | 16,383 | 0.4 | 24.0 | 0.0106 (0.0104...0.0107) | 3.0 | 1.0 |
| 9 | 37.58 | 0.812 | 1.577 | 6.9 | 0.0026 | 64,008 | 1.5 | 24.6 | 0.0087 (0.0078...0.0089) | 3.3 | 1.7 |
| 10 | 164.6 | 2.584 | 3.587 | 23.4 | 0.0126 | 215,295 | 6.0 | 29.2 | 0.0094 (0.0060...0.0102); group-scalar spread 29.8% | 4.9 | 3.5 |
| 11 | 704.8 | 11.854 | 13.430 | 86.5 | 0.0634 | 1,014,048 | 24.0 | 24.8 | 0.0074 (0.0072...0.0084) | 5.4 | 4.8 |
| 12 | 2914.3 | 43.455 | 44.739 | 170.6 | 0.2388 | 3,968,063 | 96.0 | 25.4 | 0.0039 (0.0037...0.0040) | 5.5 | 5.4 |
| 13 | 12707.1 | 181.363 | 211.016 | 502.8 | 1.4068 | 16,418,703 | 384.0 | 24.5 | 0.0028 (0.0027...0.0029) | 7.8 | 6.7 |

`N*` is derived arithmetic on the recorded medians (build seconds divided by the per-target saving, `(t_arm − t_mitm)` in seconds).  Build time per run varies (`l = 11`: 0.049-0.073 s over five runs; `l = 13`: 1.23-1.60 s; `raw/times.jsonl`).  The table build is single-threaded.
Per probe (`2|B|` per target) the table costs about 43 ns at `l = 11` (derived: 86.5 µs / 2,014 probes) and 62 ns per key to build.

## 3. Reading the numbers (interpretation, labelled; scope: this host class, this stage)

* **Observation.**  On the PS-0 corpus the table returns the same verdicts and root sets as the kernels, probes a refuted target in 1/94…1/361 of the scalar group kernel's time (`l = 8…13`), and its build costs between 3 and 8 targets' worth of that kernel's time.
* **Reading for the speedup program (assessment, not a measurement of anything end to end).**  The group-law kernels of PS-2/PS-3 recompute, for every target, quantities the table stores once: its keys `x(P_i ± P_j)` are the same additions with the target-independent operand first.
  Where the table fits in memory it replaces them at two orders of magnitude lower per-target cost at these rungs, so a stage speedup of the pairs-and-solve loop is **not** a speedup over the repository's incumbent for `m = 3`; the regime where the kernels are the only option is the one where the table does not fit.
* **Memory is the constraint (arithmetic from the measured constants, not a measurement).**  Keys grow as `|B|² ≈ 2^{2l−2}` and the table needs about 24.5 bytes per key, so `l = 14, 15, 16` would need about 1.5, 6.0, 24.0 GiB.  This host has 16 GiB.
* **Not claimed.**  Nothing about relation yield, the number of relations needed, linear algebra, or the charge in the counted unit `S` (PS-6); nothing about ECC2K-130 (`l ≈ 44` would need `~2^{86}` keys); the table here is the harness's own (`mitm.rs`), not `ic_boundary::PairTable`, and the two were not compared with each other.

## 4. Not done

* One host class only (host B); no second host.  No multi-threaded build or probe, no prefetching, no vector build of the table (leads L2, L9 of [`../LEADS.md`](../LEADS.md)).
* The repository's own `Oracle::Mitm` was not run on this corpus; PS-6 prices both in the counted unit.
* Wall figures are exploratory in the repository's isolation vocabulary (shared container with the benchmark lock, not the receipt service).
