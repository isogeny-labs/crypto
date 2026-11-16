# LEAD-STR-glv-gls-isogeny-loop-scalar-mult

## 1. Lead as recorded
- id LEAD-STR-glv-gls-isogeny-loop-scalar-mult; family structure; disposition `open_next_step` in leads.json (source atlas_survey_20261107 --pursuit; last_reviewed 2026-10-24); `disposition_20261101` = `closed_obstructed`.
- Title: "GLV/GLS and isogeny-loop endomorphisms, neighbour-model routing (known-scalar multiplication)".
- next_step_excerpt (truncated in source): "An ECDLP algorithm whose dominant cost is variable-base scalar multiplication, not additions, is ..." (the reopen condition).
- Atlas survey one-line record: "known-scalar mult only; chain GLV-2 1.525x over wNAF; loop costs 20-93x doublings saved". Related closed leads: endo-scalar-action-generic (End(G)=Z/N, GGM-simulable), automorphism-quotient-rho (ceiling sqrt|Aut|).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 54, 145-147, 259 only); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep context only).

## 2. Hypothesis
On a toy prime-order j=0 curve (p = 1 mod 3, N ~ 2^17), (H1) GLV decomposition with Shamir interleaving cuts the group-operation count of known-scalar multiplication to 1/r of plain binary double-and-add with r in [1.45, 2.0] (correct results); and (H2) the only endomorphism-derived gain for ECDLP collision search is the unit-orbit ceiling: sampling classes under the order-6 automorphism group needs 1/sqrt(6) = 0.408 of the plain draws (tolerance +-0.08), not more. Together: GLV speeds scalar multiplication only, and gives no sub-sqrt(N) lever for the discrete log, consistent with the recorded obstruction.

## 3. Preregistered prediction and falsification criterion
- Statistics: S1 = mean ops(binary)/mean ops(GLV-Shamir) over 300 random scalars; S2 = mean draws-to-first-collision (orbit-labelled)/(plain) over 20 seeds each.
- Prediction: S1 in [1.45, 2.0], all GLV products equal binary products; S2 in 0.408 +- 0.08.
- Falsified if S1 outside its band or any product mismatches (H1 broken), or S2 outside tolerance. S2 clearly below 0.33 would be a gain beyond the unit ceiling and would reopen the lead; S2 above 0.49 means the orbit model is wrong.
- Negative control: GLV with a wrong eigenvalue must fail correctness in >= 15 of 20 products (detector sees broken decomposition). Null for S2 is the plain (unlabelled) arm.
- Positive control: negation-only labelling should give about 1/sqrt(2) = 0.707 (reported, not gated).
- A/A arm: two plain arms with disjoint seeds (1000+s vs 2000+s); ratio must be in [0.8, 1.25], else verdict `inconclusive`.

## 4. Method
Python 3 stdlib only. Seed 20261110. Find prime p in [2^17, 2^18), p = 1 mod 3, and b in 1..39 with y^2=x^3+b of prime order N (Legendre counting). Find eigenvalue lambda (root of l^2+l+1 mod N, Tonelli-Shanks) and beta (cube root of 1 mod p) with [lambda]P = (beta x, y). Lattice {(a,b): a+b*lambda = 0 mod N} Gauss-reduced; k split by Babai rounding. Op count = affine additions/doublings (negation free). Table of all N multiples is built once; orbit label of k = min over {+-k, +-k*lambda, +-k*lambda^2} (equivalent to point orbit under the order-6 group). Draws: uniform k, count until a repeated label. Cost: a few minutes of one core, < 100 MB.
`--smoke` uses 11-bit p and tiny counts (loose thresholds).

## 5. Classification
runnable_local. A stdlib toy decides both claims; no GPU, build or network.

## 6. What a run cannot establish
- Toy sizes only (N ~ 2^17); op ratios are asymptotically ~1.7 for plain binary, and say nothing about wNAF baselines (recorded 1.525x) or 256-bit costs.
- Uniform-sampling birthday is not rho: fruitless cycles and walk mixing are not modelled; it checks only the sqrt|Aut| ceiling.
- It does not address GLS, isogeny loops or neighbour routing, nor non-prime-field cases.
- Absence of a gain is not a proof; the closed_obstructed status rests on the theorem (End(G)=Z/N) already in the atlas. Results are [pcc:unverified].
