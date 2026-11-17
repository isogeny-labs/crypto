# Frozen seeded-walk protocol v1 - 2026-10-27

Purpose: discriminate a seed-specific trajectory deficit from persistent DP
selection effects. This protocol is not a performance result. BEST2 remains 0
by default. A namespace is the resampling unit; repetitions are timing controls.

## Exact-curve CPU diagnostic (pending EKS access)

Build the native `gpu/nist/test_best2_b0`, `test_best2_b1` and `best2_stats`
from the same pinned commit. OpenSSL development headers and libcrypto are
required. Run both arms for each W=16,32,64 and dp_bits=8,12 with seed IDs
20261027..20261058 inclusive, 65536 iterations per lane, on an authorized EKS
worker. For each cell do two distinct panels: fixed table_seed=7/varying starts,
and the default independently derived table/start namespaces. Do not pool them.
The target is Q=[7]G on each named OpenSSL NIST curve, full-size field arithmetic.
The script is a bounded DP diagnostic, not a practical NIST DLP solver.

Example inner argv (each arm; shell only redirects the raw records):

```
gpu/nist/test_best2_b1 --sample 20261027 32 64 65536 12 7
gpu/nist/test_best2_b0 --sample 20261027 32 64 65536 12 7
```

Preserve both JSONL streams and feed their concatenation to `best2_stats`.
It refuses duplicate namespaces, missing arms, mismatched work and accounting
violations. It reports emitted DP / (physical attempts * 2^-d) and emitted DP /
(eligible advances * 2^-d) separately. The latter expectation is a null hypothesis,
not a proved DP law. It bootstraps 10000 paired whole-seed clusters, RNG seed
20261027, 95% percentile intervals; at least eight clusters are required.
Intervals are exploratory and unadjusted across curves, W, DP bits and panels.
Do not interpret a zero-DP bootstrap failure as a zero rate or discard that seed.
Report every seed and the failure; obtain more events in a separately frozen panel.
Score-bucket [candidate count, DP count] and plus/minus/selected DP counts expose
marginals and selection enrichment. Steps from one walk are dependent.

Resource contract: four CPUs, 16 GiB, no GPU; 1800 seconds per cell, no automatic
retries. Preserve partial/failing records. No wall-time comparison is admitted
without L2 and A/A; diagnostic counts do not claim CPU speed. Check existing job
IDs before submitting. A stopped cell is not resumed as an independent seed.

## GPU seed panel (pending authenticated GPU access)

On the same authorized GPU and pinned toolchain, run:

```
gpu/nist/best2_ab.sh 2048 3 32 20261027 7
gpu/nist/best2_ab.sh 2048 3 32 20261027
```

These are separate fixed-table and varied-table panels, 3 timing repetitions
per seed, interleaved BEST2 order, W=16/32/64, both curves. Capture the full logs,
source commit, exact compiler flags/version, binary SHA-256, GPU UUID/driver,
clocks, temperatures and power state. The `SEED` marker states the actual table
and start namespace. Same seed repetitions MUST reproduce DP counts. Only seeds,
not repetitions or W cells, count as new statistical samples. This benchmark
excludes warmup DPs after the correction but retains four warmup trajectory steps.
Its throughput numerator is physical lane-iterations, not eligible DP tests.
Do not feed this multi-seed panel to the historical `gpuab` parser as though the
repetitions were independent. A seed-aware throughput/DP GPU-log validator and
DP-record transfer/replay path are still required before a claim.

No GPU wall limit or new spending authorization is invented here; launch only
on available authorized infrastructure within its existing budget. Preserve
failed cells and report missing telemetry. Instrumented kernel timings must be
reported separately if extra atomics or replay transfers change throughput.

## ecbench comparison (implemented model, unexecuted session)

Use `research/nist_best2_small_batch_20261027/ecbench-model.json` on EKS with a binary built from the pinned commit.
It has 16 independent one-target workloads on the existing 20-bit prime-search
constructor, five rounds, a warmup, paired method seeds and A/A. Run the native
session and `ecbench verify --replay 12` on another host class; preserve both
audits/receipts and hashes. Run `ecbench plan` first and require the existing
curve identity to be registered. Report GAE, setup/search, all unpriced counters
and the 0/0.1/1 GAE sensitivity band. Count both candidate additions; do not
pretend this generic implementation shares a field inversion.

The two new model IDs have explicit jumps=256, walks=64, dp_bits=8 and physical
step budget=10000*sqrt(r). The strong `rho.negation` reference remains unchanged.
The model uses domain-separated hashes of group keys, full points for cycle
checks, and direct generic group operations. It is NOT the coordinate-limb CUDA
walk and cannot validate a P-256/P-384 GPU cost estimate. Do not pool this model
with the old standalone model or GPU throughput to claim a measured solve cost.

## Actual successful-collision cost and decision gate (still missing)

A stage throughput run does not recover a scalar. `RhoHost` is restricted to
orders below 2^63 and cannot replay full-size NIST scalars. Before a GPU solve
claim, implement a bounded, explicitly identified known-answer collision workload,
a compatible GPU-to-host DP trace, independent coefficient recovery/verification,
and complete interval accounting (setup, starts/restarts, physical steps,
DP transfer/table processing, fruitless collisions, recovery and verification).
Small-order fixtures verify plumbing only; they are not full NIST solve timings.
Record time and every failed/censored attempt per successful verified collision.
A full-size NIST rho solve is not a feasible small validation workload.

Enablement requires reproducible matched evidence, independent replay and
number recomputation, correctly admitted uncertainty, no unexplained material
DP deficit, and a lower complete effective cost at the best configuration.
Until those exist, leave BEST2=0, do not add a benchmark/leaderboard row, and
report the bias-vs-sampling question as unresolved. If only fixed-table seed
variation removes the deficit, report sampling evidence limited to that panel.
If a deficit persists among eligible advances, test alternative score/DP hashes
under a separately committed amendment; never pick a favourable seed or mask.
