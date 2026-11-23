# LEAD-ICP-icperf-writability-boundary

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (disposition_20261101: closed_obstructed); family: ic-prime; last_reviewed 2026-10-24; source atlas_survey_20261107 --pursuit.
- title: Writability of the descended system - arity forced to m >= 4 and the ANF too large to write inside the rho budget below n ~ 300.
- next-step excerpt (truncated): "n >= 409, or a representation (e.g. chained S_3 at m = 5, 6, ~2^31-2^33 monomials) whose written ..."
- atlas_survey line 34: binary field only; written ANF exceeds rho by +40.8 bits at ECC2K-130 and is -4.9 bits at n=409.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 31-35, 236-242 only). atlas_tally.json matched by grep, not read. Nothing in cryptanalysis searched.

## 2. Hypothesis
For the Weil-descended Semaev summation-polynomial system over GF(2^n) (S_3 with m=2 free points, S_4 with m=3, one factor-base subspace of dimension ceil(n/m) per point), the number of written ANF terms over the n Boolean component equations grows as 2^(c*n) with c >= 0.6 for both m, i.e. strictly faster than the rho budget 2^(n/2). So the recorded writability obstruction (writing cost above rho cost at small n) is a scaling property, and only a crossover at large n (409) can remove it.

## 3. Preregistered prediction and falsification criterion
- Statistic: least-squares slope c of log2(ANF terms) against n, per m, over all (n, seed) rows.
- Threshold: consistent_with_prediction if min over m of c >= 0.6; falsified if max over m of c < 0.5 (writing grows within the rho budget slope, so the obstruction would not scale); otherwise inconclusive.
- Null control: same monomial support with uniformly random GF(2^n) coefficients (one size); its log2 terms should match the structured system within about 1 bit, else the count reflects support rather than algebraic cancellation (reported, not decisive).
- Positive control: the dense cap log2(2^(m*l)*n) per row; observed terms must not exceed it (sanity of the counter).
- A/A arm: three further seeds (different b, subspaces, R) at one size; their spread of log2 terms is the noise floor, and slope differences below it are not read as an effect.
- Rho budget slope 0.5 is the comparator; the 5a GAE pricing band is not applicable, since no cross-method ranking is made.

## 4. Method (exact algorithm, parameters, seeds, sizes; cost estimate within the machine limits)
- Field GF(2^n), modulus found by brute irreducibility search. Curve y^2+xy=x^3+ax^2+b, b random nonzero. S_3(x1,x2,t)=A t^2+B t+C with A=(x1+x2)^2, B=x1x2, C=x1^2x2^2+b. S_4 = Res_t(S_3(x1,x2,t), S_3(x3,R,t)) = (AC'-A'C)^2 + (AB'-A'B)(BC'-B'C) (char 2).
- Each free x_i = sum_j y_ij * basis_ij, l = ceil(n/m) random basis vectors, Boolean variables y with y^2=y. Polynomials held as dict mask -> GF element; ANF terms = total set bits of coefficients (written size of the n component equations).
- m=2: n=6..14; m=3: n=6..11; seeds 11,22,33 (derived seed 1000*seed+n); null at n=10 (seed 5); A/A seeds 101,102,103 at n=10.
- Cost: pure Python; m=3 n=11 dominates (products of about 4k-term polynomials). Estimate 1-5 min, <200 MB; the 660 s timeout applies.

## 5. Classification: runnable_local
A stdlib toy decides the scaling slope directly. Smoke test (`python3 -I run.py --smoke`) passed in under 1 s.

## 6. What a run cannot establish
- Toy n<=14 only; slopes there need not extrapolate to n=130..409, and Boolean reduction saturates near the dense cap, so a slope near 1 may reflect saturation, not true structure.
- Only m=2,3 are run; the lead concerns m>=4, chained S_3 at m=5,6 and n>=409, out of reach. No claim about the +40.8 / -4.9 bit figures or any crossover n.
- Counts a naive expansion, not an optimized representation (factored forms could be smaller); no claim about solving cost, Groebner or SAT behaviour, or the prime-field case.
- Result is [pcc:unverified]; not an improvement over the index-calculus baseline.
