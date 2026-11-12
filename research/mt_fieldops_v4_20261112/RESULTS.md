# Multi-target IC v4 results: the IC is dearer than standard multi-target rho in every cell

Protocol: `PROTOCOL.md` (frozen in 25a2540781 before data). Holdout: 33 fresh prime-order curves (group orders
about 2^24, 2^28, 2^32), seeds 31 and 32, T in {1024, 4096, 16384}: **3960 runs, 0 errors, every target of every
run verified**. Data: `results/holdout/rows.jsonl`, `runs.json` (compact counters in Git), `analysis.txt`,
`summary.json`, `noverify.txt`. Every number here is [pcc:unverified] until the claim is signed.

Cost per target in GAE = [(c_I I + M + S)/(c_I + 3) + h (H + L)] / T; ratios are medians of 22 per-configuration
ratios with bootstrap 95% intervals. icbest = the cheaper of ic5 and ic5ks per configuration and price; rhobest =
the cheapest of the 15 retention-rho runs (rho2xks x 3 kappa, rhodp x 12) per configuration and price.

## Result

- **icbest vs rhobest: dearer in all 27 (cell, c_I) decisions**, at every size and T. The median ratio is
  1.003-1.810 over all cells, c_I and h; at I = 10 M it is 1.04-1.16 at h = 0, 1.03-1.08 at h = 0.1 and
  1.20-1.76 at h = 1. ic5 alone and ic5ks alone are also dearer in all 27.
- Without the verification phase that every arm shares (`noverify.txt`, I = 10 M) the ratios are larger:
  1.10-1.17 at h = 0, 1.05-1.09 at h = 0.1, 1.32-1.77 at h = 1.
- The v3 result that ic5 was cheaper than this harness's rho (0.85-0.98, PCC-20261111-dda51e) was a property of
  the rho reference without retention, as the v3 validator found; with the standard multi-target rules, rho wins.
- Distance to BSGS: icbest is 1.14-1.50x a paired BSGS (dearer in all 27).

## Table (I = 10 M)

| bits | T | icbest/rhobest h=0 | h=0.1 | h=1 | icbest/rho2xks h=0 | icbest/rhodp h=1 | icbest/bsgs2 h=0 | GAE/t icbest, rhobest (h=0) |
|---|---|---|---|---|---|---|---|---|
| 24 | 1024 | 1.110 [1.102, 1.123] | 1.063 [1.059, 1.079] | 1.474 [1.461, 1.491] | 1.110 | 1.474 | 1.353 | 142.3, 128.5 |
| 24 | 4096 | 1.073 [1.067, 1.075] | 1.044 [1.038, 1.047] | 1.322 [1.312, 1.329] | 1.073 | 1.322 | 1.237 | 87.9, 82.0 |
| 24 | 16384 | 1.039 [1.039, 1.041] | 1.028 [1.027, 1.030] | 1.197 [1.196, 1.202] | 1.039 | 1.197 | 1.146 | 61.8, 59.4 |
| 28 | 1024 | 1.148 [1.121, 1.158] | 1.083 [1.056, 1.093] | 1.663 [1.644, 1.680] | 1.148 | 1.663 | 1.443 | 456.8, 401.5 |
| 28 | 4096 | 1.112 [1.106, 1.120] | 1.057 [1.050, 1.066] | 1.566 [1.560, 1.571] | 1.112 | 1.566 | 1.371 | 247.2, 222.1 |
| 28 | 16384 | 1.082 [1.079, 1.084] | 1.041 [1.038, 1.043] | 1.446 [1.440, 1.448] | 1.082 | 1.446 | 1.280 | 143.2, 132.4 |
| 32 | 1024 | 1.156 [1.137, 1.166] | 1.083 [1.066, 1.092] | 1.757 [1.741, 1.775] | 1.156 | 1.757 | 1.488 | 1706.9, 1475.8 |
| 32 | 4096 | 1.121 [1.114, 1.125] | 1.054 [1.048, 1.058] | 1.681 [1.667, 1.685] | 1.121 | 1.681 | 1.428 | 855.6, 761.2 |
| 32 | 16384 | 1.103 [1.100, 1.105] | 1.043 [1.040, 1.045] | 1.609 [1.604, 1.612] | 1.103 | 1.609 | 1.378 | 448.1, 405.7 |

Which rho wins (per configuration, I = 10 M): at h = 0 and h = 0.1, `rho2xks` with kappa = 0.5 in all 198
configurations; at h = 1, `rhodp` in all 198 (precomputation kappa 0 or 0.25, d offset -1 to 1). kappa = 0.5 is
the smallest retention-rho database in the grid, so a finer grid could only make the reference cheaper; the
conclusion does not depend on it.

## Why (interpretation)

ic5's table and rho's database are built at the same price per stored class (x-only shared inversions), and
ic5's index-calculus phases are a few per cent of its cost (v3). What remains is coverage per field operation in
the target phase: a retention rho turns every target's walk into stored, known-log points for the next target [pcc:unverified],
so the effective database grows with the targets solved, while ic5's table is fixed after phase 1 (ic5ks's
retained trial classes help it only 0-2%). At h = 1 the distinguished-point rho touches the hash table once per
2^d steps, against once per class for the table methods, which is why the gap is largest there. None of this
uses a property of the curve.

## Pre-registered predictions vs outcome

| prediction | outcome |
|---|---|
| rhodp is the cheapest rho at h = 1, rho2xks at h = 0 | holds (all 198 configurations each) |
| icbest vs rhobest: no cell cheaper at all three h | holds |
| icbest cheaper at h = 0 at 32 b, T = 16384 | wrong: 1.103 [1.100, 1.105] (the smoke compared with rho2xks at kappa 1.0, not the grid's best) |
| most cells unranked, some dearer | wrong: dearer in all 27 |
| icbest dearer than bsgs2 everywhere | holds |

## Answer

For the generic multi-target ECDLP on prime-order curves in this harness's counted field layer, the multi-target
index calculus (ic5, the best variant built here) is not cheaper than Pollard rho [pcc:unverified] once rho uses the standard
multi-target rules: it is 4% to 76% dearer per target, depending on the hash price, and 14% to 50% dearer than
BSGS. Its validated improvement over the earlier IC (v3: 0.47-0.99x of ic4) stands as an engineering result for
the IC baseline; the distance remaining to rho is the table above. Beating rho here would need a non-generic
property of the curve (see `../mt_bsgs_gap_20261111/README.md`).

## Limits

Group orders of about 2^24 to 2^32; costs counted in field operations, not timed; memory unbounded for every arm;
one local host (no AWS identity, so not isolab-eks); the rho arms are this harness's (not ecbench's
`rho.multitarget_pooled`), and `rhodp`'s fruitless-cycle handling (step switching plus abandoning walks over
20 * 2^d steps) is one standard choice among several; the comparison is per target in a batch of T on one curve
and says nothing about single-target ECDLP or about ECC2K-130 / m = 83.
