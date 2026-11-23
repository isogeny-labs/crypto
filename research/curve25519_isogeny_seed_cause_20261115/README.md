# Why isogeny seeds differ in hits per mul-add

All claims `[pcc:unverified]`. Protocol: [PROTOCOL.md](PROTOCOL.md), committed (`7cf8f9ada3`) before any correlation was computed.
Re-analysis of PR #4213's 42 engine JSONs, the tower costs and the V dumps; no new runs. Output: [summary.txt](summary.txt), `analyse.py`.

## Answer
The seed matters through **how many of the 32 x-values in V lie on the miniature curve**. Per-seed pooled rate (blind + lifts, n = 21 seeds)
correlates with that count at Spearman +0.959 (permutation p < 5e-5). It ranges from 10 on-curve x (seed 13, rate 0.88) to 21 (seed 17, rate 4.72).

| feature | Spearman | perm. p |
|---|---|---|
| F1 `|V|` | constant (32 for every seed) | n/a |
| F2 on-curve fraction of V | +0.959 | < 5e-5 |
| F3 descent-class minority fraction | +0.077 | 0.74 |
| F4 engine `base_points` | +0.959 | < 5e-5 |
| F5 covered points, all cosets | +0.959 | < 5e-5 |
| F6a v2(aux order) | -0.334 | 0.14 |
| F6b abs(aux order - (p+1)) | +0.204 | 0.37 |
| F7 curves tried (control) | +0.211 | 0.36 |

H10 supported (Bonferroni 0.0071). H11 supported (engine counters 0.96 vs 0.33 and 0.20). H12 supported (control p = 0.36).
Caveat: F2, F4, F5 are one quantity, not three independent confirmations: `base_points` is 2 x (on-curve x count) in all 21 seeds, and
covered points follow from it. The descent-class mix (F3) and the number-theoretic features of the auxiliary curve do not explain it.

## Exploratory (post-hoc, not in the protocol)
Hits proportional to base points x exposure: blind arm consistent (0.0913 hits per 1e9 mul-adds per base point; chi2 17.8 on 20 df, approx p = 0.60),
so the blind-arm seed differences are fully accounted for by base-point count within Poisson noise. The lifts arm is still overdispersed
(chi2 66.5 on 20 df): per-base-point yield of lifts varies with something beyond the count (not identified; seed 13 gives 25 hits against 51 expected,
seed 17 gives 134 against 107). Details: `exploratory.py`, `exploratory.txt`.

## What it means for the toy
Base-point count is known from the tower dump before any F4 work, so a seed screen costs tower operations only (492 to 6,823 here, negligible at unit weight).
That predicts a screened seed would sit near the top of the 0.85 to 4.8 range; this is a prediction, not tested here, and selecting among seeds
would need its own accounting. Single k = 20, t = 5, one target set.
