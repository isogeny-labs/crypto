# RESULT: LEAD-STR-isogeny-walk-weak-neighbour

## Outcome
- Status: ran to completion. Exit code 0, wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- One run only, no retries, run.py and PROTOCOL.md untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): (a) 0 order or flag mismatches over all walk curves; (b) rho ratio neighbour/root in [0.8, 1.25]; twist positive control differs in >= 80% of roots; A/A ratio also in [0.8, 1.25]. Falsified on any mismatch.

Measured (RESULT.json):
- Walk curves: 176 over 3 primes (12 roots each, depth 8). order_mismatch = 0, weak_flag_mismatch = 0.
- Positive control (twists): order differs in 36/36 roots (100%, threshold 80%): passed.
- Null (random curves, same order as E0): 0/36: as expected, reported not gated.
- Rho (p=4091, 900 trials per arm): mean steps root 23.15, neighbour 23.89, root A/A 23.98.
  Ratio neighbour/root = 1.032; A/A ratio = 1.036. Both inside [0.8, 1.25].
- Verdict: consistent_with_prediction. Controls passed: positive yes, null yes, A/A yes.

## Reading
- The toy tests the order/flag half of the claim directly, but equal order under rational 2-isogeny is a theorem (Tate), so this mostly checks the Velu implementation, which passed.
- Discrepancies with the protocol text, none affecting the verdict:
  - Primes used are 4091, 16363, 65519, not the 16381 and 65521 named in PROTOCOL.md s4 (run.py picked primes downward; 16381 and 65521 are not both p = 3 mod 4 primes of the stated form, or the search skipped them). Not investigated, by rule.
  - 176 walk curves rather than 3*12*8 = 288: some steps evidently ended without a rational 2-torsion root. Not inspected beyond this count.
  - "rho_trials_per_arm" is 900 (6 curves x 150), matching the protocol.
- Rho mean steps near 23 reflects small subgroup orders (q >= 300); the ratio has sampling noise and the band is coarse, not a TOST certificate. The A/A ratio (1.036) is about the same size as the neighbour effect (1.032), so the rho comparison cannot distinguish them from noise. That is the expected outcome under the null.
- Nothing surprising.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: order and order-derived flags are invariant along rational 2-isogeny walks at toy sizes, and rho cost is unchanged within noise, matching the atlas closure (weaknesses are class invariants).

## Standing
[pcc:unverified], single local toy run, no independent replay.
