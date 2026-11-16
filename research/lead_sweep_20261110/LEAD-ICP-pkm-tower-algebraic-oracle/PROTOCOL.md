# LEAD-ICP-pkm-tower-algebraic-oracle

## 1. Lead as recorded
- Disposition: open_blocked (reviewed 2026-10-30). Family: ic-prime. Source: atlas_survey_20261107 --pursuit.
- Title: PKM tower and APS partitioned factor bases solved algebraically over F_p (summation-polynomial systems with low-degree auxiliary membership; first and last fall degree).
- Next-step excerpt (truncated): "Run the preregistered prime fall-degree study (crypto research/prime_fall_degree_20261027, stage..." Cost: expensive, 130-380 GB hosts, dedicated host, custody gap.
- Why it might beat sqrt(p): if solving degree stayed flat in tower depth the PDP would be polynomial per attempt; every measured D grows with N.
- Recorded evidence (atlas_survey.md item 3): F4 solving degree D grows with N (m=2: 3..6 for N=2..20), shape-matched null gives the same D; registered prediction d_F = 4+floor((t-2)/4).
- Files read: /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (grep hit only), /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 15-25, 84-94, 290-300), research/lead_sweep_20261110/leads.json. ls of research/prime_fall_degree_20261027 (PREREGISTRATION.md exists, not read).

## 2. Hypothesis
For Kummer-style square-root towers of depth t (auxiliary variables u_k^2 = a_k u_{k-1} + b_k) feeding a degree-(2,2) three-term summation-shaped equation over F_101, the Macaulay elimination degree (smallest D whose degree-<=D Macaulay span contains a nonzero univariate polynomial in the last variable) is strictly increasing in t for t=1,2,3, and equals that of a shape-matched random-coefficient null. I.e. the algebraic oracle's degree is not flat in depth, matching the recorded obstruction.

## 3. Preregistered prediction and falsification criterion
- Statistic: median elimination degree over 8 seeded instances per (t, arm).
- Prediction: median(t=1) < median(t=2) < median(t=3), all resolved within the degree cap.
- Falsified: all three resolved and median(t=3) <= median(t=2) (flat or falling degree in depth, which would reopen the lead).
- Inconclusive: any median unresolved at its cap/time limit, or positive control fails.
- Null/negative control: arm "null", random polynomial of the same monomial shape in place of the summation-shaped equation. Expected to give the same medians (matches the recorded "shape-matched null gives the same D").
- Positive control: a planted linear 2-variable system, elimination degree must be exactly 1 (detector sees a known degree).
- A/A: t=1 s3 arm rerun with different seed base; reported median gap (expected 0). Not run in --smoke.

## 4. Method
Python stdlib only, F_101, dense-free sparse Macaulay matrices, modular Gaussian elimination with order putting pure last-variable monomials smallest, then test for a pure-univariate row. Two towers of depth t (2t variables). The final equation is a summation-polynomial-shaped surrogate with degree 2 in each of the two top tower variables, with random A,B,R (it is NOT guaranteed to be the exact Semaev S_3; see section 6). t in {1,2,3}, caps D<=8/9/8, 8 reps, seed 20261110 (strings "seed-t-arm-rep"). Wall limit 540 s internal; expected well under 10 min, memory far under 200 MB.

## 5. Classification
runnable_local. A stdlib toy can sharpen the degree-growth question at t<=3 without Groebner engines.

## 6. What a run cannot establish
- Not the registered prime fall-degree study (F4 solving degree, N up to 20, m up to 4); depth t<=3 is far below it.
- The final equation is a surrogate, not verified exact S_3; elimination degree is not first/last fall degree or F4 solving degree.
- p=101 is tiny; finite-field effects dominate. No conclusion about exponent or about beating rho.
- A "consistent" verdict is only compatible with the recorded obstruction. Results are [pcc:unverified].
