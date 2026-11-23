# Multi-target IC v4 results: against standard multi-target rho, no ranking is established

Protocol: `PROTOCOL.md` (frozen in 25a2540781 before data). Holdout: 33 fresh prime-order curves (group orders
about 2^24, 2^28, 2^32), seeds 31 and 32, T in {1024, 4096, 16384}: **3960 runs, 0 errors, every target of every
run verified**. Data: `results/holdout/rows.jsonl`, `runs.json` (compact counters in Git, `mk_runs.py`),
`analysis.txt`, `summary.json`, `noverify.txt` (`noverify.py`). Every number here is [pcc:unverified] (claim PCC-20261112-830272) until the
claim is signed. Independent validation: `validation/VALIDATOR_REPORT.md` (measurement valid: 3960/3960 rows
replay exactly, 1.29M targets checked with independent EC code, every number recomputes; the headline below is
the validator's amended one, and its required amendments are applied in this file).

Cost per target in GAE = [(c_I I + M + S)/(c_I + 3) + h (H + L)] / T; ratios are medians of 22 per-configuration
ratios with bootstrap 95% intervals. icbest = the cheaper of ic5 and ic5ks per configuration and price; rhobest =
the cheapest of the 15 runs of this harness's multi-target rho (rho2xks x 3 kappa, rhodp x 12) per configuration
and price, so it takes the cheaper of two different rho algorithms at each price.

## Result

- **Under the frozen rule** (h charged on hash-table and solver operations only) icbest is dearer than rhobest in
  all 27 (cell, c_I) decisions: median ratio 1.003-1.810 overall; at I = 10 M 1.04-1.16 at h = 0, 1.03-1.08 at
  h = 0.1, 1.20-1.76 at h = 1. Against either rho algorithm alone (rho2xks or rhodp) all 27 are unranked.
- **Under section 5a pricing** the ranking disappears. rhodp, the h = 1 reference, canonicalises every point and
  computes step-index hashes and a distinguished-point test per step; none of these were counted (7.27e9
  canonicalisations against 7.66e8 priced hash operations over its runs; `validation/native_work.json`).
  AGENTS.md section 5a prices such unpriced native work in the band: priced at h, all 27 decisions are unranked
  and at I = 10 M, h = 1 the IC is 0.888-0.935x the cheapest rho.
- **The IC side was handicapped too.** ic5ks retains the trial classes of finished targets, but every target walks the same
  arithmetic progression W_k = W_0 + kD, so the retained classes cover O(K) target logs instead of O(K K'); its
  saving is at most 1.5%. The validator's sensitivity arm ic5rw (W a sum over 32 known-log points, nothing else
  changed) costs 0.79-0.93x of icbest (I = 10 M, h = 0) and is unranked against the frozen rhobest in all 27.
- **The rho side was handicapped as well** (both make rho dearer, so the frozen-rule "dearer" is conservative):
  rhodp's 1024 steps include repeated values (about 39% of its steps duplicate another), which makes 2-cycles its
  look-ahead misses, so 1-34% of its steps go to abandoned walks (a variant with distinct steps, rhodpu, costs
  0.81-0.99x of rhodp at h = 1); and rho2xks's best kappa is the grid floor 0.5 (kappa <= 0.25 is cheaper in all
  198 configurations).
- With every sensitivity arm (validator, not pre-registered): the stronger rho references against icbest are
  dearer 27 under the frozen rule and unranked 20 / dearer 7 under section 5a pricing; against ic5rw they are
  dearer 14 / unranked 13 (frozen rule) and unranked 19 / cheaper 6 / dearer 2 (section 5a pricing).
- **Distance to BSGS:** icbest is 1.14-1.49x a paired BSGS (dearer in all 27).
- The v3 result that ic5 was cheaper than this harness's rho without retention (PCC-20261111-dda51e) does not
  carry over to a rho with retention.

## Table (I = 10 M, frozen rule)

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
configurations; at h = 1, `rhodp` in all 198 (precomputation kappa 0 or 0.25, d offset -1 to 1).

Without the verification phase that every arm shares (`noverify.txt`, I = 10 M, frozen rule): 1.09-1.16 at h = 0,
1.05-1.09 at h = 0.1, 1.32-1.77 at h = 1.

## Why (interpretation)

ic5's table and rho's database are built at the same price per stored class (x-only shared inversions), and
ic5's index-calculus phases are a few per cent of its cost (v3). The remaining differences are how much each
method's stored set grows as targets are finished, and how much native work each does per step. A retention rho
stores every finished target's walk; ic5ks's retention is weak here only because all targets share one offset
progression, which is a design flaw of this arm, not a limit of the IC (ic5rw removes most of it). At h = 1 the
distinguished-point rho looks up the hash table once per 2^d steps, but it does a canonicalisation and a step
hash per step, and whether those count decides the ranking at h = 1.

## Pre-registered predictions vs outcome

| prediction | outcome |
|---|---|
| rhodp is the cheapest rho at h = 1, rho2xks at h = 0 | holds (all 198 configurations each) |
| icbest vs rhobest: no cell cheaper at all three h | holds |
| icbest cheaper at h = 0 at 32 b, T = 16384 | wrong: 1.103 [1.100, 1.105] (the smoke compared with rho2xks at kappa 1.0, not the grid's best) |
| most cells unranked, some dearer | wrong under the frozen rule (dearer in all 27); holds under section 5a pricing (unranked in all 27) |
| icbest dearer than bsgs2 everywhere | holds |

## Answer

No ranking of this harness's multi-target index calculus against multi-target rho is established. Under the
frozen rule the IC is 3% to 76% dearer per target than the cheaper of two standard multi-target rho algorithms at
each price, but that ranking rests on not pricing rho's per-step canonicalisation, which section 5a prices; and
both sides carry handicaps that a better-built arm removes (rho: duplicate steps, kappa grid floor; IC: the shared
offset progression). The IC remains 14% to 49% dearer than BSGS. The validated v3 gain of the IC itself [pcc:PCC-20261111-dda51e]
(ic5 0.47-0.99x of ic4) stands. Deciding the rho comparison needs a pre-registered v5 with the stronger arms
(ic5rw, a rhodp with distinct steps, a lower kappa grid) and with rho's canonicalisations, step hashes and
distinguished-point tests counted and priced in the band.

## Limits

Group orders of about 2^24 to 2^32 (7 of the 11 32-bit-field curves have order just above 2^32); costs counted in
field operations, not timed; memory unbounded for every arm; one local host (no AWS identity, so not isolab-eks);
the rho arms are this harness's (not ecbench's `rho.multitarget_pooled`); pricing a canonicalisation at h is the
section 5a band's convention, not a measured cost; the sensitivity arms are the validator's and not
pre-registered; the comparison is per target in a batch of T on one curve and says nothing about single-target
ECDLP or about ECC2K-130 / m = 83.
