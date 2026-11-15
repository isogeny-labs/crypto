---
title: "ic.pipeline size=auto against the tuned prime-field baseline: equal at 20 bits, a pricing-band trade at 24 and 28"
date: 2026-11-07
---

# ic.pipeline size=auto against the tuned prime-field baseline: equal at 20 bits, a pricing-band trade at 24 and 28

**Result (measured, counted GAE, local L0 host).** This compares
`prime-abscissa:size=auto` (B = round(0.5·r^(1/3))) with the current
prime-field IC baseline of `research/prime_base_oracle_tuning_20261029`:
N = 45 at 20 bits, and the kept incumbents N = 162 at 24 and N = 407 at 28.
It used that study's holdout curves, the same MITM pair-table oracle, 48
targets × 2 rounds, an A/A control and matched `rho.negation_v3`.

| bits (log2 r) | B auto | N tuned | auto / tuned, w = 0 | w = 0.1 | w = 1 | reading |
|:--|--:|--:|:--|--:|--:|:--|
| 20 (19.62) | 46 | 45 | 0.998 [0.987, 1.010] | 0.997 | 0.990 | equal |
| 24 (23.51) | 114 | 162 | 0.914 [0.883, 0.946] | 0.955 | 1.153 | order flips in the band |
| 28 (27.57) | 292 | 407 | 0.885 [0.860, 0.910] | 0.923 | 1.107 | order flips in the band |

B auto is the value `size=auto` took on each holdout curve, read from the
records' factor-base descriptions. It falls within 1–4 of the 2026-10-29
study's own selected N (45, 115, 288), so the r^(1/3) rule reproduces that
hand tuning without a per-size search. The study kept its larger incumbents
at 24 and 28 bits for the same band reason given below.

**This is not an improvement under AGENTS.md §5a.** At 24 and 28 bits the
smaller base saves charged relation work but adds unpriced linear-algebra
row operations and table probes, so the order reverses at 1 GAE per native
operation. That is the same reading the 2026-10-29 study gave its own
smaller selected N (115 and 288), and why it kept N = 162 and 407. The
baseline is unchanged.

Distance to matched rho.negation_v3 (w = 0): auto 8.47× / 13.4× / 19.0×;
tuned 8.49× / 14.6× / 21.4× at 20, 24 and 28 bits.

## What this points to

At these sizes the IC pipeline's remaining lever is the native work of its
linear algebra, not the base size. With m = 2 each relation row has at most
three non-zeros, yet incremental Gauss pays row operations that grow with
B. Reducing that native work, by a sparse or structured elimination,
would move the w = 1 column and is the next candidate.

## Run

| item | value |
|:--|:--|
| specs | `specs/P20.json`, `specs/P24.json`, `specs/P28.json` |
| sessions | `ECBS1h0d37b0d4de7f`, `ECBS1h410a3ad0a7df`, `ECBS1h3525e40319e5`; 576 records each, all verified |
| audits | `audit-P*.json`: replay-all, 384/384 each, 0 problems |
| band tables | `band-P*.md`, `band-P*.json` |
| isolation | L0 container |
