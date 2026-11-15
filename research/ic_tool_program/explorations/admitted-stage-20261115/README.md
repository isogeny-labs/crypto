# The aimed scan's admitted stage: lagged run prefetch and a doubled filter (explorations, both dropped)

- **Status:** explorations on this session's container, 2026-11-15, three
  rounds an arm, on top of the block-filled filter (`d9d14dbbd7` plus the
  candidate change). These are records, not rounds, and support no claim.
  Both changes are reverted from code.

## What the stage costs

Raw counts from the block-fill A/B's runs:
- **Admitted rate:** 2.8% of summands at k0n53 and 2.0% at k0n61.
- **Nearly all are false positives:** pairs per admitted key are 0.07% and
  0.02%.
- **Cost:** each admitted key costs 71–75 ns, or 1.5–2.0 ns a summand.

## 1. Run prefetch issued 8 admissions late, inside the filter loop (`KIC_SCAN_RUNLAG`)

- **The change:** only prefetch timing moves; lookups and counts are
  unchanged.
- **Admitted stage:** 2.07, 1.90, 2.00 → 1.80, 1.84, 1.85 ns a summand
  (k0n53), and 1.44, 1.50, 1.45 → 1.33, 1.32, 1.28 (k0n61).
- **Collect:** within noise.
- **Reading:** about 0.15 ns a summand, about 1% of collection, below what a
  same-session A/B on this host resolves.

## 2. Doubled folded filter (`KIC_FOLD_FILTER_SHIFT=1`)

Each run is `[filter ns, admitted ns, admitted rate, collect ms, build ms,
set-up ms]`:

| row | today's filter | doubled |
|:--|:--|:--|
| k0n53 | [3.84,1.6,0.028,170,77,279] [4.8,1.97,0.028,200,82,328] [4.08,1.69,0.028,179,83,295] | [4.09,0.78,0.006,172,87,294] [4.17,0.78,0.006,175,86,298] [4.47,0.82,0.006,177,89,318] |
| k0n61 | [4.42,1.3,0.02,975,157,1184] [4.32,1.24,0.02,962,148,1159] [4.53,1.34,0.02,981,154,1218] | [4.66,0.59,0.004,913,153,1115] [4.67,0.59,0.004,924,160,1131] [4.94,0.62,0.004,973,158,1191] |

- **What changes:** false positives fall about five-fold and the admitted
  stage halves.
- **What it costs:** the probe costs about 0.2–0.3 ns more, and the build
  about 6 ms more at k0n53.
- **Net:** roughly 3% of collection at k0n61 and nothing measurable at
  k0n53.
- **Side effects:** it changes which keys are falsely admitted, and so
  `ecbench`'s counted bucket reads; it also changes the table's memory.
- **Identity:** certificates and counts matched between arms.

## Key stage, by analysis (no code)

Two-stage least rotation in 32-bit lanes (a 32-bit prefix minimum, then
locating the candidates) costs about as many vector operations as today's
funnel kernel: 244 a 16 keys against 244. So no exact speed-up of the
key stage short of changing the canonical key was found.

Raw outputs: [`runs/`](runs/) (with `SHA256SUMS`).
