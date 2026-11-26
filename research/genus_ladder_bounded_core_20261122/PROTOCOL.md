# Weight-bounded core before Wiedemann (frozen protocol, 2026-11-22) [pcc:unverified]

Question. R10: the genus-3 double-large-prime arm's relation phase scales as p^1.31, its linear algebra (`la_mul`)
as p^1.50, equal to rho's. `la_mul ~ c n (nnz + n)` for an n-column merged core. In the R4 data the merged
system's columns grow as p^0.77 (core columns p^0.66, SGE merges a shrinking share) and its mean row weight
creeps up (about log p). Can a core whose mean row weight is held at a fixed target bring the `la_mul` exponent
below 1.5? Measurement of an additive option; every figure `[pcc:unverified]`.

## Construction (candidate (b), `genus_ladder.rs`: `select_bounded_rows`, `BoundedParams`, `run_ic_ext_bounded`)
Relations are collected from the unchanged random stream. At the rule's first stop (N0 relations) collection
continues to (1 + STEP) N0, then an attempt builds rows, peels, and runs the selection: the heaviest row (ties:
higher index) whose removal leaves every column it holds, the target column included, with at least COVER live
rows is dropped, until cols + margin(cols) rows remain (sorted once; equal to repeatedly taking the heaviest
eligible row, because eligibility only turns off). COVER = 1 then re-peels (R4's selection); COVER = 2 needs no
re-peel (no column falls to weight one). If the selected core's mean row weight exceeds the target W and
(level + 1) STEP <= EMAX, the attempt is abandoned (a probe) and relations go on to (1 + (level + 1) STEP) N0, so
the excess E is chosen per instance, the smallest multiple of STEP reaching W, or EMAX (then flagged `bd_capped`).
Otherwise SGE(8,3) and Wiedemann run as in the plain arm. Counting: every probe's graph, peel and selection work
is in `la` (adds only; the selection has no multiplications), so `la_mul` and FPM include all of it; comparisons
free. Each solve is certified by the existing `mul`/`mul_rtl` check. Defaults byte-identical (frozen-count tests).
Flag: `--bounded W[,STEP[,EMAX[,COVER]]]`, arms named `...+sge(8,3)+bd(W,STEP,EMAX,COVER)`.
Candidates (a) and (c) not built (at most two were allowed; (a)'s column-then-rows deletion spends the same row
surplus as (b) per nnz removed, judged not worth a second option).

## Run (`run_all.sh`, frozen with this file; `frozen.sha256`)
Genus 3, p = 32771 (scaling study's curves 8, 16, 31), 65537 (R2 curves 64, 86, 157), 131101 (2, 14, 21), 262147
(13, 19, 23); 3 curves x 2 seeds; arm c; per prime one invocation:
`--curve-seeds P=... --curves 3 --seeds 2 --arms c --sge 8,3 --excess 0.3,0.01 --bounded 8,0.1,3,2
--bounded 8,0.1,3,1 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --order-method bsgs1`.
Arms: unmerged, plain SGE(8,3) (the baseline), ex(0.3,0.01) (R4's comparison arm), bd(8) cover 2 (the task's
greedy "covered at least twice"; arm A), bd(8) cover 1 (R4's selection with adaptive E; arm B). Target W = 8,
STEP 0.1, EMAX 3.0. Both bd arms are scored; neither is primary. Analysis: `analyze.py`, layout `render.py`.

## Estimands
Cell mean = mean over curves of the mean over seeds. FPM3 = cantor_mul + 10 cantor_inv + rel_mul + 3 la_mul (the
in-situ kappa after R8). Weight after SGE = sge_nnz_out / sge_rows_out. Exponent = least squares of log cell mean
on log p over the four primes; interval = 95% percentile of 4000 hierarchical bootstrap draws (curves with
replacement, seeds within the curve, the same draw for every arm; seed 20261122).

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| B1 | every run certifies; plain and unmerged rows equal the earlier studies' rows (R2 for 65537+, the scaling study at 32771) field for field except wall time; instance lines equal | any run uncertified or any field differs |
| B2 | the new arm's mean row weight after SGE varies by <= 10 % across the four primes (max / min <= 1.10) | max / min > 1.10 |
| B3 | the new arm's `la_mul` exponent <= 1.42 with the bootstrap upper end < 1.47 (plain: about 1.50) | point > 1.42 or upper >= 1.47 |
| B4 | FPM3 of the new arm <= plain's at 262147 (cell means), and its FPM3 exponent below plain's | either fails |

## Disclosed before data: development pilot
A pilot (one binary, R4's curves 158, 217 at 65537 and 69 at 262147, none in the frozen set, 1 seed) was run to
check the code. Weights after the step, E reached and `la_mul` against plain: 65537: E = 0.3, la_mul 0.57-0.64 of
plain; 262147: E = 0.5, la_mul 0.49 of plain, FPM3 1.09-1.11 of plain (plain 3.109e9, bd cover 1 3.396e9, cover 2
3.459e9, ex(0.3) 3.145e9). Cover 2 had larger la_mul than cover 1 at equal E (3.36e7 vs 3.04e7 at 65537). So the pilot
already suggests B4 fails (the relation phase is about three quarters of FPM3), B2 and B3 may hold. Target and
STEP were not tuned beyond this check (W = 8 is the task's example; W = 7 in the pilot needed E = 0.9 and FPM3
1.29 of plain).

## Limits (declared)
3 curves per prime (6 runs): exponent intervals are narrow in counts but cover curve variation of only three
curves; counts, not wall time; one prime-by-prime shared 4-vCPU host; no isolated host, no replay, no
`VALIDATION.json`, no leaderboard row. Adaptive E uses the instance's own relation count (a data-dependent stopping
rule, charged). EMAX caps E at 3.
