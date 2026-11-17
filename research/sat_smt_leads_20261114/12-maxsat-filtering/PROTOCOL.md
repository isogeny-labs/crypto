# Idea 12 pilot protocol: MaxSAT for relation filtering (written before any measured run)

Date: 2026-11-14. Pilot only, toy sizes, one core, not claim data. Every number is `[pcc:unverified]`.

## Problem posed

Index-calculus relation matrix A: R rows (relations), N columns (factor-base elements),
each row has exactly w = m+1 = 4 nonzeros (m = 3). For ECDLP one left-kernel vector suffices,
so filtering must hand linear algebra a sub-matrix with structural excess
`rows - nonzero columns >= k`, k = 1 (this guarantees a non-trivial left kernel over any field).

Selection problem (SEL): choose a row subset S, with C(S) the set of columns it touches, such that
`|S| - |C(S)| >= k`, minimising `|C(S)|`. Since every row has weight 4 and an optimal S can be
trimmed to `|C|+k` rows, the modelled Wiedemann/Lanczos cost after selection is
`cost_sel = |C| * 4 * (|C| + k)`, monotone in `|C|`.

After selection, the SAME greedy Markowitz structured Gaussian elimination (SGE; structural fill-in,
no cancellation except the pivot column, as over a large prime field) is applied to every arm's
output; `cost_sge = min over the merge trajectory of N' * weight'`.

Merge order / fill-in is NOT put in the MaxSAT model in this pilot (not covered); only the subset
selection is. One variable per comparison: the selection method.

## Instance generator

Matched random model (NOT real decompositions; said explicitly): each row is a uniformly random
4-subset of the N columns, `random.Random(seed)` with seed = 1000*log2(N) + 10*excess_pct + rep.
Justification: PDP relations of a random target over a subspace factor base have no smoothness
skew; column distribution is near uniform. A real-curve check of that assumption is not part of
this pilot.

Sizes: N in {32, 64, 128, 256, 512, 1024} with all arms; N in {2048, 4096} baseline arms only.
Excess (R = ceil((1+e)N)): e in {5%, 20%, 100%}. (50% dropped for budget; stated up front.)
Reps: N=32: 6, N=64: 4, N=128: 4, N=256: 3, N=512: 3, N=1024: 2, N=2048/4096: 1.

## Arms (same instances, run back to back per instance, order G1, G2, G3, RC2, ANY)

Baselines:
- G0: iterated singleton removal only (reference point).
- G1: singleton removal + greedy excess removal, delete the row with largest
  score sum_c 1/(w_c - 1) (weight-2-column / clique-style score), cascade singletons, until excess = k.
- G2: same, but with cascade lookahead: among candidates (all rows if R <= 300, else top 48 by G1
  score) simulate the singleton cascade and delete the row whose cascade removes the most columns
  per unit of excess spent (free cascades first).
- G3: equal-wall-budget randomised restarts of G2 (multiplicative noise on the score, candidate set 16),
  budget T = 5 s, best kept. This is the strongest cheap baseline and the one the MaxSAT arms are judged against
  ("greedy pipeline" = min(G1, G2, G3) columns, then SGE).

Treatment:
- RC2: exact weighted partial MaxSAT, pysat RC2 (Glucose3 oracle defaults). Variables x_r (row kept),
  y_c (column kept). Hard: x_r -> y_c for c in r; one cardinality constraint
  `sum(not x_r) + sum(y_c) <= R - k` (pysat CardEnc, cardinality network). Soft: (not y_c), weight 1.
  Wall limit T = 5 s in a forked child; result = optimum or TIMEOUT.
- ANY: anytime model-improving SAT, Glucose4, same hard clauses plus `sum y_c <= B` (cardinality
  network), B starting at (best greedy columns - 1), phases initialised to the best greedy solution,
  B lowered to (model columns - 1) on each SAT answer; stops at UNSAT (optimum proven) or wall T = 5 s.
  Result = best columns found (falls back to greedy if no improving model).

Both treatment arms run on the G0-reduced (singleton-free) matrix, which is optimum-preserving.

## Metrics

Primary (solver independent): columns |C| after selection; cost_sel; cost_sge; ratio treatment / greedy pipeline.
Solver units: SAT conflicts and decisions (oracle accum_stats), number of greedy restarts in G3.
Secondary: wall seconds. Also: largest N at which RC2 / ANY proves optimality within T, per excess.
Slope: log2(RC2 conflicts) against log2(N) on cells where all reps solved (>= 3 sizes needed, else "no fit").

## Validation before measurement

1. Brute force: N=10..12, R=14..18 tiny instances, enumerate all column subsets, compare the optimum
   with RC2 and ANY (20 instances).
2. Planted: a dense block (12 rows inside 9 columns) planted in a sparse 60-column matrix; the
   MaxSAT arms must return <= 9 columns and the returned rows must lie inside the returned columns
   with excess >= 1.
3. Every arm's output on every instance is checked by an independent verifier (rows inside columns, excess >= k).

## Prediction

Random 4-uniform hypergraphs have no small dense sub-structures, so the optimum is a constant
fraction of N and the greedy cascade is near-optimal: predicted MaxSAT column reduction over G3
below 5% wherever it terminates, and RC2 exact solving stops (5 s) at N <= 128.

## Kill criterion (from the task card, frozen)

Killed if EITHER
(a) cost reduction (cost_sge, and cost_sel reported beside it) of the best MaxSAT arm over the greedy
    pipeline is under 5% at every size N, OR
(b) exact solving (RC2 or ANY proving optimality within T) stops before N = 2^10 AND the anytime arm
    shows no gain >= 5% over the greedy pipeline at the sizes where exact solving failed.
Alive only if neither holds. Inconclusive if the budget prevents evaluating either.

## Budget

Timeouts T = 5 s per arm (G3, RC2, ANY); worst case about 66 instances * ~17 s. Single process.
Deviations from this protocol will be listed in README.md.

## Deviation D1 (recorded after validation + two smoke instances, before the measured run)

Smoke runs showed `cost_sge = min over the whole merge trajectory of N' * weight'` is degenerate:
with merge work uncounted, full elimination to one column always wins (cost 2). Replaced, once, by the
usual density stop: SGE merges while the average row weight stays <= 16 (4x the input row weight) and
cost_sge is the minimum of N' * weight' over that prefix. No other change. Smoke also showed
`EncType.cardnetwork` is spelled `cardnetwrk` in pysat 1.9.dev22 (same encoding).
