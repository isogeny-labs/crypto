# Supersingular D-equality beyond N = 20: m = 2, t = 11 … 12

2026-11-01 · item 3 of the auxiliary-curve open items (Conductor T-5750) ·
instrument `examples/pkm_ss_tower.rs` (`f4` subcommand, engine
`f4_fp_tower`) · records `ordinary_m2_t11_12.jsonl`,
`ss-j1728_m2_t11_12.jsonl` · script `run_cells.zsh` · log `run.log`

**Class: stage diagnostic of the solver axis (AGENTS.md §2).** Extends the
supersingular-auxiliary study `research/pkm_supersingular_aux_20261015/`
(PR #1947, Q2: "the supersingular tower gives the ordinary tower's `D` in
all 14 (m, N) cells, N ≤ 20") by two cells at `m = 2`: `t = 11` (`N = 22`)
and `t = 12` (`N = 24`), same prime `p = 14,680,063 = 7·2^21 − 1`, two targets
per cell, default seeds, every finished row exhaustively re-verified
(`verify = true`).

## Status: running (rows land incrementally)

- `ss-j1728, t = 11 (N = 22)`: **D = 6 on both targets** (width 63,073
  columns, ≈470 s and ≈570 s per target). The ordinary arm initially
  panicked: its seeded search landed on a supersingular curve
  (`#E' = p + 1`, the study's own genuineness guard, probability ≈ 1/√p
  per accepted curve) — `build_arm` now walks the seed until the searched
  `E'` is ordinary (fixed in `examples/pkm_ss_tower.rs`, this task) and the
  ordinary arm was relaunched.
- `t = 12` cells for both arms and the ordinary `t = 11` cell: in flight.

## Reading (so far)

At `N = 22` the supersingular tower's `D = 6` matches the ordinary tower's
measured staircase at `N = 20`/`N = 22` (the re-measured Kummer ladder
`research/pkm_kummer_remeasure_t1_14_20261101/` gives 6 at `t = 10 … 13`),
i.e. the D-equality claim of PR #1947 Q2 extends one cell further, as its
derived argument (same leading forms and supports) predicts. The
comparison to close is ss-j1728 against the *ordinary* arm at the same
`(m, t)` once that arm lands.

## Reproduce

```bash
cargo run --release --example pkm_ss_tower -- f4 --p 14680063 \
  --arm ordinary  --m 2 --t-min 11 --t-max 12 --targets 2 \
  --budget 259200 --out ordinary_m2_t11_12.jsonl
cargo run --release --example pkm_ss_tower -- f4 --p 14680063 \
  --arm ss-j1728 --m 2 --t-min 11 --t-max 12 --targets 2 \
  --budget 259200 --out ss-j1728_m2_t11_12.jsonl
```
