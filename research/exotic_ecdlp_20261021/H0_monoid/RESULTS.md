# H0 results — additive-closure defect of the Semaev interval base (EXP-EXOTIC-H0-001)

Run: isolab job `exotic-h0-monoid-r6` (final; deterministic re-run of r5 with a refined exponent fit; r4 added the hashed control, r5 the cost arm; 2 CPU / 4Gi; compute a few minutes; seed 20261022 + p for numpy, Sage seed 20261022). Program: `run.py`; data: `out/result.json`.
Infra note: attempt 1 (`exotic-h0-monoid`) ran an empty program (wrapper truncated the input file) and r2 was rejected (8 KB arg limit); both were harness bugs, fixed centrally. r3-r6 all give identical real-x numbers (same seed). No cell censored.

## Rubric
- **Object**: random `E/F_p`, `p in {1009, 65537, 1048583}`, base `F = {P : 0 <= x < ceil(sqrt p)}` (both y-signs); curve cofactor of the large prime subgroup is 1, 3, 5.
- **Observable**: |F|; sumset coverage rho_m (m=1..6, K=1e5 random m-subsets, T=2000 targets); normalized rank `x(R-Q)/p` (uniformity by KS, one random Q per R, n=2000); greedy recursion `R <- R - Q*` for 10 steps, 2000 walks.
- **Null**: rho_m follows random-sampling coverage; `x(R-Q)` rank uniform; no recursive shortcut.
- **Win**: rho_m = Theta(1) at m = O(polylog) and a size decrease that strengthens with p.
- **Kill**: rank uniform (KS p > 0.01) at all p and recursion slope >= 0.

## Table
| p | N | cofactor | abs(F) | sqrt p | rho_1 / rho_6 | random-draw null rho_6 | KS p (rank, 1 random Q) | recursion slope (steps 1..10, rank/step) | z | slope / (1/(abs(F)+1)) |
|---|---|---|---|---|---|---|---|---|---|---|
| 1009 | 983 | 1 | 18 | 31.8 | 0.017 / 1.000 | 1.000 | 0.062 | -2.55e-3 | -22.1 | -0.049 |
| 65537 | 65523 | 3 | 264 | 256 | 0.0045 / 0.999 | 1.000 | 0.551 | -3.14e-4 | -37.0 | -0.083 |
| 1048583 | 1047385 | 5 | 1060 | 1024 | 0.0005 / 0.362 | 0.436 | 0.158 | -7.98e-5 | -40.3 | -0.085 |

rho_m curves (m=1..6): p=1009: .017,.18,.65,.98,1,1; p=65537: .0045,.39,.87,.97,.99,.999; p=1048583: .0005,.085,.16,.24,.31,.36. For p=1048583 coverage at m=6 is sample-limited (K*m/N ~ 0.57); it sits at or below the random-draw baseline at every m, and below the combinatorial sumset bound where that is <1 (m=1,2 check: .0005 vs .0010; .085 vs .54 is draw-limited). At small p, rho_m saturates by m ~ 4 simply because |F|^m exceeds N; this is the dumb sumset bound, no sign of anything beyond it.
Other observables: |F|/sqrt p = 0.57, 1.03, 1.04. Median `x(R-Q)` rank 0.501/0.509/0.486 (one random Q); mean 0.508/0.506/0.490. Best-Q rank (min over F): mean 0.073/0.0050/0.0013 (median 0.060/0.0045/0.0011), versus the iid-uniform expectation 1/(|F|+1) = 0.053/0.0038/0.00094. 200 replicate KS tests (500 pairs each): fraction with p<0.01 = 0.02/0.01/0.00.
Mean rank by greedy step (0..3): p=1009 0.505, 0.075, 0.064, 0.040; p=65537 0.496, 0.0050, 0.0028, 0.0014; p=1048583 0.504, 0.0012, 0.00065, 0.00028. Fraction of walks with strict decrease at step 1->2: 0.58/0.84/0.85; it falls to ~5% by steps 5-9 at the two larger p.

## Cost arm (the arbiter; supersedes the slope rule)
Accounting: each greedy step enumerates |F| additions R+Q; a walk finding R-Q=O at step s used s*|F| group ops and gives a decomposition of s base elements. Walks capped at 10 steps; "amortized" = total ops over all 2000 walks / number of successes (charges failed walks).

