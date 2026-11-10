# LEAD-ICB-prime-degree-algebraic-decomposition

## 1. Lead as recorded
- Disposition: open_blocked (reviewed 2026-10-29; 20261101 disposition open_blocked). Family: ic-binary.
- Title: Bounded-degree algebraic decomposition (Weil-descended S_{m+1} on subspace bases) at prime extension degree: ECC2K-130, NIST K/B, sect571.
- Next-step excerpt (truncated in leads.json): "Measure the F4 / degree-4 closure refutation degree at (m, l) = (4, 5) and (4, 6), and m = 5 wit..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry only; next_step_cost and why_it_might_beat_sqrt_p fields absent). Only the one-line listing in /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 285) was found; /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json also matches but was not read. No detailed lead note found; the mechanism below is from general knowledge.

## 2. Hypothesis
In a toy Weil-descended Semaev S_3 decomposition over GF(2^n), n prime in {11,13}, with the first factor-base variables x1,x2 ranging over a random l-dimensional GF(2)-subspace and x3 = random target, the boolean-ring Macaulay closure at degree D=6 derives 1 (refutes) for at least 50% of the instances that have no solution, in the overdetermined regime 2l < n (n=13,l=5; n=11,l=5; and n=13,l=6 where 2l < n+0). Failure of this at D=6 means closure degree grows beyond the lead's degree-4/5 expectation already at toy size.

## 3. Preregistered prediction and falsification criterion
- Statistic: R6 = fraction of unsolvable (checked by brute force over 2^(2l) points) Semaev instances refuted at Macaulay degree 6, per cell; also R4, R5 reported.
- Prediction: R6 >= 0.5 in every cell, and R6 exceeds the random-null R6 by > 0.2.
- Outcomes: all cells meet it -> consistent_with_prediction; any cell with R6 < 0.5 (soundness and controls ok) -> falsified; otherwise inconclusive (includes: fewer than 5 unsolvable instances, any refutation of a solvable instance = bug, |R6 - R6_AA| > 0.25, positive control < 0.8, or null not separated).
- Negative control / null: random quartic systems of identical shape (n equations, 2l variables, random coefficients, seeds 9000+). Note: random overdetermined systems may also refute easily; if so the verdict is inconclusive, which is itself informative (no structure-specific signal).
- Positive control: 8-variable, 24 random quartic equations (heavily overdetermined) must be refuted at D=6 in >= 80% of 10 draws (detector can see refutation).
- A/A arm: same generator, disjoint seeds 5000+K; R6 must agree within 0.25. Soundness check: zero refutations of solvable instances.

## 4. Method
Field GF(2^n) with fixed polynomials (n=7: x^7+x+1; 11: x^11+x^2+1; 13: x^13+x^4+x^3+x+1). Random seeded l-dim subspace basis B; x1 = sum a_i B_i, x2 = sum c_i B_i (2l boolean variables). System: n boolean equations from S_3(x1,x2,X) = (x1x2+x1X+x2X)^2 + x1x2X + 1 = 0 (binary curve with b=1). Macaulay closure: multiply each equation by all monomials of degree <= D - 4, reduce in the boolean ring (squarefree), Gaussian elimination with Python-int bitsets, test whether the constant 1 is in the span. Cells (n,l): (13,5), (11,5), (13,6). K=40 instances per arm (10 for l=6), D in {4,5,6}. Seeds: primary 1000.., A/A 5000.., null 9000.., positive control 77. Cost estimate: brute-force 2^12 points x instances plus Macaulay with <= ~3k rows; est. a few minutes, well under 10 min and 200 MB (not measured; full run not tested before checkin).

## 5. Classification
runnable_local. The atlas step needs F4 at larger (m,l) (blocked), but the closure-degree question at toy sizes with a pure-Python Macaulay matrix decides a sharpened version.

## 6. What a run cannot establish
Toy n <= 13 says nothing about ECC2K-130, sect571 or asymptotics; Macaulay closure with boolean-ring reduction is a proxy for F4 degree, not F4 itself; m=4,5 decompositions are replaced by S_3 (m=2); no cost comparison against rho and no GAE pricing. Results are labelled [pcc:unverified].
