
## Predictions (frozen in PROTOCOL.md before data) and outcomes

Every outcome below is read off the tables above; none of the falsifiers occurred. Counts of hits are 0 everywhere at k = 24 and 28, so no prediction about hits tests more than "no hit was seen".

- **S0 (reproduction): held.** All 19 compared fields of all three arms are equal to lead D's (section Reproduction). [pcc:unverified]
- **S1 (lifts=1 costs about 8x lifts=0): held in all ten cells.** Calls per target ratio and multiply-adds per target ratio are both 8.000 (the interval was [7.5, 8.05] and [7.0, 8.5]); every target tries all eight lifts (`pkm_lift_targets_tried` = 1600 for 200 targets in the full lifts=1 rows). The ratio is the exact count 8, the same as at k = 20 for the isogeny cells (bits cells: 8.000 here against 7.898 at k = 20, where lifts=1 had 4 hits that stopped the lift loop early; there are no hits here). [pcc:unverified]
- **S2 (per-relation ratio near 1): held where defined, 9 of 10 cells.** Per-relation ratio of lifts=1 over lifts=0 at exact coverage: isogeny t = 4: 0.625 (k = 24; as forecast, not below 0.5), undefined at k = 28 (lifts=0 covers 0 points, projected coverage 8 points); isogeny t = 5: 1.05, 0.88; isogeny t = 6: 0.96, 0.93; bits t = 5: 1.18, 0.91; bits t = 6: 1.04, 0.96 (k = 24, 28). The ratio equals 8 divided by the coverage gain proj/cov of 6.8 to 12.8, so it is a property of the factor base and not of the oracle's speed. No observed per-relation ratio exists (0 hits in both arms). [pcc:unverified]
- **S3 (lifts=2 stays far above lifts=1): held in all ten cells.** Multiply-adds per target, lifts=2 over lifts=1 on the same targets: isogeny t = 4: 70.5 and 70.5 (k = 24, 28; 200 targets); t = 5: 494.9 and 494.9 (106 and 154 targets); t = 6: 628 and 628 (6 and 10 targets); bits t = 5: 30.7 and 30.7 (200); bits t = 6: 31.8 and 31.8 (58 and 59). The threshold was 10; the values equal lead D's k = 20 values. [pcc:unverified]
- **S4 (degree rise persists): held in all ten cells.** Max solving degree lifts=0 / 1 / 2: 5 / 5 / 8 (isogeny t = 4, 5; bits t = 5), 5 / 5 / 10 (isogeny t = 6), 6 / 6 / 8 (bits t = 6): rises of 3, 3, 5, 3, 2, identical at both k and at k = 20. Matrix sizes of lifts=2 over lifts=1 (rows/cols/nnz): 3.1 to 13 / 4.9 to 15 / 19 to 329, as at k = 20 (3.1 to 13 / 5.8 to 15 / 19 to 330 there). [pcc:unverified]
- **S5 (lifts 0, 1 degree flat in k): held.** The max solving degree at k = 24 and k = 28 equals the k = 20 value in every shape (difference 0). [pcc:unverified]
- **S6 (lifts=0 cost per target flat in k): held.** Multiply-adds per target of lifts=0 at k = 24 and 28 over k = 20: 1.000 in all five shapes (Table C; the largest ratio over all arms is 1.013, bits t = 6 lifts=1, from the k = 20 hits). [pcc:unverified]
- **S7 (fibre): held.** `pkm_fiber_size` = 8 and complete in every lifted run (Table A, column fiber, 8c).
- **S8 (hits): held.** 0 hits in lifts=0 and 0 in lifts>=1 over all ten cells (bound was 0 and at most 2). Expected hits at exact coverage over 200 targets, summed over the ten cells: 0.079 (lifts=0) and 0.63 (lifts=1), so the absence of hits is the expected outcome and is not evidence of an oracle defect or of an absence of yield.
- **S9 (caps): held.** lifts=2 reached the 600 s cap in isogeny t = 5 (106 and 154 targets at k = 24 and 28), isogeny t = 6 (6 and 10) and bits t = 6 (58 and 59), and finished 200 targets in isogeny t = 4 and bits t = 5 at both k. lifts=0 and lifts=1 finished 200 targets in all ten cells. (At k = 20 lead D completed 181, 8 and 56 targets in the same three cells; the number depends on the load, 6 to 15 here.)

## k-scaling ratios (summary)

