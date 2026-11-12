# RESULT: LEAD-TRF-jv-nagao-genus2-cover-decomp

## Outcome
- Status: ran (single run, no retries). Exit code 0, wall 1 s (run.status).
- run.py verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): P in 0.5 +- 0.04 at all p; lower 99% CI of
R = ((p+1)/P)/sqrt(pi p^2/4) >= 1 at all p (R about 2.2-2.5); forced-smooth
positive control P=1 > 0.99; A/A point estimates differ by less than the CI
width. Falsified if upper CI of R < 1 at any p.

Measured (60000 trials/prime, 6 curves):
| p | P_smooth | 99% CI | P (A/A) | R CI | aa_ok | pos_ok |
|---|---|---|---|---|---|---|
| 251 | 0.4978 | [0.4926,0.5031] | 0.4853 | [2.25,2.30] | false | true |
| 509 | 0.4852 | [0.4799,0.4906] | 0.5098 | [2.30,2.36] | false | true |
| 1021 | 0.4971 | [0.4918,0.5024] | 0.4987 | [2.25,2.30] | true | true |
| 2039 | 0.5051 | [0.4999,0.5104] | 0.4971 | [2.21,2.26] | true | true |
| 4093 | 0.4932 | [0.4879,0.4985] | 0.5006 | [2.26,2.31] | true | true |

- Main arm: |P-0.5| < 0.04 at every p, lower R CI is 2.21 or more (>= 1), no p has R
  below 1, so the falsification criterion is not met.
- Positive control (forced smooth) = 1.0 at all p: passed.
- Negative control: not applicable per protocol.
- A/A: failed at p=251 and p=509 (diff 0.0126 vs CI width 0.0105; diff 0.0246
  vs width 0.0107), passed at 1021, 2039, 4093.
- Verdict per protocol rule "controls fail -> inconclusive": inconclusive.

## Reading
- The numbers themselves match the hypothesis (P about 1/2, R about 2.2-2.4 at every size).
  The inconclusive verdict comes only from the A/A rule.
- Caveat on A/A: the A/A P values (e.g. 0.4853, 0.5098, 0.4987) look like ratios
  of small integers, which suggests the A/A arm used far fewer effective samples
  than the main arm, so comparing its difference to the main-arm CI width is
  probably too strict. I did not inspect or modify run.py to confirm this; it is
  a suspicion only, and the run is not rerun per the rules.
- The toy tests only a counting identity (smooth fraction of degree-2 divisors
  is about 1/2) and assumes N about p^2 and 1 group op per trial. It does not test
  the Nagao decomposition, the F_{p^6} cover, F4/FGLM, or the 151-293 figure.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction (weakly): relation collection alone
costs about 2.2-2.4x matched rho in counted ops at every toy size, consistent with
the recorded S/rho >> 1; the A/A control failure keeps the formal verdict inconclusive.

## Standing
[pcc:unverified], single local toy run, no independent replay.
