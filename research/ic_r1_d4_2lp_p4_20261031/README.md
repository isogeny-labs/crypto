# R1: D4 + double large primes on E(F_{p^4}), end to end (2026-10-31)

Branch `research/ic-r1-d4-2lp-p4-20261031` (base origin/main 01b9d8e768). Protocol and spec frozen before any run:
[`PROTOCOL.md`](PROTOCOL.md), [`SPEC-r1.json`](SPEC-r1.json) (run as per-p slices in `specs/`).

## Verdict: constant factor / null on the exponent. The stack does not lower the exponent; it raises it by about 0.017.

On E(F_{p^4}) at p = 101..1033, stacking D4 with 2LP does not help. Relative to D4 alone the stack costs 1.00x at p = 101 growing to
1.04x at p = 1033, and its fitted exponent in p is higher by 0.017 (paired 95% CI 0.013 to 0.021). The reason is structural:
the relation phase (trials charged a fixed c4 each) exceeds the linear-algebra phase by a factor of at least 3e7 everywhere (LA share 4e-10 to 3e-8),
so 2LP, which only shrinks LA (0.80x of D4's LA at p = 1033) while adding 3 to 6% more trials (large-prime decompositions with 3+ large primes are wasted), cannot win in this
range. The LA share would reach 50% only if one trial cost about 6 GAE (a few group operations, against 2.6e8 GAE charged), so no repricing of c4 changes the verdict.
D4 alone versus plain is a large constant (0.04x of plain S) but that ratio mixes two measured c4 charges (D4 2.54e10, x-line 1.213e12), not a
harness-measured gain; the slope difference plain minus D4 is 0.09 [-0.01, 0.19], unresolved with 3 curves per p.

## What was run

- Producer: `ic.k4_gaudry` (`src/cryptanalysis/ecbench/fp4.rs`), the only registered producer exposing both D4 (`base=yline-d4`) and 2LP
  (`large_primes=2`). `ic.pipeline` (`fghr-y-line` + `gaudry-d4`, curves `fp4_torsion`) has no large-prime option and Macaulay oracle only to p ~ 29, so it cannot carry the stack.
- Linear algebra: the repo's CADO-style filter to a square core, then Wiedemann (the `sparse-cado` = `filter+wiedemann` pair in the pipeline). It is accepted only when
  [d]G = Q; the first attempt is at small+1 full relations, retry +5%. That is the full-rank completion rule (not stop-at-target-pin). The old null of 10-18 does not apply to this producer.
- Correction to review M3: the D4+2LP stack was already run (item `ic_improve_20261014/01`, same producer, 24 held-out curves, p = 101..1301): whole 1.025x of D4, exponent +0.0055.
  R1 reproduces the sign and size on new curves (1.00 to 1.04x; +0.017) at a different seed set.
- Arms: rho-neg (reference), plain (x-line), d4, 2lp (x-line, lp_exponent 0.95), d4-2lp (lp_exponent 0.95, the item-01 frozen value, not retuned).
- 7 sizes p = 101, 181, 269, 397, 521, 769, 1033; curve seeds 1 to 3; 2 targets each; 210 runs.

## Per-arm per-p table (mean of 3 curves x 2 targets)

| arm | p | counted cost, GAE (mean of 6) | S = GAE/sqrt(r) | trials | relations used | full rels | columns | LA cost, GAE | LA share |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| rho-neg | 101 | 4.093e+03 | 0.803 | - | - | - | - | - | - |
| rho-neg | 181 | 1.802e+04 | 1.100 | - | - | - | - | - | - |
| rho-neg | 269 | 3.218e+04 | 0.890 | - | - | - | - | - | - |
| rho-neg | 397 | 4.388e+04 | 0.557 | - | - | - | - | - | - |
| rho-neg | 521 | 7.957e+04 | 0.586 | - | - | - | - | - | - |
| rho-neg | 769 | 2.351e+05 | 0.795 | - | - | - | - | - | - |
| rho-neg | 1033 | 3.791e+05 | 0.711 | - | - | - | - | - | - |
| plain | 101 | 1.792e+13 | 3.513e+09 | 1433 | 53 | 53 | 52 | 7.541e+03 | 4.2e-10 |
| plain | 181 | 3.142e+13 | 1.918e+09 | 2512 | 90 | 90 | 89 | 2.149e+04 | 6.8e-10 |
| plain | 269 | 4.870e+13 | 1.346e+09 | 3894 | 132 | 132 | 131 | 4.532e+04 | 9.3e-10 |
| plain | 397 | 6.683e+13 | 8.480e+08 | 5344 | 198 | 198 | 197 | 1.058e+05 | 1.6e-09 |
| plain | 521 | 7.870e+13 | 5.799e+08 | 6293 | 267 | 267 | 266 | 1.825e+05 | 2.3e-09 |
| plain | 769 | 1.273e+14 | 4.307e+08 | 10183 | 380 | 380 | 379 | 3.727e+05 | 2.9e-09 |
| plain | 1033 | 1.543e+14 | 2.892e+08 | 12340 | 530 | 530 | 529 | 7.186e+05 | 4.7e-09 |
| d4 | 101 | 8.647e+11 | 1.695e+08 | 3302 | 24 | 24 | 23 | 1.608e+03 | 1.9e-09 |
| d4 | 181 | 1.547e+12 | 9.443e+07 | 5907 | 43 | 43 | 42 | 4.643e+03 | 3.0e-09 |
| d4 | 269 | 2.001e+12 | 5.531e+07 | 7643 | 66 | 66 | 65 | 1.099e+04 | 5.5e-09 |
| d4 | 397 | 3.060e+12 | 3.883e+07 | 11685 | 97 | 97 | 96 | 2.475e+04 | 8.1e-09 |
| d4 | 521 | 3.239e+12 | 2.386e+07 | 12369 | 133 | 133 | 132 | 4.648e+04 | 1.4e-08 |
| d4 | 769 | 4.942e+12 | 1.671e+07 | 18873 | 194 | 194 | 193 | 9.935e+04 | 2.0e-08 |
| d4 | 1033 | 6.299e+12 | 1.181e+07 | 24054 | 265 | 265 | 264 | 1.872e+05 | 3.0e-08 |
| 2lp | 101 | 1.827e+13 | 3.583e+09 | 1461 | 52 | 44 | 52 | 6.358e+03 | 3.5e-10 |
| 2lp | 181 | 3.144e+13 | 1.919e+09 | 2514 | 89 | 73 | 89 | 1.761e+04 | 5.6e-10 |
| 2lp | 269 | 5.085e+13 | 1.405e+09 | 4066 | 131 | 104 | 131 | 3.694e+04 | 7.3e-10 |
| 2lp | 397 | 6.828e+13 | 8.665e+08 | 5460 | 197 | 153 | 197 | 8.412e+04 | 1.2e-09 |
| 2lp | 521 | 8.030e+13 | 5.916e+08 | 6421 | 266 | 203 | 266 | 1.452e+05 | 1.8e-09 |
| 2lp | 769 | 1.320e+14 | 4.464e+08 | 10556 | 376 | 283 | 379 | 2.937e+05 | 2.2e-09 |
| 2lp | 1033 | 1.632e+14 | 3.059e+08 | 13052 | 528 | 388 | 529 | 5.622e+05 | 3.4e-09 |
| d4-2lp | 101 | 8.647e+11 | 1.695e+08 | 3302 | 24 | 21 | 23 | 1.372e+03 | 1.6e-09 |
| d4-2lp | 181 | 1.558e+12 | 9.512e+07 | 5950 | 42 | 36 | 42 | 3.944e+03 | 2.5e-09 |
| d4-2lp | 269 | 2.014e+12 | 5.566e+07 | 7691 | 65 | 54 | 65 | 9.587e+03 | 4.8e-09 |
| d4-2lp | 397 | 3.108e+12 | 3.944e+07 | 11868 | 96 | 78 | 96 | 2.011e+04 | 6.5e-09 |
| d4-2lp | 521 | 3.337e+12 | 2.459e+07 | 12743 | 132 | 105 | 132 | 3.798e+04 | 1.1e-08 |
| d4-2lp | 769 | 5.099e+12 | 1.724e+07 | 19472 | 193 | 150 | 193 | 8.053e+04 | 1.6e-08 |
| d4-2lp | 1033 | 6.528e+12 | 1.223e+07 | 24929 | 264 | 201 | 264 | 1.495e+05 | 2.3e-08 |

Verified logs: 210 of 210 runs `verified`, [d]G = Q and equal to the planted d. `ecbench verify --replay-all`: audit OK, 30 records, 30 replays reproduced, 0 problems for each of the 7 sessions
(`sessions/p*.audit.json`, SHA-256 prefixes p101 caeee151c15c, p181 525e3367c46c, p269 2321712989ca, p397 6363c1e79385, p521 17334715ed1e, p769 11d2a2f4b85e, p1033 7745482f2a28).
Raw records: `sessions/p*/records.jsonl`; analysis: `analyse.py` -> `analysis.json`.

## Fitted exponents (log2 cost vs log2 p, OLS on per-curve means, 95% bootstrap over curves within p, 4000 resamples)

Cost = total counted GAE (the exponent in p of total counted cost). S = GAE/sqrt(r) has slope lower by about 2 (r ~ p^2).

| arm | total GAE slope [95% CI] | relation phase | LA phase | S slope |
|:--|:--|:--|:--|:--|
| rho-neg | 1.855 [1.681, 2.028] | - | - | -0.145 [-0.319, 0.028] |
| plain | 0.935 [0.871, 0.995] | 0.935 [0.871, 0.995] | 1.971 [1.922, 2.023] | -1.065 |
| d4 | 0.847 [0.747, 0.958] | 0.847 [0.747, 0.958] | 2.072 [2.002, 2.137] | -1.153 |
| 2lp | 0.949 [0.886, 1.008] | 0.949 [0.886, 1.008] | 1.939 [1.897, 1.986] | -1.051 |
| d4-2lp | 0.864 [0.766, 0.974] | 0.864 [0.766, 0.974] | 2.039 [1.974, 2.100] | -1.136 |

Paired slope differences (same curves, same resamples): d4 minus d4-2lp = -0.017 [-0.021, -0.013] (stack is steeper: regresses);
plain minus 2lp = -0.014 [-0.019, -0.009] (2LP alone also steeper); plain minus d4-2lp = 0.071 [-0.026, 0.175] and d4-2lp minus 2lp = -0.085 [-0.19, 0.011] (D4 effect unresolved);
LA phase: d4-2lp minus d4 = -0.033 [-0.050, -0.018], i.e. 2LP lowers the LA exponent by 0.03 (2.07 to 2.04) as in item 01, but LA is not a cost term here.
Total-cost slope versus rho: rho-neg minus d4-2lp = 0.99 [0.77, 1.22]. All IC arms are far below rho in exponent only because their cost
is a count of trials charged at a constant per-trial price; in absolute terms every IC arm is 1e7 to 1e9 times rho in S (S d4-2lp 1.2e7 vs rho 0.71 at p = 1033).
The IC/rho S ratio falls with p (plain 4.5e9 at p = 101 to 4.1e8 at p = 1033; d4-2lp 2.2e8 to 1.7e7), the expected lower-exponent signature of the Gaudry regime. The 1.86 rho slope is p^2 within noise.

Reprice sweep (c4 scaled, LA kept as counted; analysis.json `fit_S_reprice_c4_scale`): the d4-minus-d4-2lp slope stays negative at every scale down to c4 x 1e-7 (-0.012 [-0.015, -0.009]); no scale makes the stack win.

## Caveats

- `ic.k4_gaudry` charges each trial a measured constant c4 and finds decompositions with an exact-enumeration oracle counted uncharged, so every S is a lower bound
  and the exponent reflects trials (~ number of columns) times a constant. The real per-trial algebraic cost at larger p is not measured by this harness; D4 vs plain mixes two c4 constants.
  The R10 ledger notes #1792 measured the D4 solve 311 to 370 times cheaper than 2.54e10; c4 is constant across p so slopes are unaffected.
- p <= 1033 only (p < 2^15 is the producer limit; p = 1033 session took 1685 s with replay audits extra). Larger p was affordable in time but the oracle tables grow; not attempted.
- 3 curves per p, 2 targets each; slope CIs are wide for the D4-vs-plain comparison. 2LP parameters were not retuned (item 01 found no setting below 1.0).
- Curves are not in the registry (registered=false); no leaderboard claim. Wall time not used (L0, loaded host).
- Not done: `ic.pipeline` sparse-cado arm with real Macaulay oracle (no 2LP there); twist/GLS variants (stop list); equal-advice or published-baseline (FGHR, Joux-Vitse) timing comparison.
