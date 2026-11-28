# Results: eliminated S4 against chained S3 as the m = 3 oracle

Registered in `PREREGISTRATION.md` (commit dba1211d3, before any registered
cell ran). Run id `run-20261018T2059Z` (queue 20:59:58-21:06:55 UTC); records in
`runs/cell-*.jsonl`, scored by `examples/s4_oracle_cost_score.rs`
(`score-output.txt`), report in `reports/run-20261018T2059Z/`. Engine
`examples/s4_oracle_cost.rs`, 1 thread, `prlimit --as=3000000000`,
`F4_F2_MAX_MATRIX_WORDS = 2^26`, 600 s budget. Wall time is a practicality
note only: the container was shared (load average 1.2-4.2), no isolation tier
was earned.

## Table (4 draws per cell, same basis and x_R on both arms; medians over verified draws)

| (n,l) | arm | verified/run | solving degree | median word_xors | largest matrix | f4 s |
|---|---|--:|---|--:|---|--:|
| (12,3) | chained | 4/4 | 3,4,4,4 | 1.98e6 | 3249 x 3088 | 0.03 |
| (12,3) | eliminated | 4/4 | 7,7,7,7 | 3.62e5 | 240 x 478 | 0.01 |
| (13,4) | chained | 3/4 (1 oversize) | 4,4,4 | 2.59e8 | 14994 x 14833 | 1.4 |
| (13,4) | eliminated | 4/4 | 7,7,7,7 | 5.40e7 | 3104 x 2426 | 0.16 |
| (16,5) | chained | 2/4 (2 oversize) | 4,4 | 2.46e10 | 42644 x 35501 | 35 |
| (16,5) | eliminated | 4/4 | 7,7,7,7 | 2.32e10 | 39487 x 16382 | 13 |
| (18,6) | chained | 0/2 (both oversize) | unmeasured | | | |
| (18,6) | eliminated | 0/2 (rc 134, allocation failed under the 3 GB cap) | unmeasured | | | |

(21,7) and (24,8) were not run: the stop rule fired at (18,6). Every finished
record is `verified` (solution sets equal E for the eliminated arm and E_rat
for the chained arm, every solution passes `binary_semaev_s4`, extraction
complete). A/A at (13,4) draw 0: word_xors identical in both arms (spread 0);
wall 1.43 vs 1.71 s (chained), 0.093 vs 0.105 s (eliminated).

## Registered verdicts
- (12,3): **stage improvement**, 4 paired draws, chained/eliminated median word_xors
  5.5x (w=0), 5.4x (w=0.1), 4.9x (w=1): the order holds across the 5a band.
- (13,4): **stage improvement**, 3 paired draws (draw 1 chained oversize), 6.4x, 6.4x,
  6.3x.
- (16,5): no verdict (2 paired verified draws, rule needs 3). Observation only: on the
  2 paired draws the medians are 2.46e10 (chained) and 2.32e10 (eliminated).
- (18,6): no verdict, both arms unmeasured.
- Trend: not determined (2 cells with a verdict; the rule needs 3).

## What the data say, and what they do not
- Per cell the gap shrinks 5.5x, 6.4x, then about 1.1x on the paired draws at (16,5).
  The chained draws that exceeded the 512 MiB matrix cap at (13,4), (16,5)
  and (18,6) are censored from above (they cost at least the work done before
  the cap), so the medians over measured draws favour the chained arm. The
  (16,5) paired draws are both random-target draws with no solution; the
  planted draws are the censored ones. A registered verdict at (16,5) needs a run with a
  higher cap; this study did not do one (the preregistration forbids rerunning a registered cell).
- The eliminated arm solves at degree 7 on every measured draw, the chained
  at 4 (3 once), as the two earlier studies hinted. The eliminated matrices are
  smaller in columns, but at (16,5) its largest matrix has 39,487 rows against
  42,644, so its row count stops falling away.
- The two oracles are not the same set: at (13,4) draw 1, E has 46 elements, E_rat 19. The
  eliminated system returns resultant zeros whose common root u lies outside
  F_{2^n}; the chained system returns only rational ones. The eliminated
  arm's answers need the extra rationality filter the chained one does not,
  which is outside this cost.
- Costs exclude equation construction beyond its term count (no counter): lower bounds.

## Prediction outcomes
P1 (degree 7 vs 4 where both finish): held, 85%. P2 (eliminated below chained
everywhere both finish, and at least 10x at (13,4)): the ordering held
where a verdict exists, the 10x clause failed (6.4x). P3 (chained unmeasured at (21,7),
(24,8); at (16,5)): (16,5) only half unmeasured, the larger cells not run, so it
was not tested. P4 (trend slope < 0): undetermined, 2 cells. P5 (all finished records
verified): held, 0 unverified of the finished. P6 (E larger than E_rat in a planted
draw): held (1 record, (13,4) draw 1). P7 (A/A spread 0): held.

## Success criterion
A stage improvement at (12,3) and (13,4): eliminated S4 costs 5-6x fewer counted
word XORs than chained S3 on the same instances, answers verified. It moves no
end-to-end S, and the distance to rho is unchanged. No external solver
comparison was made (AGENTS.md 15 owed). Next: rerun (16,5) and (18,6) with a
higher matrix cap and memory to resolve the chained censoring and the trend.
Single host class (shared x86-64 Linux container).

## Unmeasured
(16,5) chained draws 1 and 3 and (18,6) chained draws 0 and 1 (oversize);
(18,6) eliminated draws 0 and 1 (allocation failure under the 3 GB cap);
(21,7), (24,8) not run (stop rule).
