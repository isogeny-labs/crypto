# AMENDMENT 1 (frozen 2026-11-23 after the first pass, before any replication data)

The first pass (PROTOCOL.md, fixed sample) gave at g = 3, p = 65537 a pooled sibling-minus-forward
full-split rate difference of −0.0022 [−0.0043, −0.0000] (z ≈ −2.0, one of three cells, no
multiplicity correction was pre-specified). By the frozen letter **P2 failed at g = 3**, and that
first-pass verdict stays recorded. This amendment adds a pre-registered replication; it does not
replace the first-pass result.

- Replication R2: the same 6 genus-3 curves, fresh walk seeds (s = 3), 120,000 forward steps per
  curve, arm N (forward step + independent `cantor(D, −M)`; its siblings are bit-identical to arm
  B's, checked on 100 % of first-pass B steps: 0 mismatches). Same split test.
- Pre-stated readings:
  - **R-real:** the pooled 95 % interval of (sibling − forward) lies below 0 → the deficit is
    real; report its relative size and the net per-relation effect
    (B/A per candidate) × (forward rate / sibling rate).
  - **R-noise:** the interval contains 0 → the first-pass P2 failure is treated as a false alarm
    in the overall reading, while P2 is still listed as failed in the first-pass scorecard.
- Expected under "real at the first-pass size": SE ≈ 0.0006, so −0.0022 would be about 3.4 SE.
- Also recorded post hoc before this amendment (diagnostic `diag_dups.py`, not in the protocol):
  at g = 2, p = 65537 (r ≈ 5–11 × 10^8) the P3 duplicate count (9,329 siblings) comes from
  merged walk trails: on curve 17 over 60,000 steps the forward walk itself repeats 6,182
  points, siblings repeat 5,787 earlier siblings and only 3 forward points. P3 failed at that cell
  by its letter; its cause is the walk (4096-step segments against r < 10^9), shared by both arms.
