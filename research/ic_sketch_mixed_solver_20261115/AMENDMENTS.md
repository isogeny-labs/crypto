# Amendments (after the protocol was frozen in 735883bee6)

## A1 (2026-11-15, after Part A v1 aborted)

Part A v1 (`partA/v1-aborted.jsonl`, 827 of 1,600 records) stopped with a panic in
`mixed-peel` at |F| = 128, m = 3: a column's incidence list could hold the same row
twice after a cancelled entry was refilled, and the Markowitz loop then eliminated
that row twice. Before the panic, the same defect made some `mixed-peel` solves fail
and retry, so it read more rows than `incremental-gauss-full-rank` (302 against 229
rows at |F| = 128, m = 3); no wrong answer was returned (the surplus-row check held).

Two changes, both to the implementation, none to the predictions:
1. `solve_square`: the live rows of a column are de-duplicated.
2. `MatchGate`: columns visited by a failed augmenting search stay marked dead until
   the next augmentation (the epoch moves only on augmentation). v1 showed the gate
   charging about 1,300–28,000 visits a stream because each failed search re-explored
   the whole alternating tree; dead marks stay correct because an unmatched new row
   adds edges only at a vertex no alternating path enters, so failed searches between
   two augmentations cost O(edges) together.

The v1 records are kept as observed. Part A is re-run in full (v2) with the fixed
binary; the predictions are scored on v2 only, and v1's gate figures are reported
beside them as the cost of the unamortized gate.

## A2 (2026-11-15, after scoring Part A v2; before any `mixed-peel-lazy` data)

Part A v2 scored: P1 failed (the gate reads the same rows as `filter+wiedemann` on
98.75 % of streams and its visits raise row_ops), P2 held (100 %), P3 held only at
|F| ≤ 64 against `filter+wiedemann` and only at m = 3 against
`incremental-gauss-full-rank`, P4 held (no duplicates; Bloom pure overhead).

The `mixed-peel` breakdown showed its work is bookkeeping, not arithmetic: at m = 2 the
2-core has 16–24 columns, and the gate (one search a row) and full-sweep peeling
dominate; at m = 3, |F| = 512, an O(core²) pivot scan dominates. A new arm,
`mixed-peel-lazy`, keeps the algorithm and changes those data structures:
- the matching is built once, when every column is covered (coverage is necessary
  for full rank and costs one count a non-zero), then maintained incrementally;
- peeling runs from queues of weight-one rows and columns (IBLT decoding);
- Markowitz pivots come from a bucket queue keyed by exact live column weight.
Every queue operation, weight update and incidence read is charged in row_ops, and
split out as `mixed_bookkeeping` / `mixed_arithmetic`. `mixed-peel` is unchanged.

Predictions for the new arm (exploratory; scored separately from P1–P6):
- **Q1.** It reads exactly the rows of `incremental-gauss-full-rank` on every stream.
- **Q2.** Its median row_ops are below `mixed-peel`'s in all 10 cells.
- **Q3.** Its median row_ops are below the cheapest existing full-rank arm
  (min of `incremental-gauss-full-rank`, `filter+wiedemann`, `filter+lanczos`) in at
  least 6 of the 10 cells.
- **Q4.** Existing arms reproduce their v2 records exactly (determinism check).

Part A v3 runs all 11 arms; Part B specs gain the `mixed-peel-lazy` arm before any
Part B run.

## A3 (2026-11-16, after Part B on real curves; `mixed-peel*` defect)

Part B v1 (`sessions/aborted-A3/`; p24 complete with 352 records, p28 stopped by hand at
254) showed every `mixed-peel` variant **exhausting the 10^8-trial cap** on 11 of 32
p24 runs and 7–8 of 23 p28 runs, at structural full rank. Every other arm verified
32/32 (p24) and 23/23 (p28). Cause: real index-calculus rows are
`±x_i ± x_j + b·t = a`: the factor-base coefficients are ±1, not random. The
Schwartz–Zippel argument of the protocol (a full matching makes the matched square
block non-singular with probability ≥ 1 − cols/r) assumed random coefficients, and is
false here: an even cycle of ±1 rows in the column graph is singular whichever rows
the matching picks, and the retry cadence kept picking the same block. Part A's
synthetic streams used random coefficients, so they could not show it; P2 and Q1
were scored on that assumption and hold only for random-coefficient rows.

Change (implementation only; predictions unchanged): when the matched square block is
singular, `mixed-peel*` eliminates over **all stored rows** (same peeling and
Markowitz code, pivots chosen among every row), checks every row against the
solution, and stays in that mode. New unit test `mixed_peel_solves_signed_two_term_rows`.
Part A is re-run (v4; it must reproduce v3 exactly, since the fallback only fires on
a singular block) and Part B is re-run in full with the fixed binary. Part B v1 is kept
as the observed failure.
