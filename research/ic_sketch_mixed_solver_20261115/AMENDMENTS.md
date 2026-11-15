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
