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

## Results (all 8 rows, every one exhaustively re-verified, `verify = true`)

| arm | t | N | D (both targets) | width | per-target wall |
|:--|--:|--:|:--:|--:|--:|
| ss-j1728 | 11 | 22 | **6, 6** | 63,073 | 470 s, 570 s |
| ordinary | 11 | 22 | **6, 6** | 63,078 | 960 s, 646 s |
| ss-j1728 | 12 | 24 | **6, 6** | 110,862 | 1,878 s, 2,495 s |
| ordinary | 12 | 24 | **6, 6** | 110,862 | 3,135 s, 2,049 s |

The ordinary arm's seeded search initially landed on a supersingular curve
(`#E' = p + 1`, the study's own genuineness guard, probability ≈ 1/√p per
accepted curve): `build_arm` now walks the seed until the searched `E'` is
ordinary (fixed in `examples/pkm_ss_tower.rs`, this task).

## Reading

**The D-equality of PR #1947 Q2 extends two cells beyond the measured
`N ≤ 20`: supersingular `D` = ordinary `D` = 6 at `N = 22` and `N = 24`**,
with the same widths (63,073/63,078 and 110,862 columns) and the
supersingular arm cheaper per target (no search; its aux share is the
build only).  This matches the derived argument (same leading forms and
supports) and the re-measured Kummer ladder, which gives 6 at
`t = 10 … 13` (`research/pkm_kummer_remeasure_t1_14_20261101/`).

## Reproduce

```bash
cargo run --release --example pkm_ss_tower -- f4 --p 14680063 \
  --arm ordinary  --m 2 --t-min 11 --t-max 12 --targets 2 \
  --budget 259200 --out ordinary_m2_t11_12.jsonl
cargo run --release --example pkm_ss_tower -- f4 --p 14680063 \
  --arm ss-j1728 --m 2 --t-min 11 --t-max 12 --targets 2 \
  --budget 259200 --out ss-j1728_m2_t11_12.jsonl
```
