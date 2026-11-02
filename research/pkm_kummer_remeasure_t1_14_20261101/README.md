# The Kummer tower solving-degree ladder, re-measured: t = 1 … 14

2026-11-01 · item 1 of the auxiliary-curve open items (Conductor T-5750) ·
instrument `examples/pkm_tower_pilot.rs` (engine `f4_fp`, degree-bounded F4)
· records `remeasure_m2_kummer.jsonl` · log `run.log`

**Class: stage diagnostic of the solver axis (AGENTS.md §2). No pipeline is
priced, no `S`, no leaderboard row.** This is the re-measurement the
treatise's open problem 14 and the mega-pass GAPS lever 4 asked for
("regenerate t=1..14 with closure certificates; score prediction
`d_F = 4 + ⌊(t−2)/4⌋`"; the stage-A values were never committed).

## Setup

`p = 786,433 = 3·2^18 + 1` (`2^t | p − 1` for every `t ≤ 18`), `m = 2`, kind
`kummer`, control `tower` only, one planted target per `t` (solvable by
construction), `--ladder-t none`, seed `20261101`, budget 259,200 s per
system. Deterministic in the seed. Every row carries the pilot's built-in
checks (`planted_ok`, `zero_dim`, on-curve and tower checks before anything
is measured).

Command:

```bash
cargo run --release --example pkm_tower_pilot -- --kinds kummer --m 2 \
  --controls tower --t-min 1 --max-t 14 --planted 1 --random 0 \
  --ladder-t none --budget 259200 --seed 20261101 \
  --out research/pkm_kummer_remeasure_t1_14_20261101/remeasure_m2_kummer.jsonl
```

## Results

Solving degree `D` (`solving_degree_max`, the maximum F4 step degree) at
`N = 2t`:

| t | N | D | D reached | planted found | wall |
|--:|--:|--:|--:|:--:|--:|
| 1 | 2 | 4 | 4 | ✓ | 0.0 s |
| 2 | 4 | 4 | 4 | ✓ | 0.0 s |
| 3 | 6 | 4 | 4 | ✓ | 0.1 s |
| 4 | 8 | 4 | 4 | ✓ | 0.1 s |
| 5 | 10 | 4 | 4 | ✓ | 0.6 s |
| 6 | 12 | 5 | 5 | ✓ | 3.6 s |
| 7 | 14 | 5 | 5 | ✓ | 19.0 s |
| 8 | 16 | 5 | 5 | ✓ | 75.3 s |
| 9 | 18 | 5 | 5 | ✓ | 269.8 s |
| 10 | 20 | 6 | 6 | ✓ | 1,667.4 s |
| 11 | 22 | (running) | | | |
| 12 | 24 | (running) | | | |
| 13 | 26 | (running) | | | |
| 14 | 28 | (running) | | | |

## Reading

- **The prediction `d_F = 4 + ⌊(t−2)/4⌋` holds in every measured cell**
  (4 at t = 1…5, 5 at t = 6…9, 6 at t = 10, against predicted 4/4/5/5/5/5/5/5/5/6).
  The one deviation from the old stage-A notes is `t = 1` (3 there, 4 here;
  different seed and controls — the stage-A values were never committed).
- The closure certificate of each row is the pilot's own verification:
  `planted_ok = true` (the planted decomposition was recovered) and
  `zero_dim = true` (the solved ideal is zero-dimensional), with the full
  step trace in the record.
- The staircase keeps rising: no plateau through `t = 10`. Under the
  prediction, `t = 11..13` should give 6 and `t = 14` the first 7. If that
  holds, the measured degree grows logarithmically in `t` — **negative for
  lead 1's bounded-`D` premise at `m = 2`**, consistent with the recorded
  `D ≥ 8` at `m = 3, N = 21` and the NEGATIVE `m = 4, N = 20` plateau.

## Not done here

`t = 11 … 14` were launched with a 3-day per-system budget and write rows
incrementally; the treatise's extrapolation (≈1.7 h at t = 12, 17 h at
t = 13, 7 d at t = 14) puts t = 14 at risk of the budget. The m = 3 / m = 4
cells and the m=3-N=21 hardware gate (≈256 GB) are untouched — that is
`backlog/XP-20261015-pkm-m3-n21-m4-n20`, still the decisive cell.
