# RESULT: LEAD-STR-endo-scalar-action-generic

## Outcome
- status: ran to completion, single run, no retries. exit code 0, wall 1 s (run.status: exit=0 wall=1). run.stderr empty.
- N=131041, 300 paired trials per arm, seeds 1000+t (paired), 900000+t (A/A).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md sec 3): step_ratio within +-25% (relative) of 1/sqrt(k) for k=2,3,6; charged_cost_ratio(k=60) > 1.
Falsified if any k in {2,3,6} deviates by >25%, or charged_cost_ratio(k=60) <= 1. Inconclusive if controls fail.

Baseline mean steps 1013.7 (A/A ratio 1.0015, tol 0.139, pass).

| k | step_ratio | predicted 1/sqrt(k) | rel. deviation | charged ratio (k*step) |
|---|-----------|---------------------|----------------|------------------------|
| 2 | 0.4255 (se of mean 12.5 steps) | 0.7071 | -39.8% | 0.851 |
| 3 | 0.4101 | 0.5774 | -29.0% | 1.230 |
| 6 | 0.2633 | 0.4082 | -35.5% | 1.580 |
| 60 | 0.0899 | 0.1291 | -30.4% | 5.393 |

Controls: A/A passed. Positive control (k=2 step_ratio < 0.85) passed (0.4255). Null/negative control: k=60 charged ratio 5.39 > 1, so no charged gain (large_orbit_no_charged_gain = true). Controls are OK, so the result is not inconclusive.
Verdict (run.py): falsified, because unit_orbit_scaling_ok = false (all of k=2,3,6 are more than 25% below 1/sqrt(k)). The no-large-orbit-gain clause held.

## Reading
- The numbers match run.py's verdict field. The falsification comes only from the scaling clause, and the deviation is in the direction of more speedup than 1/sqrt(k), roughly 1/k^0.8 to 1/k, not less.
- This is almost certainly a toy artefact, not evidence of a real gain. Baseline mean is 1014 steps against about sqrt(pi*N/2) ~ 454 for an ideal rho walk, so the baseline is about 2.2x worse than ideal (restart on in-walk revisit, 24-jump walk, crude cycle handling). Orbit reduction apparently also makes fruitless cycles and revisits cheaper to hit or avoid, so the arms are not compared at a constant per-step efficiency. I did not diagnose this; it is a hypothesis.
- k=2 gives 0.85 charged ratio (<1) only because the toy charges k ops per step; negation canonicalisation in real groups is nearly free, so this is consistent with the known sqrt2 ceiling and is not a new improvement. It is a constant factor, and a relabelling of an already known effect.
- k=60 charged ratio of 5.4 confirms that in the toy a large orbit is not free. That is the cost assumption of the protocol itself, not a measurement.
- The toy does test the generic-group scaling claim, but with a poorly calibrated baseline, so the "falsified" label says the preregistered 25% band was too tight for this walk, not that the obstruction fails. Not a matched-rho calibration (no sec 5a band).

## Bearing on the sub-sqrt(p) question
constant_factor_only: the best observed gain (k=2, charged 0.85; k=3,6 charged above 1) is a constant-factor effect inside an abstract group with an uncalibrated baseline, and the recorded obstruction End(G)=Z/N (ceiling sqrt|Aut|) is neither refuted nor materially confirmed.

## Standing
[pcc:unverified], single local toy run, no independent replay.
