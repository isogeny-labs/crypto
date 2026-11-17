# LEAD-ICE-semaev-cover-monodromy

## 1. Lead as recorded
- Disposition: closed_obstructed (proof), last reviewed 2026-10-24. Family: ic-engine.
- Title: Semaev summation-cover monodromy / Frobenius cycle types as a relation-rate gate.
- next_step_excerpt in leads.json is empty. Atlas survey line: generic monodromy (Z/2)^(m-2) ordered, (Z/2)^(m-2) x| S_{m-1} symmetric, uniform in the curve for p>3 (KN-FIND-bdcd60, 4f0d84); cycle-type census is a no-go as a relation-rate probe at every m>=3. Open question KN-OPEN-62c870 Q1/Q3 remains.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 41, 150). atlas_tally.json matched but was not read (JSON blob).

## 2. Hypothesis
For m=3 over prime fields p~2^12, the Frobenius cycle type of the roots of S_3(x1,x2,X) (split vs inert, determined by chi(f(x1)f(x2))) has the same split probability (1/2) on every curve, i.e. the curve-to-curve dispersion of the split fraction is binomial, and the per-curve split fraction does not correlate with the per-curve relation rate (pairs in a random factor base summing to a random target x). So the census is no gate.

## 3. Prediction and falsification
- Statistic 1: dispersion D = Var_curves(split fraction) / (s(1-s)/NS). Predicted D < 1.6 for both population A and the independent population B (A/A arm: |log(D_A/D_B)| < 0.5 required for validity).
- Statistic 2: Pearson correlation of split fraction vs relation rate across A's curves, two-sided permutation p (shuffle null). Predicted p > 0.01.
- Positive control: population with a planted per-curve bias (0.06 on odd curves) must give D > 3.2, else inconclusive.
- Decisions: control or A/A failure -> inconclusive; D<1.6 on A and B and p>0.01 -> consistent_with_prediction (obstruction re-derived at toy scale); otherwise falsified (a curve-dependent cycle-type signal exists).
- Negative control: the permutation null and the binomial null. A/A: independent seeded population B.

## 4. Method
Pure stdlib. p=4093, K=60 random curves y^2=x^3+ax+b per population (seeds 1101, 2202, bias population 1101+7, permutation 3303). Per curve: NS=15000 random (x1,x2); split iff disc/4 of S_3 in X is a nonzero square mod p. Relation rate: FB=96 x-coordinates of rational points, NT=150 rational targets x3, count unordered FB pairs with S_3(u,v,x3)=0 (direct evaluation). Cost ~ 60*150*4600 evaluations per population, about 8e7 operations total; expected a few minutes at most on one core (can be near the 10 minute cap in slow Python; timeout 660 given). Memory negligible. Scope: m=3 only.

## 5. Classification
runnable_local: stdlib toy decides the m=3 case; the closed lead's bound is re-derived empirically.

## 6. What a run cannot establish
Not a proof of the monodromy group; m>=4 not tested; toy p only; relation rate is a sum-of-two-FB-points count on x-coordinates with no linear algebra or cost pricing (no rho comparison, no GAE band); dispersion thresholds are heuristic. Results labelled [pcc:unverified].
