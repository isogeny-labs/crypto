# RESULT: LEAD-RHO-ecc2k130-iteration-function

## Outcome
- status: ran (single run, no retries). exit code 0, wall 0 s by run.status (integer seconds; under 1 s as measured by date).
- run.stderr empty. RESULT.json is a single JSON object. n=262153, table 16, 1200 trials per arm.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): ratio of mean steps-to-first-revisit (class/plain) is 1/sqrt(m) within 15%
for m=2 (0.707) and m=8 (0.354), else falsified. Positive control: plain m=1 within 25% of sqrt(pi n/2);
A/A within 10% of 1; if either control fails, verdict inconclusive.

Measured:
- mean steps: m1 = 1132.5 (se 28.5), m2 = 48.8 (se 5.0), m8 = 135.7 (se 3.3).
- ratio to m1: m2 = 0.0431 (rel dev -0.939), m8 = 0.1198 (rel dev -0.661).
- Positive control: theory sqrt(pi n/2) = 641.7; measured 1132.5 is 1.76x theory (+76%). FAILED (limit 25%).
- A/A ratio 0.988, within 10%. PASSED.
- Null control: m1 ratio 1 by definition.
- Verdict (run.py): inconclusive. controls_ok = false (positive control failed).

## Reading
- The verdict field matches the numbers: positive control fails, so the rule forces inconclusive.
- Why the positive control fails: a 16-entry table walk on Z_n has a large fruitless-cycle/short-cycle
  excess even for m=1. The mean is about 1.76x the random-mapping value. That is a defect of the
  random-mapping baseline for this walk, not a detector defect, but it means the calibration premise failed.
- The m2 and m8 ratios (0.043, 0.120) are far below 1/sqrt(m). They would be "falsified" if the controls had
  passed, but the toy cannot be trusted to say so. The class arms are probably dominated by degenerate short
  cycles: in Z_n the multiplicative map x -> g^k x with canon = min over the orbit makes the step map
  collapse onto few classes (m=2 is sign, orbit {x, -x}; x=0 and small values are absorbing-like).
  m2 mean of 49 steps against theory of order 450 is strongly suggestive of that.
- The toy uses additive translation with a multiplicative automorphism, which does not commute, so it is not
  the Frobenius/negation structure of a Koblitz curve (the protocol already says this). It does not test the
  GPU rate comparison at all.
- Ratio comparison also depends on m1 being inflated; both numerator and denominator are affected by
  fruitless cycles, so the ratios are not interpretable here.
- Note: n=262153 was used (protocol text says n=262297 region); run.py unchanged, not an issue for the verdict.

## Bearing on the sub-sqrt(p) question
none: the run is inconclusive (failed positive control) and a Z_n toy cannot bear on sub-sqrt(p) behaviour
of an ECC2K-130 walk; only a constant-factor (cycle handling) caution is hinted, not established.

## Standing
[pcc:unverified], single local toy run, no independent replay.
