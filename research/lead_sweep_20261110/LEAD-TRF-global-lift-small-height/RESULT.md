# RESULT: LEAD-TRF-global-lift-small-height

## Outcome
- status: ran (single run, no retries). exit code 0, wall 5 s (run.status: exit=0 wall=5).
- stderr empty. RESULT.json is one JSON object.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): "slope <= -0.5. Falsified if slope > -0.5 with >= 10 events at H=128. Inconclusive if positive control < 0.95 or < 20 events at H=8, or too few nonzero points."
- Measured (n=4000 per H per seed, seeds 101 and 202, pooled 8000 per H):
  - dep/torsion counts: H=8: 369, 16: 106, 32: 35, 64: 6, 128: 2
  - rates: 0.046125, 0.01325, 0.004375, 0.00075, 0.00025
  - log-log slope = -1.920 (threshold -0.5)
- Controls:
  - positive (planted P2=kP1, k in 2..4): detect rate 1.0 (>= 0.95, passed).
  - A/A (two seeds, identical code): max per-H count difference 15, which is the noise floor. The run does not report which H or the per-seed counts.
  - null/negative: none beyond A/A (a stated limit of the protocol).
  - >= 20 events at H=8: yes (369). Nonzero points: all five H values.
- Verdict: consistent_with_prediction. Controls passed.

## Reading
- The numbers agree with run.py's verdict field. The slope is far below -0.5.
- Caveat on the tail: only 6 events at H=64 and 2 at H=128. The slope is dominated by H=8..32 and the last two points are Poisson-noisy (2 events is about +-70%). The falsification clause needs >= 10 events at H=128, so it could not have fired here. A shallower true tail slope is not excluded by this run, but the observed data lie far from -0.5.
- A/A difference of 15 on 369 events at the low end is about 4%, small relative to the trend. Where it occurs and its size at high H are not reported.
- The toy tests only its stated family: random integer pairs forced onto a Q-curve, with dependence tested for multiples <= 6. Larger relations are missed and bias rates down. Part of the steep decay is probably that most dependent or torsion cases at small H come from degenerate coincidences (for example small coordinates, repeated x or y), which are rarer at large H.
- No reduction mod p, no selected-lift or height bound, no number-field (SNFS/Solinas) variant. This is not a test of a prime-field lift probability exponent.
- Nothing surprising otherwise.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: for this toy xedni-style Q-lift family, dependence decays at slope about -1.9, well past the alpha >= 1/2 bound, so it offers no route to alpha < 1/2. This matches the recorded obstruction and says nothing about other lift families.

## Standing
[pcc:unverified], single local toy run, no independent replay.
