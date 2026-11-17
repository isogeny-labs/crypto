# LEAD-ICP-xedni-function-field-sections
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (disposition_20261101: closed_obstructed); family: ic-prime; last_reviewed 2026-10-24.
- title: Function-field xedni - Mordell-Weil sections of elliptic surfaces over F_p(t) as a relation source.
- next_step_excerpt (truncated in leads.json): "A surface family with a dimension surplus (more free section parameters than prescribed conditio..."
- recorded evidence (survey): exact P_lift(p) = (1-2/p+p^-3)/(2p^3), alpha = 3, checked on 596,268,874 slots with zero disagreements; viability needs alpha < 1/2; reopen only for a surplus family with lift exponent < 1/2. Proof-kind for rational elliptic surfaces.
- files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 25, 159, 244 only). README.md, atlas_tally.json, candidate_matches.json were hit but not read. Lead atlas YAML encrypted/unavailable.
## 2. Hypothesis
In the classical (non-function-field) xedni toy, lifting a mod-p-collinear triple of E(F_p) points to integers with coordinates u+p*k, v+p*k' (k,k' in 0..3) gives P(lifted points collinear over Z) that decays like p^-alpha with alpha >= 0.9 (> 1/2), so the lift exponent needed by the lead (< 1/2) is not reached by unstructured lifts; this is a sanity proxy for, not a test of, the function-field alpha = 3.
## 3. Preregistered prediction and falsification criterion
- Statistic: alpha_hat = -slope of log2 P_hat vs log2 p, primes {127,251,509,1021,2039}, 600k samples each (rows with >= 5 hits only).
- Prediction: alpha_hat >= 0.9. Falsified if alpha_hat < 0.5 (unstructured lifts would beat the 1/2 bar). Between 0.5 and 0.9: inconclusive.
- Null/negative control: same sampler but third point random on the curve (not collinear mod p), independent seed; its alpha is reported for comparison (expected not smaller than the main arm's).
- Positive control: integer-collinear lifts reduced then tested; detector must flag 1000/1000, else verdict inconclusive.
- A/A: not separately run; the null arm shares sampler and differs only by seed/third-point rule. Stated limitation.
## 4. Method
Curve y^2=x^3+3x+7 over F_p (b bumped if singular). Draw P,Q random on E(F_p), R = -(P+Q). Lift each coordinate by +p*k, k uniform 0..3. Test determinant == 0 over Z. Seed 20261110 (+1 null, +2 positive). Stdlib only, ~1 minute, small RAM.
## 5. Classification: runnable_local
A stdlib toy can sharpen how the lift exponent behaves for unstructured lifts. Smoke test passes (0.3 s).
## 6. What a run cannot establish
It does not touch elliptic surfaces over F_p(t), Mordell-Weil sections, rank or dimension surplus, so it cannot confirm or refute alpha = 3 or reopen the lead. Collinearity of 3 points is a weaker condition than a full xedni dependency. Small-p fits are noisy and depend on the lift box. Result is [pcc:unverified].
