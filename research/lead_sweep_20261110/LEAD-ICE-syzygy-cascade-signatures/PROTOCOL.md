# LEAD-ICE-syzygy-cascade-signatures

## 1. Lead as recorded
- disposition: open_next_step (last reviewed 2026-10-24); family: ic-engine
- title: Exploiting the degree-born non-rewritable syzygy (F5 signature) cascade of Semaev systems
- next-step excerpt (truncated): "Run the repaired SIG-D6-NULL-RECAL-v2 calibration, adding the planted-syzygy unit-slope control"
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 44-50, 225-231: lead is binary/GF(2) engine, a method precedent only for prime fields);
  research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md (~5171-5177: atlas cross-check, lead open, not that solve).
  SIG-D6-NULL-RECAL-v2 itself was not found; its definition is reconstructed here (assumption).

## 2. Hypothesis
A Macaulay-matrix syzygy detector, defined as D_d = (#rows) - rank of the degree-d Macaulay matrix, has these properties on toy systems over F_10007 (n=5):
(i) for generic m=5 quadrics, D_3 = 0 and D_4 = C(5,2) = 10 (Koszul syzygies only: the null signature);
(ii) each planted non-Koszul syzygy (a cubic generator l*f_1 in the ideal) adds exactly 1 to D_3, so the slope of D_3 against planted count k is exactly 1 (unit slope).

## 3. Preregistered prediction and falsification criterion
Statistic: D_3 and D_4 per trial; slope of mean D_3 vs k in {0..4}.
Prediction: null arms A and B (A/A, independent seeds 1001, 2002) give D_3=0 and D_4=10 in all 20 trials each; planted arm k gives D_3=k in all 20 trials, slope = 1.
Falsified: any null trial with D_3 != 0 or D_4 != 10 (detector or null model wrong; a rank loss mod p is possible with probability about 1/p per trial, so one miss means rerun with other seeds before concluding).
Inconclusive: null passes but the slope is not exactly 1.
Negative control/null: generic random quadrics (k=0). Positive control: planted k=1..4. A/A: null_A vs null_B, which must agree exactly.
Decides: whether the planted-syzygy unit-slope control is valid, a precondition for using the signature as an F5-cascade detector. It does not decide any speedup.

## 4. Method
Pure-Python modular Gaussian elimination on homogeneous Macaulay matrices (degrees 3 and 4; at most 100x70). n=5 variables, m=5 random quadrics, p=10007, 20 trials per arm, 7 arms (null A, null B, k=0..4 positive); fixed seeds 1001, 2002, 3000+k. Planted generator g = l*f_1, l a random linear form. Cost: under 1 minute, under 50 MB, 1 core. Group orders are not involved (a polynomial-system toy, not a curve). Smoke uses n=m=3.

## 5. Classification
runnable_local. Stdlib toy decides the control exactly; no external engine needed.

## 6. What a run cannot establish
- It is a calibration of a rank-deficit detector on random generic and planted ideals, not on Semaev summation polynomials.
- It does not test F5 criteria, Groebner engines, or non-rewritable-signature counts in a real F5 run.
- The recorded SIG-D6-NULL-RECAL-v2 spec was not available, so this is a reconstruction. The unit slope is nearly a tautology of linear algebra.
- No claim of beating rho or improving the IC baseline. All results are [pcc:unverified].
