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

## Addendum: the wide backend, and the 61-bit data point (same day)

`pkm_aux_seed_panel --wide` runs the same panel on the `FpWide<1>`
backend through `prime_tower_wide::isogeny_tower_wide` — the multi-limb
construction behind the P-192 runs
(`research/pkm_isogeny_wide_p192_20261101/`).  Two panels, 16 seeds per
`(t)`, family `any`:

**Cross-check at the same prime `p = 14,680,063`** (wide1 against the u64
plug-in above; the rng streams differ, so this is a statistical, not
per-seed, comparison):

| backend | t | mean tried | tried/2^t | GAE/curve |
|:--|--:|--:|--:|--:|
| u64 | 8, 10, 12 | 221, 434, 5,075 | 0.87, 0.42, 1.24 | 50, 40, 35 |
| wide1 | 8, 10, 12 | 188, 542, 3,463 | 0.74, 0.53, 0.85 | 52, 42, 37 |

Indistinguishable at this seed count (the spread is two to three orders
of magnitude): the wide search is the same instrument.

**The top of the u64 range, `p = 2^61 − 1`** (`panel_wide1_p61.jsonl`):

| t | mean tried | tried/2^t | GAE/curve | mean wall |
|--:|--:|--:|--:|--:|
| 8 | 156.0 | 0.609 | 11,030 | 2.4 s |
| 10 | 898.1 | 0.877 | 5,231 | 6.4 s |
| 12 | 1,859.8 | 0.454 | 2,624 | 8.8 s |

Reading: the per-curve cost has risen from ≈ 35–50 GAE at 14.7M to ≈
2.6–11k GAE at 2^61, consistent with the `Θ(p^{1/4})` Mestre/BSGS cost
(`2^{15}` at 61 bits, with the table split shifting in `t`).  A whole
build stays practical (≤ 9 s).  Extrapolated to P-192, `2^{45}`–`2^{48}`
GAE per accepted curve — the recorded caveat of
`prime_tower_wide`: at NIST size the ordinary search is an instrument
for small primes; the supersingular auxiliary curve (no search) is the
route, as `research/pkm_supersingular_aux_20261015/` concluded.
