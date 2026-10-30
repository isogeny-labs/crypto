# XP-20261015-shape-g1-frozen-cells: protocol

**Status: frozen** by the commit that adds this file, after the sizing probes
of §6 and before any measured draw at the card's seed. Card:
[`card.json`](card.json); narrative [`README.md`](README.md). The study
whose rules this inherits:
[`research/shape_search_g1_g6_20261015/PROTOCOL.md`](../../research/shape_search_g1_g6_20261015/PROTOCOL.md)
(§3 "G1", §4 "Decision rules"), frozen in `3bdf9b06e`. Every statement is
*recorded* (a committed row or `.meta`), *derived* (a calculation shown here)
or *predicted* (stated before the runs).

## 1. Question

On class-0 targets at `m = 3`, do the `gp` and `fp` factor-base shapes have a
mean refutation degree at least 0.5 below random at two of the frozen cells
`(n, l) = (13, 5), (17, 6), (19, 7)`?

## 2. Instrument, fixed

| | value |
|:--|:--|
| binary | `examples/factor_base_shape_ladder` (`g1`, `score`, `selftest`), release build at the commit that adds this file (PR #1892 is merged: `3c0646ba9`) |
| shapes | `random, random2, gp, fp, coord` (the card's list); `random2` is the negative control |
| draws | `--instances 2 --targets 2` (the card), seed `20261015`; `coord` has one instance and `2 × 2` targets, as the binary fixes |
| environment | the card's `env`: one thread, `F4_F2_MAX_ROWS/COLS 50000000`, `KIC_SPARSE_DENSE_FINISH 1`, `KIC_SPARSE_F5 1`, `KIC_SPARSE_DENSE_BUDGET_MB 1200` |
| execution (disclosed) | one process per `(cell, shape)` instead of one per cell: the binary seeds each `(n, l, shape, instance)` independently (`seed_of(seed, [n, l, shape_id, k])`), so the per-shape output equals the lines of a single-process run (the study's `crosscheck.sh` relies on the same fact). Each process runs under `prlimit --as` and `timeout --signal=KILL`; its `.meta` records exit, wall and sampled peak RSS |
| commands | [`research/shape_search_g1_frozen_cells_20261018/run.sh`](../../research/shape_search_g1_frozen_cells_20261018/run.sh) |
| output | `runs/g1-<n>-<l>-d<d_max>-<shape>.jsonl` plus `.err` and `.meta`; scored by `factor_base_shape_ladder score runs/*.jsonl` |

## 3. Stages and sizing (from the probes of §6)

| stage | cell, `d_max` | processes | per-process caps | role |
|:--|:--|:--|:--|:--|
| A | `(13, 5)`, 6 | 5 (one per shape), in parallel | address space 16 GB, wall 24 h | the one-sided `(13, 5)` arm the study's §3 defined, run on every shape. *Derived from the probe:* one random draw at `d_max = 6` takes 5,732 s at 2.1 GB, so a 4-draw process takes about 6.4 h |
| B | `(13, 5)`, 7 | 5, at most 2 at a time | address space 60 GB, wall 14 days (the card's `timeout_per_shard_s`) | the card's shard 0. *Recorded:* one random draw exceeds 4 h on one thread (killed at the cap, peak 7.7 GB sampled); not completed in any probe before the freeze. Sized by the card's 47–50 GB estimate; the host has 124 GB |
| C | `(17, 6)` and `(19, 7)`, 6 then 7 | as B | address space 24 GB at `d_max = 6` (probes), 60 GB at 7, wall 14 days | the card's shards 1 and 2. Run only if their `d_max = 6` probes finish within their 4 h cap; otherwise recorded as *not sized on this host* and not run |

Stages run in the order A, B, C. Stage A's result is reported whatever
happens to B and C. A process the kernel or `prlimit` ends is recorded with
its `.meta` and its draws count as `caps_hit`; it is not re-run.

## 4. Decision rules, fixed (the study's §4, applied per cell and `d_max`)

- Statistic: class-0 mean refutation degree per arm; a censored draw
  (`at_least d`) enters at `d`; `caps_hit` draws are excluded and counted.
- Per shape `X ∈ {gp, fp}`: `gap = mean(random) − mean(X)`; *established*
  if `gap ≥ 0.5` with no censored `X` draw; *excluded* if both arms fully
  resolved and `gap < 0.5`; *excluded by censoring* if `X` is censored and
  the lower-bound gap is `< 0.5` with random resolved; else *undetermined*.
- Verdict: *survives* if a shape has the gap established at two or more of
  the three frozen cells; *killed on the frozen cells* if no frozen cell has
  it established and every cell was measured; *not testable (resource)* for
  a cell that was not run or whose arms are all `caps_hit`.
- Negative control: if `|mean(random2) − mean(random)| ≥ 0.5` at a cell, a
  gap there is not counted as established.
- Stage A is one-sided: at `d_max = 6` a random draw censored at `≥ 7`
  (*predicted*, from Results 7–9 of `RESEARCH_DREG_MEASUREMENT.md` and the
  probe) means a `gp` or `fp` draw resolving at `≤ 6` shows a gap `≥ 1` on
  that draw, and a `gp`/`fp` draw at `≥ 7` leaves the cell undetermined at
  this `d_max`, because both arms are censored. Stage B lifts the censoring.

**Inadmissible:** changing shapes, instances, targets, seed, `d_max` or caps
after a stage starts; dropping an arm; re-running a draw.

## 5. Predictions (before the runs)

1. Stage A: every arm censored at `≥ 7` (gap 0.000 at the lower bound), as on
   the scaled cells; `(13, 5)` undetermined at `d_max = 6`.
2. Stage B: every arm resolves at 7, gap 0.000, G1 excluded at `(13, 5)`.
3. Stage C: not sized on this host (prediction: the `d_max = 6` probes of
   `(17, 6)` and `(19, 7)` do not finish within 4 h).

## 6. Sizing probes (disclosed; seed 1, not the measured seed; not results)

Recorded in `research/shape_search_g1_frozen_cells_20261018/probe/` (`progress.txt`, `*.meta`).

| probe | result |
|:--|:--|
| `(13, 5)`, `d_max = 7`, random, 1 × 1, one thread, 64 GB cap | killed at the 4 h wall cap, no draw finished, sampled peak RSS 7.7 GB |
| `(13, 5)`, `d_max = 7`, same, `RAYON_NUM_THREADS = 8` | runs at one core: the binary does not parallelise a draw (still running at the freeze, 12 h cap) |
| `(13, 5)`, `d_max = 6`, random, 1 × 1, one thread | 5,732 s, peak 2.1 GB, outcome `at_least 7` |
| `(17, 6)`, `(19, 7)` at `d_max = 6` and 7, one thread, 24 GB cap | running at the freeze (4 h and 12 h caps); §3 stage C depends on them |
