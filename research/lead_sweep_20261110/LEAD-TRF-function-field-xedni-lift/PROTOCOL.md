# LEAD-TRF-function-field-xedni-lift

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (leads.json; a 20261101 field says closed_obstructed). family: transfer.
- title: Function-field xedni: lift relations to sections of elliptic surfaces y^2 = x^3 + a(t)x + b(t) over F_p(t).
- next-step excerpt (truncated): "a section shape with prescribed-fibre lift probability exponent below 1/2, derived before measur..."
- Atlas survey (item 11) records: exact P_lift(p) = (1-2/p+p^-3)/(2p^3), alpha = 3 (596,268,874 slots, zero disagreements); a viable source needs alpha < 1/2; reopen only with a surface family with dimension surplus and lift exponent < 1/2.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 23-36, 170-175, 265); the atlas_tally.json hit was matched by grep only, not read.

## 2. Hypothesis
For the section shape "a(t), x(t), y(t) each of degree <= 1 over F_p, b(t) := y^2 - x^3 - a x", the probability that a uniformly random member has a prescribed fibre datum (a0, x0, y0), y0 != 0, at t=0 equals p^-3 exactly, so the empirical exponent alpha_hat = -d log P / d log p is 3, not below 1/2. This shape therefore does not meet the lead's reopen condition (alpha < 1/2): fixing a fibre costs one full F_p-coordinate per fibre parameter, and the section/surface parameters add no surplus.

## 3. Preregistered prediction and falsification criterion
- Statistic: alpha_hat, least-squares slope of -log P vs log p over p in {5,7,11,13}, P = exact hit count / p^6 over exhaustive enumeration.
- Prediction: alpha_hat in [2.5, 3.5] for both prescribed targets (mean used).
- Falsified: mean alpha_hat < 0.5 (a shape with exponent below 1/2 exists in this family). Inconclusive: alpha_hat in [0.5,2.5) or (3.5, inf), or any control fails.
- Positive control: "planted" arm, prescribing only y0 given (a0,x0): exact exponent 1; must read 1 +- 0.2, proving the slope estimator sees low exponents.
- A/A arm: two independent seeded prescribed targets, same instrument; |alpha_hat difference| <= 0.3.
- Null/negative control: the defining identity y^2 = x^3+ax+b is re-verified at t=2 for every enumerated tuple (0 violations required), i.e. the enumeration really is the surface-section family; no random-label null is otherwise applicable (exact counts, no sampling noise).

## 4. Method
Stdlib Python, one core. For each p, enumerate all (a0,a1,x0,x1,y0,y1) in F_p^6, build b(t) as a cubic by polynomial arithmetic, check the surface identity at t=2, tally fibre-0 data. Seed 20261110 (random.Random) chooses the two targets. Cost: sum p^6 about 7.0M inner iterations, estimated 30-90 s, memory under 10 MB. `--smoke` uses p in {3,5}. Command: `python3 -I run.py`.

## 5. Classification: runnable_local
A stdlib exhaustive toy decides the (deliberately narrow) shape-specific exponent question.

## 6. What a run cannot establish
- It checks a near-tautological dimension count for one linear shape, not the atlas's exact P_lift formula or its 596M-slot check, and not higher-degree, non-Weierstrass-section, or dimension-surplus families, which are the real reopen routes.
- Nothing about xedni lift smoothness, heights, or ECDLP cost; p <= 13 only, so asymptotic exponent is inferred from exact scaling.
- Results are [pcc:unverified]; a "consistent" verdict is a negative for this shape only and does not close the lead.