| p | abs(F) | success rate | median ops (successful walk) | amortized ops / decomposition | amortized ops / sqrt p | amortized steps (= ops/abs(F)) | length median / max |
|---|---|---|---|---|---|---|---|
| 1009 | 18 | 0.421 | 54 | 298 | 9.39 | 16.6 | 3 / 6 |
| 65537 | 264 | 0.834 | 792 | 1271 | 4.96 | 4.81 | 3 / 8 |
| 1048583 | 1060 | 0.877 | 3180 | 4473 | 4.37 | 4.22 | 3 / 7 |

Length histogram (length: count) p=1048583: 2:848, 3:499, 4:320, 5:61, 6:18, 7:8 (full histograms for all p in result.json).
Exponent fit, ops ~ p^c (sqrt N reference 0.5):
- primary, p in {65537, 1048583} (|F|/sqrt p = 1.03-1.04): c = 0.454 (amortized), 0.501 (median of successful walks).
- all three p: c = 0.386 (amortized), 0.591 (median). These bracket 0.5; the amortized value is pulled down by p=1009, where |F|=18 (|F|/sqrt p = 0.57) and the capped walks are a finite-size regime.
- Excluding p=1009 from the primary fit was my choice after seeing the data; the literal 3-point amortized rule (c < 0.45) would read WIN, stored as `verdict_literal_3pt_amortized_fit` = "win". I judge it an artifact: ops = (steps) x |F|, steps is O(1) and flat (4.8 -> 4.2) at the two larger p, and |F| ~ sqrt p, so cost per decomposition is Theta(sqrt p) with a constant ~4-5. Three (or two) points cannot resolve an exponent to better than about +/-0.1.
- Decomposition length is O(1) (median 3, max 6-8, not growing clearly with p), but the cost to find one is a full |F| scan per step, i.e. birthday-type search.

## Verdict (cost rule): KILL
Cost per relation scales like sqrt(p) (c ~ 0.45-0.50 on the p >= 65537 fit; ops/sqrt p flat at about 4-5), no sub-sqrt recursion; with |F| ~ sqrt p unknowns still to be solved, total cost is above rho. This is C0 surviving. `result.json` verdict = "kill". Rank uniformity also holds (KS p = 0.062, 0.551, 0.158).
The earlier slope "win" (r4) was an artifact: F is defined as {x < ceil(sqrt p)} and the greedy arm minimizes x, so it steers into F by construction, while the hashed-label control cannot. It is superseded by the cost measurement and kept below only as context.

## Context (demoted): real vs hashed-label slope
| p | real slope (z) | hashed slope (z) | real-minus-hashed z | hit rate real / hashed |
|---|---|---|---|---|
| 1009 | -2.55e-3 (-22.1) | -1.24e-3 (-17.1) | -9.6 | 0.421 / 0.024 |
| 65537 | -3.14e-4 (-37.0) | -1.46e-4 (-22.3) | -15.6 | 0.834 / 0.048 |
| 1048583 | -7.98e-5 (-40.3) | -3.04e-5 (-19.6) | -19.7 | 0.877 / 0.003 |
Hashed control = fixed random permutation of x (F unchanged); hash of full (x,y) not tried. The top-level `recursion_slope` in result.json (-2.55e-3, real arm) is retained for the harness metric but is not evidence.

## Caveats
- The probe has no direct rho cost. Gap to rho: qualitative only. Nothing here gives a relation-finding cost; a decreasing greedy x-rank does not by itself yield a membership test cheaper than sqrt(p) or a subadditive size.
- The raw negative slope is largely a generic artifact (the hashed control also descends); see the control section. The hashed control permutes x only; a hash of the full point (x,y) was not tried.
- Step 0 -> 1 is a selection effect (random point to best-of-|F|); the slope uses steps 1..10 only. Including step 0 gives about -0.023 at every p (not informative).
- Best-Q rank vs the Beta(1,|F|) null: KS rejects (p ~ 1e-80 or lower), and pooled all-pairs KS is small at p=1009 and 65537 because pairs sharing R are dependent; the decision uses one independent random Q per R. The best-Q mean is above the iid expectation (x-values of points occupy about half of [0,p), and sums over Q are not independent).
- Interpretation of "200 KS": 200 replicate KS tests of 500 pairs each. rho_m uses cumulative sums over j <= m of distinct-element subsets; sample-limited at the largest p. Small-p cells (|F|=18 at p=1009) are weak evidence. G (prime-order generator) was built and validated but is not used by the probes.
- Validation: fields_built, generator_order_prime_factor_ok, vadd_matches_sage (vectorized addition checked against Sage on 60 pairs per p) all true.
