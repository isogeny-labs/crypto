# RESULT: LEAD-ICP-lattice-ic-relation-construction

## Outcome
- status: ran; exit code 0; wall 3 s (run.status: exit=0 wall=3); run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): "Consistent: ratio >= 1 at every size and slope >= 0.05. Falsified: any size with median ratio < 1. Inconclusive: controls fail, A/A differs by > 25 %, or slope < 0.05."

Measured median ops / sqrt(pi n/4), arm A (A/A, null in brackets):
- 2^12: 13.79 (14.05, 13.55)
- 2^14: 16.65 (17.03, 16.82)
- 2^16: 20.99 (20.65, 20.81)
- 2^18: 26.29 (25.86, 26.28)
- 2^20: 32.54 (32.85, 32.75)
- 2^22: 40.86 (41.02, 40.68)
Slope of log2 ratio vs log2 n = 0.1585 (generic model ~1/6 = 0.167).

Controls: positive 30/30 at every size, all relations valid, 0 censored; A/A relative difference at most 2.2 % (limit 25 %); null equals arm A within noise.
Verdict: consistent_with_prediction. Controls passed.

## Reading
- The toy tests only the generic pair-collision baseline for collecting m=2(8n)^(1/3) four-term relations. It does not implement any lattice, quaternion or isogeny constructor, nor elliptic curves.
- run.py's verdict field matches the numbers: ratio far above 1 everywhere, slope 0.158 is above 0.05.
- Null ratio equals arm A, as expected for a generic group where elements carry no exploitable structure.
- Relations are not rank-checked; the ratio is 14x to 41x of rho and growing.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction (for the generic baseline only): an oracle-free generic relation constructor costs more than rho and the gap grows about n^(1/6); any lattice method needs a non-generic handle.

## Standing
[pcc:unverified], single local toy run, no independent replay.
