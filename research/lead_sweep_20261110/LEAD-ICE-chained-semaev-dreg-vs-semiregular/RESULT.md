# RESULT: LEAD-ICE-chained-semaev-dreg-vs-semiregular

## Outcome
- Status: ran to completion. Exit code 0. Wall 4 s (run.status: exit=0 wall=4; run.py reports wall_s=3.87).
- run.stderr empty. Single run, no retries. PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): consistent_with_prediction if diff = mean r(null) - mean r(chain) at D=5 exceeds
thr = max(0.02, 3*|A/A difference|) AND the positive-control effect exceeds thr. Falsified if diff <= A/A noise
(with the positive control passing). Otherwise inconclusive.

Measured (K=14, GF(2^6), N=12):
- Mean r at D=3/4/5: chain 7.512 / 3.114 / 1.531; null 1.0 / 1.0 / 1.0; aa1 1.0; aa2 1.0; pos 0.9167 at every D.
- diff (null - chain) at D=5 = -0.531. A/A noise = 0.0. thr = 0.02. Positive-control effect = 0.0833 (> thr, detected).
- Chain instances inconsistent at D=5: 10 of 14.
- Verdict: falsified. Controls: positive control passed; A/A passed trivially (zero noise); null control produced r exactly 1.0.

## Reading
- The chained system does not fall below the semi-regular yardstick at toy scale. It lies far above it: r > 1 and falling with D (7.5, 3.1, 1.5).
- r > 1 is not a degree-fall signal. The prediction formula assumes M cubic equations (1+z)^N/(1+z^3)^M. The chain equations are S3 expanded to bit polynomials, so they have higher degree than 3. Their Macaulay rows span more than the cubic-system prediction, so the proxy mismatches the chain's shape. This is a limitation of the preregistered proxy, not evidence of a structural effect.
- The null arm has r = 1.0 exactly in all 14 instances, at every degree, and both A/A arms are identical. The null rank therefore equals pred exactly, so the A/A noise of 0.0 is a saturation artifact: the null saturates the prediction and cannot show variance. The positive control still dropped the ratio (0.9167), so the detector is not blind to a planted syzygy, but the 0.0 noise floor makes the 3x-noise term of the threshold vacuous (thr is the 0.02 floor).
- 10 of 14 chain instances are inconsistent (random b, R, no planted solution), which also distorts the rank.
- run.py's verdict field ("falsified") agrees with the numbers under the literal rule (diff <= noise with positive control passing). The sign of diff is the opposite of the predicted direction, but the shape mismatch means this is closer to "the proxy is not comparable across arms" than a clean negative. The protocol's own caveat holds: this is not the C1 item (n=15, D=6).

## Bearing on the sub-sqrt(p) question
none. A toy-scale rank comparison against a mismatched (cubic) semi-regular proxy shows no earlier degree fall for the chain and says nothing about ECDLP cost versus rho.

## Standing
[pcc:unverified], single local toy run, no independent replay.
