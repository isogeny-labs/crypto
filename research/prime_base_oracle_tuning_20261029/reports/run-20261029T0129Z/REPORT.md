# Run report: XP-20261016 prime-base oracle tuning, run.sh all (2026-10-29T01:29Z)

This report records observations only. The decision is in the study's [`README.md`](../../README.md).

## What ran

- **Command:** `sh backlog/XP-20261016-prime-base-oracle-tuning/run.sh all research/prime_base_oracle_tuning_20261029` with `RAYON_NUM_THREADS=1` and `PBO_CPU=3` (`taskset -c 3`).
- **Launch:** frozen task card `CARD-XP-20261016-prime-base-oracle-tuning-01` (experiment director → mechanical executor).
- **Timing:** started 01:29:21Z, analysis written 01:56:09Z.
- **Commit and binary:** worktree commit `7df34ba67dc6`; binary source `0e124fb1b207`; ecbench SHA-256 `bf5eec6d…3807d9`.
- **Host:** 4-vCPU Intel Xeon @ 2.10 GHz cloud container, env class `ECBENV3hc4d998f5ae3f` (`host/host.json`). Isolation: one pinned core, shared host. No dedicated-host claim.
- **Stages, in order:**
  - host
  - identity: replay of frontier P20–P32
  - train: 4 sessions + 6 edge-extension sessions at 20 bits
  - select
  - holdout
  - kummer
  - audit
  - analyse
- **Verify:** `run.sh verify`, run afterwards, wrote `verify/`.

## Runs requested and completed

| stage | sessions | measured records | verified |
|:--|--:|--:|--:|
| identity (replays of frontier P20–P32) | 4 | 240 each | 240 each; replays reproduced |
| train P20 / P24 / P28 / P32 | 4 | 232 / 232 / 208 / 208 | all |
| train edge extension (20 bits; mitm3, rfmitm3, rfsub3 at N 91, 128) | 6 | 16 each | all |
| holdout P20 / P24 / P28 / P32 | 4 | 1,008 each (7 arms × 48 targets × 2 rounds, plus warm-ups) | all |
| kummer P22 / P24 | 2 | 160 each | all |

No run was exhausted, timed out, errored or ran out of memory. Every audit printed `ok` with 0 problems (`audits/`, `pbo-run.log` lines).

## Measured values

All values are *recorded*. They are in the unit S = counted GAE/√r, priced at w = 0, 0.1 and 1 GAE per native operation.

- Holdout means per arm: `analysis/band-holdout.md` and `analysis/table-holdout.md`. The latter also gives 95% intervals of the mean, and median wall time per run, which is not a claim.
- Paired ratios: `analysis/compare-*.json`, 96 pairs each.
- Train cells: `select/band-train-P*.md`. Selection: `select/selection-P*.json`.

Selected cells (train rule: minimum mean S at w = 0.1):

| size | selected | incumbent |
|--:|:--|:--|
| 20 | `mitm2-n45` | `mitm2-n32` |
| 24 | `mitm2-n115` | `mitm2-n162` |
| 28 | `mitm2-n288` | `mitm2-n407` |
| 32 | `mitm2-n1024` | `mitm2-n1024` |

![holdout ratio](../../figures/holdout_ratio.svg)

Figure 1. Holdout `Σ S_sel / Σ S_inc` per size at w = 0, 0.1 and 1, with cluster-bootstrap 95% intervals. There are 96 pairs per point. Sessions: `ECBS1h685271327a8a`, `ECBS1ha436e6362605`, `ECBS1h3c10dafc408f` and `ECBS1h1901ad2eb0e5`.

![train sweep](../../figures/train_n_sweep.svg)

Figure 2. Train mean S against N for the incumbent oracle, 8 runs per cell. Sessions: `ECBS1h9fcb65b4770e`, `ECBS1hc463af97cc77`, `ECBS1had86361014ec` and `ECBS1h5cd4a4b40770`.

## Controls

- **A/A arms:** `inc-aa` and `rho-neg-aa` give exactly 1.000 at every weight and size.
- **Matched rho:** `rho-neg`, in the same sessions.
- **Stride control:** `prime-abscissa:size=N*,stride=7919`.
- **Panel K:** `kummer` vs `lowx`, descriptive only.
- **Random-base control:** absent (PROTOCOL.md §12.2).

## Raw records

`sessions/*/` (records.jsonl, session.json and receipts) are kept in the artifact store, with `.artifacts.json` pointers committed. Audit receipts are in `audits/`, and the replay receipts from verify are in `verify/`.
