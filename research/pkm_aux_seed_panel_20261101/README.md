# Auxiliary-curve search: seed panels and the X₁(2, 8) family

2026-11-01 · items 5 and 6 of the auxiliary-curve open items (Conductor
T-5750) · instrument `examples/pkm_aux_seed_panel.rs` (new; no pipeline,
no `S`) · records `panel_any.jsonl`, `panel_z2xz8.jsonl` · log `run.log`

**Class: stage diagnostic of the solver axis (AGENTS.md §2).** Two
questions from `research/pkm_isogeny_tower_ecbench_20261015/` panel 3 and
its "what is not done" list: how big is the seed spread of the ordinary
auxiliary-curve search at fixed `t` (P3 held only for 16-seed means), and
what does drawing from the genus-0 family `X₁(2, 8)` (the `z2xz8` family of
`two_sylow_search`, "would cut the curve count by a constant; not tried")
actually buy? The family is new in the `prime-isogeny` plug-in
(`family=z2xz8`, this task).

## Setup

`p = 14,680,063 = 7·2^21 − 1` (the supersingular study's prime), `t ∈
{8, 10, 12, 14}`, seeds 0 … 15 per `(family, t)`, 64 rows per family, 128
builds total. Same charged search as the plug-in (residue-class BSGS
filter, Mestre order, order-aware `Q` construction), families `any`
(every curve) and `z2xz8` (every draw's 2-Sylow exponent ≥ 2³; needs
`p ≡ 3 mod 4`). Deterministic in `(p, t, seed, family)`.

## Results

| family | t | mean tried | min | max | tried/2^t | mean wall |
|--:|--:|--:|--:|--:|--:|--:|
| any | 8 | 221.4 | 23 | 629 | 0.865 | 5 ms |
| any | 10 | 433.5 | 38 | 969 | 0.423 | 20 ms |
| any | 12 | 5,075.3 | 90 | 12,769 | 1.239 | 105 ms |
| any | 14 | 9,464.5 | 546 | 30,825 | 0.578 | 288 ms |
| z2xz8 | 8 | 45.8 | 1 | 179 | 0.179 | 0 ms |
| z2xz8 | 10 | 254.6 | 7 | 944 | 0.249 | 1 ms |
| z2xz8 | 12 | 463.8 | 20 | 1,426 | 0.113 | 36 ms |
| z2xz8 | 14 | 715.9 | 63 | 3,039 | 0.044 | 50 ms |

## Reading

- **Seed spread (item 5)**: reproduced. At fixed `t` the tried-curve count
  spans two to three orders of magnitude (t = 12: 90 … 12,769). The
  16-seed mean of `tried/2^t` is 0.42 … 1.24 for `any`, consistent with the
  ecbench panel-3 pooled `C = 1.05` within noise; P3's "held for the mean,
  failed on single draws" is the right reading, and no single-seed number
  should be quoted as the search cost.
- **The X₁(2, 8) family (item 6)**: mean tried drops 4.8× (t = 8), 1.7×
  (t = 10), 11× (t = 12), 13× (t = 14) against `any`; wall drops ~6× at t = 14
  (288 ms → 50 ms). The Haar model predicts the asymptotic gain 2³ = 8×;
  the t = 12/14 means exceed it (the family also avoids filter false
  positives and singular draws). The recorded caveat stands: at primes
  with `v₂(p+1)` large the supersingular auxiliary curve still dominates
  (no search at all); `z2xz8` is the generic-prime improvement.
- Accounting unchanged: the family draws are charged exactly like the
  `any` draws (same filter, Mestre and `Q` machinery; counters
  `aux_curves_tried` … `aux_tower_build_gae` as before), and rows carry
  the new marker `aux_family_z2xz8` when the plug-in uses the family.

## Reproduce

```bash
cargo run --release --example pkm_aux_seed_panel -- --p 14680063 \
  --t 8,10,12,14 --seeds 16 --family any  --out panel_any.jsonl
cargo run --release --example pkm_aux_seed_panel -- --p 14680063 \
  --t 8,10,12,14 --seeds 16 --family z2xz8 --out panel_z2xz8.jsonl
```
