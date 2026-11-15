# Idea 9 — solution-rich regime, budgets, XOR streamlining (pilot, 2026-11-14)

Pilot, toy sizes, one core per run, 4-core shared container. Not claim data.
Every number below is an unverified pilot assertion. Protocol and kill criterion
were written first: `PROTOCOL.md` (plus Amendment 1, written after unmeasured
timing probes and before the measured runs).

## What was run

- Curve y^2+xy = x^3+x^2+1 over F_{2^n}, n in {7, 11, 13}; m = 3; V = span(1..z^{l-1}).
- One CNF encoding for every SAT arm: chain S3(x1,x2,t)=0, S3(x3,x(R),t)=0, t free
  (n bits), Tseitin ANDs, XORs cut at 4 literals, no symmetry breaking (`pdp.py`).
- Arms on the same targets, interleaved per target (`run.py`): (a) Glucose 4.1
  untreated, CaDiCaL 1.5.3 untreated (small l only); (b) per-target conflict budget,
  computed post hoc from (a) as an exact cut at B conflicts; (c) k in {2,4,8} random
  parities over the 3l coordinate bits, Glucose.
- Baselines from MITM ground truth on curve points (counts, not timed): linear 2+1
  scan with early exit (expected leaf solves), enumeration of ordered (x1,x2) with
  early exit (expected pairs tested).
- Targets: random rational points, seeds 1000003*n + 1009*l + i. 24/cell (n=7),
  16/cell (n=11; l=6 only 13, run stopped at the time budget), 8/cell (n=13,
  l=3..5 only; l=6,7 not run).
- Raw rows: `rows_n{7,11,13}.jsonl`, also inside `results.json` (`rows`), cell
  summaries in `results.json` (`cells`, `per_n`), full table in `analysis.txt`.

## Validation

- Field/curve self-tests pass for n = 7, 11, 13, 17, 19 (`validation.json`).
- Planted targets: 60/60 SAT and lifted to rational points with +-P1+-P2+-P3 = +-R.
  One planted instance (n=7, l=3) first returned a spurious model of a known class:
  x1 = x2 = 1 (twist-type point), t = 0, i.e. R = P3 + (0,1). Blocking t = 0 gave a
  model that lifts. In the measured rows this class appears once (n=7, l=3,
  `sat_not_lifted` = 1); a SAT row that does not lift is not counted as a relation.
- Brute force over all x-triples and all t at n=11, l=3: 12/12 targets agree with the
  SAT answer and the MITM ground truth, but all 12 were non-decomposable, so this
  checks the UNSAT side only.
- All measured cells: Glucose SAT/UNSAT equals ground truth on every uncensored
  target (0 mismatches in 194 uncensored targets of 197).

## Results (conflicts or baseline units per relation found)

`a` = Glucose untreated; `b*` = best post-hoc exact-cut budget (B chosen in-sample,
see caveats); `xor k` = arm (c); `lin` = linear-scan leaf solves; `enum` = pairs tested.
dec = fraction of targets decomposable; nsol = mean ordered solutions per target.

