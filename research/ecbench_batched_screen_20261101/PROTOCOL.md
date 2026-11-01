# `batch_trials` development screen on `ic.koblitz_batched`

Status: **frozen before data.** Local, diagnostic, counted-unit development
screen. Not a registered tournament round, not isolated, not promotable.

## Question

Round 0026 (`research/ic_candidate_tournament_20260915/runs/round-0026/`)
measured the native tournament worker in Valgrind instruction reads and
found `batch16` at 0.630× the incumbent `batch64`. `ic.koblitz_batched`
(PR #3134) is that worker's pipeline in `ecbench`'s counted unit, pinned to
the worker's own counts on four round-0026 cells. Here, on the same four
development cells, how does the **counted** cost move with `batch_trials`,
and where is its minimum?

The counted unit is group-addition equivalents (GAE): the point additions
and doublings each stage performs. It prices neither field arithmetic,
canonicalisations, pair-table probes nor linear-algebra row operations, so a
GAE ratio is not an instruction or time ratio and is not compared with
round 0026's `Ir` ratio.

## Instrument

`ic.koblitz_batched` at source commit `729847dcea` (branch
`claude/ecbench-batch-trials`, PR #3134), built with `cargo build --release
--bin ecbench`. The session records the binary and host. Parameters other than
`factor_base_points` and `batch_trials` are the worker's and the tournament's:
`fb_seed` 43, `max_trials` 4096, `summands` 3, `seed` = the run's algorithm seed.

## Panel

Cells (round 0026's development cells), one `ecbench` session each, with the
tournament's `6n`-point subgroup-orbit base:

| cell | curve | `factor_base_points` |
|:--|:--|--:|
| `n13a0` | Koblitz a=0 n=13 | 78 |
| `n17a1` | Koblitz a=1 n=17 | 102 |
| `n19a0` | Koblitz a=0 n=19 | 114 |
| `n23a0` | Koblitz a=0 n=23 | 138 |

Arms, differing only in `batch_trials`: `ic-b1`, `ic-b2`, `ic-b4`, `ic-b8`,
`ic-b16`, `ic-b32`, `ic-b64` (**baseline**: the tournament incumbent),
`ic-b128`, `ic-b256`, and `ic-b64-aa` (**control**: the baseline's
duplicate). `batch16` is round 0026's selection; 1, 2, 4, 8 and 256 are the
neighbours the tournament's own refinement rule (`batch/2`, `batch/4`, `2·batch`,
`4·batch`, 1, 2, 8, 256) would try next; 32 and 128 are round 0026's other
`batch_trials` candidates. Round 0026's `dense`, `excess8` and `window8` are not
`batch_trials` changes and are not ported, so they are not in this screen.

Targets: 32 public points per curve, `target_seed` `202611011001` (n13),
`…1002` (n17), `…1003` (n19), `…1004` (n23). Measurement: 3 rounds after 1
warmup, order `alternate`, seed `202611012001`, isolation `L0`. All arms see the
same targets, so every comparison is paired by target.

## Predictions (stated before data)

On these cells every query yields a relation and the factor base has 7–8
projected columns (round 0026's worker draws exactly 64 and 16 queries at
`batch_trials` 64 and 16, and 8 at 1–4), so:

- **P1.** The queries drawn are `ceil(columns / b) · b` for `b ≤ 8` (8) and
  `b` for `b ≥ 16`; for `b ≥ 16` the cost therefore rises with `b` through
  the relation phase alone.
- **P2.** Counted GAE rises monotonically from `ic-b16` to `ic-b256` on every
  cell, and `ic-b128` and `ic-b256` are the two most expensive arms on every cell.
- **P3.** `ic-b1`, `ic-b2`, `ic-b4` and `ic-b8` draw the same queries; any
  difference among them is per-batch overhead (the collector's per-unit set-up),
  so GAE is non-increasing in `b` from 1 to 8.
- **P4.** The minimum over the nine arms lies in `{4, 8, 16}` on every cell.
- **P5.** `ic-b16 / ic-b64 < 1` on every cell.

Any of these may be wrong; the report states which hold.

## Reporting

For every cell and arm: the paired ratio of mean total counted GAE to
`ic-b64`, with its cluster-bootstrap 95% interval (`ecbench compare --save`),
and the per-phase table (`ecbench table`). The A/A control must read 1.000.
Verification: `ecbench verify --replay-all` on every session (every record's
counts, unpriced work and factor base reproduced bit for bit). An arm whose
interval lies wholly below 1 is a counted improvement over the incumbent
on that cell; wholly above 1, a regression; otherwise unresolved. No
cross-cell pooling, no rho comparison, no scaling claim.

## What this cannot show

- Wall time: `L0` (no isolation); wall figures are descriptive only. The open
  round-0026 question (wall time under core reservation) needs an isolated
  run.
- Instruction reads, field arithmetic or any unpriced native work.
- Anything about larger curves, other factor bases, `dense`, `excess8`,
  `window8`, or rho.
- Promotion: a development screen is never promotable; it informs which
  arms a registered, isolated round should contain.

## Stop conditions

A session that does not verify, a failed A/A control (ratio outside
[0.999, 1.001] in counted GAE), or any record that is not `VERIFIED` stops
the screen and is reported as such. The seeds are not reused after a measured
failure.
