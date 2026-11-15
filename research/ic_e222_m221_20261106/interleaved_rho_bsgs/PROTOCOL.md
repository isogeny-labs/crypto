# Protocol: interleaved multi-target rho vs folded multi-target BSGS on j=0 curves (2026-11-14)

Frozen before any row of this study. Owner request: "run the interleaved rho vs BSGS comparison".

## Question
On the same j=0 curves and planted targets, which takes less wall time: the batched folded multi-target rho (Kuhn-Struik restarts, dp rule `DPRULE=ks DPDIV=1.5`, fixed in PR #4144 on held-out targets) or the folded multi-target BSGS (`MC=1`, tuned by its author)? The earlier comparison put them in separate, non-interleaved runs.

## Arms
- `rho`: `examples/prime_fast_j0_rho_multi.rs`, `DPRULE=ks DPDIV=1.5`, 128 lanes.
- `bsgs`: `examples/prime_fast_j0_bsgs_multi.rs`, `MC=1`.
- `rho_aa`: the `rho` arm run a second time on the same targets in the same round (A/A noise floor).
No arm is retuned during the study.

## Design
- Cells: 26 bits T = 20, 200, 1000; 28 and 30 bits T = 200, 1000, 3000.
- Rounds r = 0..5 per cell. Round r uses planted targets `INST0 = 100 + r`, 2 instances x 2 curves (4 problems), shared by all three arms. These targets were used by no earlier run.
- Arm order within a round rotates through the six permutations of (rho, bsgs, rho_aa), one per round.
- Cells run in the order above; one process per arm per round; single thread; local 4-core container, not isolated.
- Every row must recover all T logs (checked against the planted values). A row that does not is reported and the round is excluded from the ratio, not rerun.

## Metric and analysis
- Per round and arm: the sum over its 4 problems of wall ms.
- Per cell: ratio rho/bsgs per round; report the geometric mean over the 6 rounds with a 95% percentile bootstrap interval (10,000 resamples of rounds, seed 20261114), and the same for rho_aa/rho.
- Reading: rho faster if the interval for rho/bsgs lies below 1, BSGS faster if above 1, otherwise unresolved. The A/A interval must contain 1; if it does not at a cell, that cell's verdict is reported as unreliable.
- Also reported: rho steps / sqrt(2nT/6) and BSGS giant steps, medians.

## Not in scope
§5a band (both arms are counted in wall time only), isolated host, larger sizes.

## Addendum A (2026-11-14, frozen before any 32/34-bit row): 32 and 34 bits
Owner request: "run it at 32 and 34 bits". Same arms, settings (rho `DPRULE=ks DPDIV=1.5`, BSGS `MC=1`), rounds (r = 0..5, `INST0 = 100 + r`, 2 curves x 2 instances), order rotation, metric, bootstrap and decision rule as above, on cells 32 and 34 bits x T = 200, 1000, 3000 (`run.sh "32 200" "32 1000" "32 3000" "34 200" "34 1000" "34 3000"`). No retuning. Reported alongside the first nine cells, not pooled with them.
Also recorded per row: BSGS table size `m` (its memory grows as sqrt(Tn)), to read any change in the ratio against it.

## Addendum B (2026-11-15, frozen before any 36-bit row): 36 bits
Owner request: "run it at 36 bits". Same arms, settings (rho `DPRULE=ks DPDIV=1.5`, BSGS `MC=1`), rounds (r = 0..5, `INST0 = 100 + r`, 2 curves x 2 instances), order rotation, metric, bootstrap and decision rule as above, on cells 36 bits x T = 200, 1000, 3000 (`run.sh "36 200" "36 1000" "36 3000"`), same binaries as addendum A (source unchanged since `a8cc24a6fe`). No retuning. Reported alongside the earlier cells, not pooled. BSGS table size `m` recorded as in addendum A.

## Addendum C (2026-11-15, frozen before any 38-bit row): 38 bits
Owner request: "run it at 38 bits". Same arms, settings (rho `DPRULE=ks DPDIV=1.5`, BSGS `MC=1`), rounds (r = 0..5, `INST0 = 100 + r`, 2 curves x 2 instances), order rotation, metric, bootstrap and decision rule as above, on cells 38 bits x T = 200, 1000, 3000 (`run.sh "38 200" "38 1000" "38 3000"`), same binaries (source unchanged since `a8cc24a6fe`). No retuning. Reported alongside the earlier cells, not pooled. BSGS table size `m` recorded.

## Addendum D (2026-11-15, frozen before any 40-bit row): 40 bits
Owner request: "run it at 40 bits". Same arms, settings (rho `DPRULE=ks DPDIV=1.5`, BSGS `MC=1`), rounds (r = 0..5, `INST0 = 100 + r`, 2 curves x 2 instances), order rotation, metric, bootstrap and decision rule as above, on cells 40 bits x T = 200, 1000, 3000 (`run.sh "40 200" "40 1000" "40 3000"`), same binaries (source unchanged since `a8cc24a6fe`). No retuning. Reported alongside the earlier cells, not pooled. BSGS table size `m` recorded (up to about 23M entries, 2^26 slots, about 0.9 GB at T = 3000).