| n | l | targets | dec | nsol | a: median conf SAT / UNSAT | a | CaDiCaL | b* (B, relations) | xor2 | xor4 | xor8 | lin | enum | best SAT / lin | best SAT / enum |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 7 | 1 | 24 | 0 | 0 | - / 36.5 | - | - | - | - | - | - | - | - | - | - |
| 7 | 2 | 24 | 0.167 | 0.33 | 143 / 152.5 | 926 | 819 | 926 (256, 4) | - | 1638 | - | 22.1 | 86.4 | 37.1 | 9.48 |
| 7 | 3 | 24 | 0.958 | 12.6 | 67 / 1348 | 189.7 | 182.6 | 23.0 (4, 4) | 251 | 530 | 436 | 1.96 | 8.69 | 11.7 | 2.65 |
| 7 | 4 | 24 | 1.0 | 82.7 | 32 / - | 55.5 | - | 21.6 (8, 8) | 261 | 438 | 1552 | 1.35 | 3.90 | 16.0 | 5.54 |
| 11 | 2 | 16 | 0 | 0 | - / 950 | - | - | - | - | - | - | - | - | - | - |
| 11 | 3 | 16 | 0 | 0 | - / 3929 | - | - | - | - | - | - | - | - | - | - |
| 11 | 4 | 16 | 0.125 | 0.56 | 821 / 16258 | 111960 | 112578 | 7989 (1024, 2) | 219854 | 129185 | - | 117 | 1842 | 68.3 | 4.34 |
| 11 | 5 | 16 | 0.875 | 9.75 | 27905 / 207322 | 63257 | - | 1997 (128, 1) | 95569 | 133320 | 392507 | 11.0 | 253 | 181 | 7.89 |
| 11 | 6 | 13 | 1.0 | 95.5 | 14309 / - | 25147 | - | 821 (64, 1) | 18596 | 132224 | 99672 | 2.62 | 46.4 | 313 | 17.7 |
| 13 | 3 | 8 | 0 | 0 | - / 11784 | - | - | - | - | - | - | - | - | - | - |
| 13 | 4 | 8 | 0 | 0 | - / 44773 | - | - | - | - | - | - | - | - | - | - |
| 13 | 5 | 8 | 0.375 | 3.0 | 6265 / 359580 (3 of 5 UNSAT censored at 400k) | 664225 | 318900 | 1008 (128, 1) | 764637 | 333068 | - | 61.0 | 1835 | 16.5 | 0.549 |

Result lines:

- [pcc:unverified] In all 7 cells that produced a relation, the best SAT arm (minimum
  over untreated Glucose, untreated CaDiCaL, the in-sample-optimal idealised budget
  and the three streamlining levels) costs 11.7x to 313x the linear 2+1 scan count
  per relation at the same l. Untreated Glucose alone: 41x (n=7, l=4) to 10,900x
  (n=13, l=5) the linear-scan count.
- [pcc:unverified] Against plain enumeration with early exit the best SAT arm is
  2.6x to 17.7x above in 6 of 7 cells and 0.549x in one cell (n=13, l=5). That one
  value comes from a budget B = 128 chosen in-sample that keeps a single relation
  out of 8 targets; untreated Glucose in the same cell is 362x the enumeration count.
- [pcc:unverified] Conflicts per relation of untreated Glucose do fall as l rises
  through the solution-rich regime: fitted -2.03 bits per l at n=7 (l=2..4) and
  -1.08 at n=11 (l=4..6). The baselines fall as fast or faster on the same targets:
  linear scan -2.02 (n=7) and -2.74 (n=11); enumeration -2.24 and -2.66. The ratio
  SAT / linear scan therefore does not shrink with l (n=11: 957x, 5,730x, 9,600x
  for untreated Glucose at l=4,5,6).
- [pcc:unverified] Refutation (UNSAT, untreated Glucose, median conflicts) grows at
  +2.60 (n=7), +2.54 (n=11), +2.47 (n=13, l=5 biased low by censoring) bits per l,
  and at fixed l by about 0.7 to 0.8 bits per bit of n (l=3: 1348, 3929, 11784 at
  n=7, 11, 13; l=4: 16258, 44773 at n=11, 13). This chain encoding pays for the n free bits of t.
- [pcc:unverified] Budgets (idealised exact cut, B chosen in-sample): ratio to
  untreated Glucose per relation 0.12 (n=7, l=3), 0.39 (n=7, l=4), 0.071 (n=11, l=4),
  0.032 (n=11, l=5), 0.033 (n=11, l=6), 0.0015 (n=13, l=5); at n=11 and n=13 the
  optimum keeps only 1 or 2 relations, so these are upper bounds on the effect, not
  estimates. Real Glucose `conf_budget` through pysat overshoots the cut (checked at
  restarts): total conflicts 3.8x (B=1024), 28x (B=128), 32x (B=4) the exact-cut
  count on the checked cells (`budget_check.json`); runs are prefix-consistent 56/56.
