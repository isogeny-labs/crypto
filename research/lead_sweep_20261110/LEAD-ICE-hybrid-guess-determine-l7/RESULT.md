# RESULT: LEAD-ICE-hybrid-guess-determine-l7

## Outcome
- status: ran (single run, no retry). exit code 0. wall 1 s by run.status (run.py reports seconds=1.43; protocol estimate was 1-3 min). run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): k*(12) >= 1 and k*(14) >= 1, A/A k* within 1 of main; consistent_with_prediction if both hold and controls pass; falsified if k* = 0 at both n >= 12.
- Measured: k*(10)=4, k*(12)=4, k*(14)=4 (main); A/A k*(10,12,14)=4,4,4. Differences main vs A/A: 0.
- Median cost (2^k * cols^2.37) at n=12: k=0 inf, k=1 6.58e6, k=2 5.40e6, k=3 4.08e6, k=4 7.40e5. At n=14: k=0 inf, k=1 inf, k=2 2.98e7, k=3 2.63e7, k=4 2.16e7. Pure solving (k=0) never solved at D<=4 for any n in the median.
- Controls: positive (linear, D=2) passed; negative (underdetermined) unsolved for all seeds, passed; A/A within 1, passed. run.py verdict: consistent_with_prediction.

## Reading
- Verdict matches the numbers as preregistered, but the support is weak:
  - k* sits on the upper edge of the grid (k=4, max allowed) at every n, with cost still falling monotonically. No interior minimiser is observed, so "an optimum k* >= 1 exists" is not shown; the toy only shows that costs fall as k rises over 0..4.
  - Likely artefact: guessing k variables leaves n equations in n-k unknowns, so the system becomes increasingly overdetermined and XL succeeds at lower D. The 2^k charge (2^4 = 16) is small next to the drop in cols at these tiny n. A real descended system would not necessarily gain this way.
  - Heavy censoring: k=0 is infinite at all n, and medians are infinite for many cells (e.g. n=14 k=1; A/A n=12 k=1..3 is inf, while main n=12 k=1..3 is finite). Main and A/A therefore differ in cells but agree at k*=4. Several finite cells have 1 of 4 seeds censored.
  - The cost depends only on (n, k, D_min) because cols is deterministic, so identical medians across arms are expected and not independent confirmation.
  - Falsification criterion (k*=0) was weak: pure solving was never feasible at D<=4, so k*=0 was nearly impossible by construction.
- Does not test the lead's claim about the l=7 Semaev/Weil descent cells (E8 (3,22,8) not attempted); random quadratic stand-in only, XL not F4/F5.

## Bearing on the sub-sqrt(p) question
none: a generic random-system toy with edge-of-grid optimum and a modelling artefact says nothing about the descent cells or about beating rho.

## Standing
[pcc:unverified], single local toy run, no independent replay.
