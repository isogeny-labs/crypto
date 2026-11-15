# LEAD-ICP-coppersmith-smallroot-semaev

## 1. Lead as recorded
- Disposition: `open_next_step` (leads.json); `disposition_20261101` = `closed_obstructed` (proof recorded, scoped closure). Last reviewed 2026-10-29.
- Family: ic-prime. Source: atlas_survey_20261107 --pursuit.
- Title: Small-root (Coppersmith / Jochemsz-May) decomposition with lattice-describable (window, AP, GAP) factor bases.
- Next-step excerpt (truncated in leads.json): "A joint lattice for the three-block (4, 2) system (three equations in six unknowns) reaches more ..."
- Cost: "est. cheap (only if a new solver/joint lattice is proposed)".
- Why it might beat sqrt(p): needs reach > 8/9 at (m,w)=(4,2); proved max 0.3536 per block (equal bounds), numerical 1/2.
- Recorded proof (atlas_survey.md line 24): equal-bound lattice reach <= (n! vol Newt(S_3))^(-1/n) = 0.3293 (3 unknowns), 0.3536 (2 unknowns); unequal-bound numerical optimum 0.4357 / 0.4892 -> 1/2; margin about -0.56 proved, -0.39 numerical.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 22-24, 88-90, 144, 170 only; grep hits in README.md, atlas_tally.json, session logs not read in full). No cryptanalysis-repo search done.

## 2. Hypothesis
For the Semaev summation polynomial S_3(x1,x2,x3; a,b) with generic coefficients, the exact Newton-polytope volumes are vol = 14/3 (support projected to 3 unknowns) and vol = 4 (x3 fixed, 2 unknowns), so the equal-bound Coppersmith reach (n! vol)^(-1/n) equals 0.32932 (n=3) and 0.35355 (n=2), both below the IC demand 8/9 by a margin larger than 0.5. The recorded obstruction therefore holds at the level of the equal-bound formula.

## 3. Preregistered prediction and falsification criterion
- Statistic: reach_n = (n! * vol Newt(S_3))^(-1/n), computed exactly (Fractions) from the symbolic support of S_3.
- Prediction: |reach_2 - 0.35355| < 5e-4 and |reach_3 - 0.3293| < 5e-4, and both < 8/9.
- Falsified if either value lies outside its 5e-4 window while controls pass (the recorded numbers would be wrong; the obstruction margin would need re-derivation). Consistent if both inside and controls pass. Inconclusive if any control fails.
- Positive control: univariate degree-2 polynomial must give reach 1/2 (classical Coppersmith N^(1/d)); bivariate linear x1+x2+1 must give 1.0.
- Negative control: unstructured full cube support {0,1,2}^3 (no Semaev structure): reach 48^(-1/3)=0.2752, must be below the S_3 reach, showing the detector separates a structured from an unstructured support.
- Validity control: S_3 expansion must vanish on x-coordinates of P+Q+R=0 on random toy curves (3 seeds, p=1009, 400 triples each, 0 failures allowed).
- A/A arm: the whole derivation is replayed twice in the same process; reach_3 must agree bit for bit. (There is no stochastic noise floor in the exact volume; the seeds only drive the curve check.)
- What the outcome decides: consistent -> the formula part of the obstruction is reproduced; it says nothing on unequal bounds, joint lattices or non-Coppersmith solvers.

## 4. Method
1. Expand S_3 = (x1-x2)^2 x3^2 - 2((x1+x2)(x1x2+a)+2b) x3 + (x1x2-a)^2 - 4b(x1+x2) as a sparse dict in (x1,x2,x3,a,b); take the x-support (13 points).
2. Convex hull volume in 3D (facet enumeration, pyramid decomposition, Fractions) and in 2D (monotone chain, shoelace) for the x1,x2 projection.
3. Compute reach, the controls above, margins to 8/9.
4. Curve check: seeds 20261110, 7, 99; prime p=1009; random short Weierstrass curves; x-coordinates of random P, Q, R=P+Q.
Cost: about a few seconds, <20 MB, single core, stdlib only. `--smoke` uses p=31 and 20 triples.

## 5. Classification: runnable_local
The recorded bound is a closed-form volume computation that a stdlib toy re-derives exactly.

## 6. What a run cannot establish
- It does not run LLL or any lattice, so it does not reproduce the unequal-bound numerical optimum (0.4357/0.4892) or the "1/2" figure.
- The equal-bound formula is an upper bound for a restricted lattice family; it does not exclude a joint lattice for the three-block (4,2) system, non-convex monomial sets, rank-r>=2 GAP substitution, or a non-Coppersmith small-root solver (the lead's own reopen clauses).
- The demand 8/9 is taken from the atlas, not re-derived here.
- Results are labelled [pcc:unverified]; agreement is a consistency check, not a new claim, and is not an improvement over the IC baseline.