- [pcc:unverified] XOR streamlining: conflicts per relation relative to untreated
  Glucose on the same targets was above 1 in 15 of 17 (cell, k) pairs with a
  relation (1.15x to 28x) and below 1 in two: 0.74x (n=11, l=6, k=2, 13 targets)
  and 0.50x (n=13, l=5, k=4, 8 targets, 2 relations). k=8 leaves
  6% to 54% of solution-rich targets satisfiable.
- [pcc:unverified] Model T(l) = 2^l * cost/relation + 3 * 2^{2l}, unit weights:
  minimising l is 4 (n=7) and 6 (n=11) for untreated Glucose, both the top of the
  sweep and above n/3 (2.33, 3.67); for the linear scan it is 2 (n=7) and 4 (n=11).
  T at the SAT optimum is 1,656 vs linear-scan minimum 136 (n=7), and 1.62e6 vs
  2,639 (n=11). n=13 has one cell with relations, so no optimum can be read.

## Verdict against the pre-written kill criterion

- Primary kill (K-lin): **met**. No cell has any SAT arm below the linear 2+1 scan
  count per relation; the smallest ratio is 11.7 (n=7, l=3, in-sample budget).
- Secondary (K-enum): SAT is above enumeration in 6 of 7 cells; the single cell
  below (0.549) rests on one relation and an in-sample budget. Per the protocol a
  result below K-enum but not K-lin counts as killed.
- P1 (SAT-optimal l above n/m): observed for n=7 and n=11, at the edge of the sweep,
  so the true optimum is not bracketed. P2 (gain over the balanced point growing
  with n): not supported; untreated Glucose balanced/optimum ratio is 16.7 (n=7,
  l=2) and 4.45 (n=11, l=4), and n=13 has no data above l=5.

## Caveats and deviations

- Sizes are far below the protocol's first draft (n=17, 19 dropped after timing
  probes; Amendment 1). n=13 has only l=3..5 with 8 targets and no relation below
  l=5; n=11, l=6 has 13 of 16 targets. Both runs were stopped at the time budget.
- The measured runs were launched in the same shell line as the validation script
  and started although its first version aborted on the spurious twist model; the
  encoding was not changed afterwards and validation was rerun to completion.
- The untreated encoding is weak: its refutation cost scales with n, not only with
  l. A t-free or Gauss-aware arm (CryptoMiniSat native XOR, WDSat) was not run
  (pycryptosat exposes no conflict counter). The kill verdict is for this encoding.
- Units: one conflict (15 to 180 propagations here) vs one leaf solve (a GF(2)
  linear solve) vs one tuple test are compared as raw counts.
- Budget B and the best arm are selected on the same targets they are scored on.
- Solver seeds: Glucose and CaDiCaL are deterministic here; variation is over
  targets only. 400,000-conflict cap: 3 censored runs (n=13, l=5), counted as cost
  without a relation.
- Session log: this pilot may write only inside this directory, so no entry was
  appended to `docs/session-logs/2026-11-14.md`; the content above is the entry.

## What a full study would need

1. A stronger untreated arm on the same instances: t eliminated or Gauss-aware XOR
   reasoning, with conflict counters, so refutation cost is 2^{c*l} and not tied to n.
2. Three or more n with the l sweep bracketing the model optimum (l up to about n/2),
   at least 32 targets per cell, and held-out targets for choosing B and k.
3. An exact-cut budget implemented in the solver, and restarts-with-new-target
   compared against it.
4. A weighted unit (bit operations per conflict, per leaf solve, per LA step) in
   place of unit weights, and the frozen-route runner instead of this container.

## Reproduce

    python3 validate.py validation.json
    python3 run.py 7 24 rows_n7.jsonl 1 2 3 4        # about 10 s
    python3 run.py 11 16 rows_n11.jsonl 2 3 4 5 6    # about 25 min
    python3 run.py 13 8 rows_n13.jsonl 3 4 5         # about 20 min
    python3 budget_check.py && python3 analyze.py
