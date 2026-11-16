# RESULT: LEAD-EXO-hashed-factor-base-tensor-ecdl

## Outcome
- Status: ran. Exit code 0. Wall 4 s (run.py elapsed 4.36 s). run.stderr empty.
- One run only, seed 20261110, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): slope s >= 0.25 and every median C >= 1. Falsified if C < 1 at 2^18 or s <= 0. Controls: rho slope |s| < 0.1; planted N^0.4 slope < 0.25.

Measured:
- IC median C (ops/sqrt(N)): 2^12 37.2, 2^14 85.0, 2^16 120.1, 2^18 357.7.
- Slope s = 0.515 (>= 0.25). All C >> 1. C at 2^18 = 357.7.
- Negative control (rho steps/sqrt(N)) slope = -0.026, |s| < 0.1: pass.
- Positive control (planted N^0.4) slope = 0.066 < 0.25: pass.
- A/A (rho second seed) medians: 0.55, 0.44, 1.00, 0.37 vs primary 0.73, 0.40, 0.31, 0.71. Spread up to about 3x at 2^16, so the noise floor is large, but the control slope stays flat.

Verdict: consistent_with_prediction. Controls passed.

## Reading
- The toy tests only the hashed-factor-base m=2 enumeration baseline. It does not implement tensor contraction or the Boolean encoding, and does not audit the Akshima et al. proof.
- The verdict field matches the numbers. Slope 0.515 is steeper than the 0.5 theory value, plausibly from the 3B^2 linear-algebra term and the B grid. Four points and 3 curves each give only a coarse slope.
- The rho A/A scatter (3x) shows medians over 3 curves are noisy. The IC slope is far from the thresholds, so this does not change the verdict.
- Nothing surprising. The baseline is far above sqrt(N) (C of 37 to 358) and gets worse with size.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction (weakly): the plain hashed-factor-base baseline shows no sub-sqrt flattening at toy size. This says nothing about the paper's tensor-contraction claim, which remains unaudited.

## Standing
[pcc:unverified], single local toy run, no independent replay.