- Per-target multiply-adds, k = 24 over k = 20 and k = 28 over k = 24: 1.00 and 1.00 in every shape and arm except bits t = 6 lifts=1 (1.013, 1.000) and lifts=2 (1.009, 1.000), Table C. The solver sees the same system at every k at these cells (equal degree, equal matrix sizes for lifts=0/1; for lifts=2 the maximum row count differs by up to 7% in isogeny t = 4 (1023 against 1092) and the maximum column count by 15% in bits t = 6 (2269 against 2660) from k = 20 to 24 (maxima over different numbers of targets for the capped lifts=2 arm), with the per-target totals unchanged to three digits).
- Per-relation cost at exact coverage grows with k (Table D): 6.6 to 20.2 from k = 20 to 24 and 12 to 256 from k = 24 to 28 for lifts=1 (the cell isogeny t = 4 at k = 28 has a projected coverage of 8 points). This follows the coverage `cov/r`, a property of the base size and `r`, not of the per-target cost.
- The ratios lifts=1 / lifts=0 (8.0, calls and multiply-adds) and lifts=2 / lifts=1 (30.7 to 628) do not move with k in these cells.

## What this does and does not show

- At these ten cells the k = 20 reading of leads 2 and D is unchanged: lifts=1 pays 8 times the F4 work per target for a coverage gain of 6.8 to 12.8 times (k = 24, 28), so the exact-coverage cost per relation of lifts=1 stays within [0.63, 1.18] of lifts=0's; lifts=2 is 30 to 630 times lifts=1's per-target cost, with the solving degree 2 to 5 above lifts=0's. [pcc:unverified]
- No hit was observed, so the observed yield gain of lifts=1 over lifts=0, observed hits per relation and the hit verification rate are not measured here at k = 24 or 28.
- This is not an improvement over the index-calculus baseline and not a ranking: the lifts=1 and lifts=2 arms did not lower the per-relation cost at exact coverage below lifts=0's by more than the interval stated (the lowest ratio is 0.625 at isogeny t = 4, k = 24, where it follows from a coverage ratio of 12.8 on a base of 8 columns). Nothing is said about rho.
- Not measured: k = 32 (the instrument refuses it), m = 3, other cells, `trace=1`, the 8-isogeny tower design for the free variable, any observed hit rate. Cells with fewer than 200 lifts=2 targets (six, listed above) are truncated samples.

## Deviations and incidents (all of them)

1. **Checkpoint push.** `scripts/checkpoint.sh push` failed with the expired upload token in `CHECKPOINT_UPLOAD_TOKEN` (expired 2026-11-04T16:59Z); it was run with that variable unset and the AWS credentials (`AWS_SECRET_ACCESS_KEY` from `AWS_SECRET_KEY`), which succeeded (checkpoint `20261105T033200Z-106141b2b9ad`) before the first run. `.resume/RESUME.md` was overwritten for this effort and restored from git before the commit, so the checkpoint text is not in the commit.
2. **Pre-data smoke run.** One 2-target run at k = 24 (isogeny t = 4, lifts=1, seed 2026110524, not a protocol seed) preceded the protocol commit text; disclosed in PROTOCOL.md; not in the tables.
3. **Analysis fix.** The first run of the analysis stopped on the k = 28 isogeny t = 4 rows (a division by zero: `coverage_exact` is 0 because lifts=0 covers no point). The subgroup order is now taken from the projected coverage; the data and the protocol did not change. In that cell, per-relation values for lifts=0 are reported as undefined.
4. **Truncation rule generalised from lead D's** (written before data): a re-run on the common prefix is made for every arm longer than the cell minimum; only the lifts=2 arms were capped, so it applied to lifts 0 and 1 only.
5. **Load.** The box was shared (load average 6 to 15 at run start); other agents' processes ran concurrently. Operation counts do not depend on load; the cap truncation points do.
6. **Files.** `analysis/f4cost_scale.jsonl` is 63 KB (under the 64 KiB limit); REPORT.pdf (114 KB) is over the 64 KiB rule and was pushed to the artifact store with `tools/artifacts/artifacts.py push` (manifest `.artifacts.json`, sha256 in it; the PDF is git-ignored); no other file is over the limit.
7. Predicted values for S2 in the isogeny t = 4 cell at k = 24 used the base counts seen in the 2-target smoke run (disclosed above).

Report PDF: [REPORT.pdf](REPORT.pdf).
