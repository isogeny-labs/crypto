# Wave 3 depth predictions (frozen before the t ≥ 8 runs, 2026-11-15)

Every number is `[pcc:unverified]`. G_t(c) is the number of rounds in which the round-indexed closure at degree c grows. r* is the round at which 1 enters.

Observed before freezing (from `sat.jsonl` and `depth.jsonl`):
- m = 2: r* = 2t at c = d_F for t = 2…5. The lemma's bound is attained.
- m = 2, c = 4: G = 11 at t = 6 and at t = 7.

Predictions:
- P1. m = 2, c = 4: G_t(4) = 11 at t = 8, 9, 10, and 1 never enters.
- P2. m = 2, c = 5, t = 8: 1 enters at r* = 16 = 2t.
- P3. m = 2, c = 5, t = 10: G_10(5) = 19, and 1 never enters. This comes from the growth-law fit `d_F = 4 + ⌊(t−2)/4⌋`, which steps at t = 6, 10, 14, combined with `d_F = min{c : G_∞(c) ≥ 2t}`.

Falsifiers:
- P1 fails if G_t(4) ≠ 11 at any of t = 8, 9, 10.
- P3 fails if G_10(5) ≠ 19, or if 1 enters.

## Addendum, frozen before the c = 5, t ≥ 11 runs (2026-11-15T08:20Z)

Observed before freezing (Rust `hky_depth`, m = 2):
- `G_10(5) = 19`, and 1 never enters. P3 held.
- Planted (consistent) draws give `G = 15, 17` at t = 8, 9. That is `2t − 1`, which also gives 19 at t = 10, so t = 10 cannot tell saturation from growth.

Prediction:
- P4. Conjecture D (saturation): `G_11(5) = G_12(5) = 19`.
- The alternative "still growing as 2t − 1" predicts 21 and 23.
- Falsifier: P4 fails if `G_11(5) ≠ 19`.
