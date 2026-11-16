# RESULT: LEAD-STR-isogeny-transport-preprocessing

## Outcome
- status: ran to completion. exit code 0. wall 36 s (run.status: exit=0 wall=36; run.py self-reported 36.26 s). run.stderr empty.
- One run only; PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): consistent if all rho_l > 1, min rho in (1.0, 2.0], max rho in [10, 25],
rho nondecreasing in l (tol 0.05), and both controls pass. Falsified if any rho_l <= 1 or the range falls outside
the bands with controls passing. Decision at I = 10.

Measured mean rho_l at I = 10 (I=0 and I=30 differ by <= 0.006):
l=3: 1.004, 5: 2.007, 7: 3.011, 11: 5.018, 13: 6.022, 17: 8.029, 19: 9.033, 23: 11.040, 29: 14.051, 31: 15.054.
- all rho_l > 1: yes. min = 1.004 in (1, 2]: yes. max = 15.054 in [10, 25]: yes.
- nondecreasing in l: yes (strictly increasing, about about l/3).
- Controls: positive (all transported x on E', injective on half the N-part, 20/20 rows) passed;
  negative (perturbed b' on-curve rate < 0.8, observed 0.42-0.57) passed; A/A max relative spread of rho = 0.0
  (counts are deterministic, so the two draws agree exactly).
- Verdict by the preregistered numbers: consistent_with_prediction.
- Verdict field in RESULT.json: "falsified" (see Reading; this is a discrepancy).

## Reading
- DISCREPANCY: run.py's verdict is "falsified", but every preregistered condition holds on the numbers. Cause found by
  reading run.py line 147: the monotonicity check is `r10[k] <= r10[j] + 0.05` over consecutive pairs (j earlier l,
  k later l). That tests rho NONINCREASING in l, the opposite of the protocol ("nondecreasing, tolerance 0.05").
  Since rho rises by 1 to 1.7 per step, the check fails and the code emits "falsified". This is a sign-error bug in
  the frozen script, not a refutation. Per rules the script was not fixed and not rerun; the structured output
  reports run.py's field as required, and this reading is stated here.
- The toy tests the cost primitive only: Velu x-evaluation (counted 768 M at l=3, growing
  linearly, with T inversions) vs T-1 affine additions (766 M, 255 I). Ratio is ~ l/3, close to 1.00 at l=3 and
  15.05 at l=31, within the recorded 1.5x-21x loss range. Transport never beats regeneration.
- Caveats: the cost model charges kernel setup nothing (favours transport) and still loses; the l=3 ratio of 1.004 is
  barely above 1 (just inside the preregistered (1, 2] band; a different counting convention could put it at or
  below 1, so "all > 1" at l=3 is fragile). Toy p = 40039 (~2^15.3); the larger-p behaviour, projective/batched
  variants, multi-target amortisation and the distinguished-point walk-table case are not tested (PROTOCOL.md s6).
  Two curve draws gave identical counts, so the A/A arm is uninformative beyond confirming determinism.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: table transport costs about l/3 times regeneration (1.0x to 15x here), so
no preprocessing gain appears from transporting x-tables across odd-l isogenies in this cost model.

## Standing
[pcc:unverified], single local toy run, no independent replay.
