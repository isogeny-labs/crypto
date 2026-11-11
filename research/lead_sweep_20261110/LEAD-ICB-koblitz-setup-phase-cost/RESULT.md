# RESULT: LEAD-ICB-koblitz-setup-phase-cost

## Outcome
- status: ran (single run, no retry). exit code 0, wall 1 s (run.status: exit=0 wall=1). run.stderr empty.
- RESULT.json: one object, top-level verdict "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): consistent if every (n,w) cell median f >= 0.95 and K* >= 10, all controls pass,
A/A |diff| <= 0.02; falsified if any cell median f < 0.90; inconclusive otherwise, including any failed control.

Measured (medians over seeds 101..105, computed from the per-trial rows; run.py gave by_weight only for n = 13):
- n=11 (l=5): f(w=0) ~ 0.902, f(0.1) ~ 0.912, f(1) ~ 0.951; K* median at w=0 ~ 9.25 (<10), w=0.1 ~ 10.4, w=1 ~ 19.4.
  Seed range of f(w=0): 0.898..0.912, so f < 0.95 at w=0 and 0.1 and K* < 10 at w=0.
- n=13 (l=6): f = 0.981 / 0.982 / 0.990; K* = 53 / 58 / 97. A/A diffs 0.0014 / 0.0017 / 0.0005 (pass).
- n=17 (l=8): f ~ 0.997 / 0.997 / 0.9986 (all seeds 0.9969..0.9986); K* ~ 343 / 381 / 702.
- Positive control (every target log recovered): passed in all cells (20/20 for every seed).
- Null control (corrupted log table makes >= half the targets fail): PASSED for n=13 only (16..20 of 20 wrong);
  FAILED for n=11 (0..7 of 20 wrong) and n=17 (0..1 of 20 wrong).
- A/A: n=13 passes by run.py; for n=11 and 17 run.py skipped the by-weight summary because controls failed. From the
  trial rows, A/A medians look close (n=11 w=0: seeds 201..205 median ~0.8996 vs main ~0.9024, diff ~0.003;
  n=17 diff < 0.001), but these are my hand reading, not run.py output.
- Verdict: inconclusive (run.py "controls_failed" for n=11 and n=17). Even ignoring the controls, the n=11 cell
  misses the consistent criterion (median f < 0.95 at w in {0, 0.1}; K* < 10 at w=0) but does not reach the
  falsified line (median f(w=0) ~ 0.902 >= 0.90, only just). So the prediction is not confirmed at the smallest size.

## Reading
- The toy does measure where the cost sits (setup vs online op counts), as the protocol says, and the share rises
  steeply with n: 0.90 -> 0.98 -> 0.997 at w=0, tracking the table size growth (sums table adds 182 / 763 / 14520
  against about 15-50 online adds per target). This is largely built in by the l = (n-1)//2 table, as PROTOCOL
  section 6 warns.
- The null control is badly designed: it corrupts a single log, so it is detected only when that log is used by the
  recovered target decompositions. With a factor base of 120 (n=17) and tiny online decompositions (~3 lookups),
  almost no targets touch the corrupted entry. The failed null control at n=11 and n=17 is therefore a defect of
  the control, not evidence that the pipeline is broken (the positive control passed everywhere). It nonetheless
  forces "inconclusive" under the frozen rule, and I did not change anything.
- n=13 is degenerate: r = 79 with cofactor h = 106, so the prime-order subgroup is tiny relative to the factor base
  (rank 22 over Z/79), which makes it less representative. n=11 has r = 991, n=17 has r = 65587.
- run.py's verdict field ("inconclusive") agrees with the numbers; its omission of by_weight for failed-control
  cells hides the n=11 shortfall, which I recomputed from trial rows.

## Bearing on the sub-sqrt(p) question
none: the run only shows that setup dominates a toy m=2 pipeline once the table is large (f >= 0.98 for n >= 13,
only ~0.90 at n=11); it implements no reduction and says nothing about n = 83 or a sub-sqrt(p) algorithm.

## Standing
[pcc:unverified], single local toy run, no independent replay.
