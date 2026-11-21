# LEAD-TRF-global-lift-small-height

## 1. Lead as recorded
- disposition: open_next_step (disposition_20261101: closed_known); family: transfer; last_reviewed 2026-10-29.
- title: Global lifts to E(Q) or E(K) with small canonical height (xedni, SNFS / cyclotomic / Solinas number fields, rational 2-torsion lifts).
- next-step excerpt (truncated): "one explicit prime-field lift family either composes the selected-lift, minimal-discriminant hei..."
- Context: desk note says Q-lift xedni is closed by the literature (exponent 3 for the named family); a viable source needs lift-probability exponent alpha < 1/2 (function-field sibling has alpha = 3).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 25, 266); research/ecdlp_subexp_desk_20261101/README.md (lines 10-13, 61, 80, 108).

## 2. Hypothesis
For a toy xedni-style lift family (two integer points with coordinates bounded by H, the curve y^2=x^3+Ax+B over Q forced through both), the probability that the lifts are dependent (a*P1 = +-b*P2 with 1<=a,b<=6, or either point torsion of order <=6) decays as H^-alpha with alpha >= 1/2. I.e. this family does not reach the alpha < 1/2 needed for a lift to beat sqrt(p).

## 3. Preregistered prediction and falsification criterion
- Statistic: least-squares log-log slope of dependence rate vs H over H in {8,16,32,64,128}.
- Prediction: slope <= -0.5. Falsified if slope > -0.5 with >= 10 events at H=128. Inconclusive if positive control < 0.95 or < 20 events at H=8, or too few nonzero points.
- Positive control: curves with planted P2 = k*P1 (k in 2..4); detector must flag >= 95%.
- A/A arm: two independent seeds (101, 202) with identical code; max per-H count difference reported as noise floor.
- Negative control: no clean null population exists in this toy beyond the A/A arm; dependence is counted only against the explicit bounded-multiple test, so the null is the high-H tail where genuine dependence should vanish. Stated as a limit.

## 4. Method
Pure-Python Fractions. Sample x1,x2 uniform in [-H,H], y1,y2 in [1,H]; solve A,B in Q; reject singular. Compute multiples 1..6 of each point by exact addition; classify torsion / dep / indep. n=4000 per H per seed, seeds 101, 202 (sample seed = seed*1000+H); positive control seed 7, 400 curves, H=10. Estimated cost: well under 2 minutes, < 50 MB.

## 5. Classification
runnable_local: a stdlib toy decides the slope question for this family (smoke run took 0.1 s).

## 6. What a run cannot establish
- Not a prime-field lift: no reduction mod p, no selected-lift or minimal-discriminant height bound, no number-field (SNFS/Solinas) variant.
- Dependence test limited to multiples <= 6; larger relations are missed, biasing rates down.
- Toy H up to 128 only; slope is not an asymptotic exponent.
- Says nothing about ECDLP cost; not a replay-certified or rho-matched result. All results [pcc:unverified].
