# LEAD-EXO-hashed-factor-base-tensor-ecdl

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-30). Family: exotic. Source: atlas_survey_20261107 --pursuit.
- Title: Hashed-factor-base ECDL with a compact low-degree Boolean encoding and exact tensor contraction (Akshima et al., Bochum v0.2), conditional on the openai/math #k-SAT result.
- Next-step excerpt (truncated in leads.json): "Two steps, either first. (a) Independent audit of correctness and of the complexity proof (crypt..." Cost: step (a) audit cheap; (b) panel blocked.
- Why it might beat sqrt(p): claims subexponential ECDL via exact tensor contraction; unaudited; native attempts at ECCp-131 exceed leaf budget.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 74, 112, 185 only); .../README.md (lines 76-80, 207-213, 297-301). Nothing found in /home/user/cryptanalysis (search not needed beyond the crypto hits; not read).
- Recorded context: 34 native correctness tests pass; ECC2K-130/ECCp-131 attempts exceed leaf budget without a relation; the desk note asks whether the Boolean encoding's variable count makes counting lose to enumeration.

## 2. Hypothesis
Any exact relation-collection pipeline whose relation source is a hashed factor base with m=2 decompositions (the baseline the tensor-contraction encoding must beat) has toy-size total cost C(N) = ops / sqrt(N) that grows with log2(N) at slope >= 0.25 (generic theory: cost ~ N^(1/2) * poly, giving C ~ N^0.5), and C >= 1 at every size 2^12..2^18. A subexponential claim would require C(N) to flatten or fall; the toy baseline should not.

## 3. Preregistered prediction and falsification criterion
- Statistic: slope s of log2(median C) vs log2(N) over sizes 2^12, 2^14, 2^16, 2^18 (3 curves each, median), plus C at the largest size.
- Prediction: s >= 0.25 and every median C >= 1.
- Falsified: C < 1 at the largest size, or s <= 0 (the baseline flattens: hashing the factor base would be giving something sub-sqrt at toy size; would then justify the step (b) panel).
- Inconclusive: otherwise, or if a control fails.
- Negative control / null: the same instrument running Pollard rho (collision walk) on the same curves; its steps/sqrt(N) must have |slope| < 0.1.
- A/A arm: rho with two independent seeds; reported side by side (noise floor).
- Positive control: planted advantage, the IC cost divided by N^0.4; its slope must come out < 0.25, showing the slope detector sees a real exponent reduction.
- Consistent verdict requires all controls to pass.

## 4. Method
- Python 3 stdlib only. Per size, 3 random prime-field curves y^2=x^3+ax+b, p random odd prime of the given bit length; N = #E(F_p) by Legendre table (O(p)).
- Relation source: factor base = points with the B smallest valid x, hashed by x. A trial draws a random point R and succeeds if x(R - f_i) is in the hash for some i (m=2, up to sign). Cost per trial = B counted subtractions; B relations are collected (trial count extrapolated from the observed rate when capped); linear algebra charged 3 B^2. Total = trials*B + 3B^2, minimized over a grid B = frac*k*sqrt(N)/2 (frac in {0.25,0.5,1}, k in {1,2,4}).
- Rho arm: 16-partition walk with a collision dictionary, steps counted on the same group-op instrument.
- Seed 20261110 (curve seed = seed*1000 + bits*10 + curve index; rho seeds seed+1+c, seed+2+c). Cost estimate: well under the limits (several minutes at most, <200 MB); the full run is not executed at protocol time.

## 5. Classification
runnable_local. A stdlib toy decides the slope question directly and cheaply. It does not audit the Akshima et al. proof, which is a separate desk step.

## 6. What a run cannot establish
- It does not audit the paper's complexity proof or its Boolean encoding, and does not implement tensor contraction; it tests only the hashed-factor-base, m=2 enumeration baseline.
- Toy sizes (2^12..2^18) cannot see asymptotic exponents; four points give a coarse slope.
- Rho arm uses the whole group order N as the scale (assumes the random point is near-generating), a coarse control.
- A "consistent" verdict says the baseline is not sub-sqrt at toy size, not that the claim is false. Results are [pcc:unverified].
