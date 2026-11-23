# LEAD-ICE-descent-closure-degree-at-scale

## 1. Lead as recorded
- disposition: open_blocked (last_reviewed 2026-10-29); family: ic-engine; source: atlas_survey_20261107 --pursuit.
- title: "Does the MutantXL/F4 closure (refutation) degree of descended S_3/S_4 chains stay at 4 as l and n grow?"
- next-step excerpt (truncated): "Run the frozen dreg_closure C1 cell (m = 3, n = 19, l = 7, degree 5) and A2/B1/B2 cells on a hos[t ...]".
- Context: atlas_survey says closure degree 4 through (m,l)=(4,6), degree 5 at (3,7) capped at 169 GB; the 2026-11-06 session log says C1 is blocked at 217 GB dense and a sparse run was at 5.5 GB after 65 min.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 40-60, 280-300); docs/session-logs/2026-11-06.md (lines 1830-1845). No dedicated lead note found; cryptanalysis hits were only atlas-input JSON (not read).

## 2. Hypothesis
For Weil-descended Semaev S_3 systems over GF(2^n) with m = 2 factor-base variables in a random l-dimensional F2-subspace and n = 2l+1 (slightly overdetermined, unsatisfiable draws), the degree-D multilinear XL refutation degree is at most 4 for every l in {3,4,5,6}, and its mean grows by at most 0.2 per unit l. (Toy proxy for the blocked scale cell; the real C1 cell is not run.)

## 3. Preregistered prediction and falsification criterion
- Statistic: per l, closure degree D(l) = least D <= 6 at which 1 lies in the span of monomial multiples of the equations (degree <= D); over 24 unsatisfiable draws per l. Summary: max over all draws and slope = (mean D(6) - mean D(3))/3.
- consistent_with_prediction: max D <= 4 and slope <= 0.2. falsified: max D >= 5 and slope > 0.2 (closure degree grows with l at toy scale). Otherwise inconclusive.
- Negative control / null: random quadratic Boolean systems with the same variable count and per-equation monomial count (arm "null"); the descended system's D should not be lower than null's if structure helps; reported, not a gate.
- Positive control: (a) satisfiable descended instances must never be refuted (detector soundness); (b) an overdetermined random quadratic system (4*nv equations) must refute (D found). Failure of either gives verdict inconclusive.
- A/A arm: same system distribution with a different seed (A vs A_prime); if the mean gap at any l exceeds 0.5, verdict inconclusive (noise floor too high).
- Decides: consistent means the "stays at 4" claim survives at toy scale only (no evidence on the m=3, l=7 cell); falsified would raise the prior that the blocked cell exceeds 5.

## 4. Method
- Field GF(2^n), n = 2l+1 in {7,9,11,13} with fixed irreducible polynomials. Per draw: random l-subspace V, random b, random xR; unknowns x1 = sum a_i v_i, x2 = sum a'_i v_i (2l bits).
- Equations: coordinates of S3 = x1^2x2^2 + x1^2xR^2 + x2^2xR^2 + x1x2xR + b (assumed char-2 Semaev form; Frobenius is bit-linear, so equations are quadratic). n Boolean equations in 2l vars; x^2=x reduction.
- Draws kept only if unsatisfiable (brute force over 2^(2l) <= 4096 points). Refutation: XL with all multipliers of degree <= D-2, GF(2) elimination with Python-int bitsets, test 1 in span.
- Seeds: A 11, A_prime 12, null 13, positive controls 14, 15 (seed*1000+l per arm). 24 draws/l, l in 3..6, D<=6.
- Cost: pure stdlib, one core, estimated seconds to a few minutes, < 50 MB RAM. Smoke: l=3, 3 draws (~0.03 s measured).

## 5. Classification
runnable_local: a stdlib toy decides the sharpened question (does D drift with l for m=2 descended S_3), though not the blocked scale cell itself.

## 6. What a run cannot establish
- Nothing about C1 (m=3, n=19, l=7, degree 5), S_4 chains, MutantXL/F4 (plain XL here is an upper bound on D), or real ECC2K descent systems with structured FB.
- The S_3 form is the assumed standard char-2 one, not checked against curve points; n = 2l+1 differs from production n/l ratios; l <= 6 is the tested range where closure was already known to be 4.
- Unsatisfiable-only conditioning; sample size 24. Results are [pcc:unverified].
