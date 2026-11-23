# RESULT: LEAD-ICP-xline-symmetry-special-j

## Outcome
- status: ran. exit code 0. wall 52 s (run.status: exit=0 wall=52; run.py elapsed_s 51.83). One run, no retries.
- stderr empty (nothing to report).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): zero hit cells; consistent_with_prediction if zero hits and controls pass; falsified if hits in >=2 cells and >= half of all cells.
Measured (15 cells = 3 primes x d=8..12, 30 curves/arm, 3 xR per curve):
- Mean deficit is identical in every arm (genA, genB, j0, j1728, smallcoef) in every cell: d=8:1, 9:5, 10:15, 11:35, 12:70, at p=97, 109, 157.
- gen_sd = 0.0 in all cells, so every effect and z is 0; n_hit_cells = 0 (j0, j1728 and smallcoef all non-hits).
- Controls: A/A z = 0 everywhere (pass); null arm smallcoef not a hit (pass); positive control (f1, x1*f1) extra deficit = 65 > 0 (pass). controls_ok = true.
- Verdict (run.py): consistent_with_prediction.

## Reading
- The deficits 1,5,15,35,70 are C(d-4,4): they look like the generic count of Koszul syzygies (f2*f1 - f1*f2 times multipliers), identical for every curve. The deficit is fully determined by degree counts and has zero variance across 150 curves per prime, so no curve-dependent syzygy exists at these degrees.
- The verdict field matches the numbers. No discrepancy.
- Caveat on power: with zero variance, the threshold max(3 SE, 0.5) collapses to 0.5. The detector saw a forced 65-syzygy drop, but the generic arm has no spread, so any real extra drop of at least 1 would have registered. Power is limited mainly by scope (unfolded chained S_3 only, d<=12 near the Koszul degree), as the protocol states. It does not test folding, the digit S_4 system, larger m, or the truncated remainder of the recorded next step.
- Nothing surprising beyond the zero variance. Special-j automorphism structure does not show in the unfolded system's Macaulay ranks.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction (narrowly): the unfolded chained S_3 system gains nothing from j=0/1728 symmetry at these toy sizes; any benefit would have to come from folding by invariants, which this run did not test.

## Standing
[pcc:unverified], single local toy run, no independent replay.
