# LEAD-EXO-low-degree-algebraic-query-bound
## 1. Lead as recorded
- Disposition: open_next_step. Family: exotic. Title: (empty in leads.json). Source: atlas_survey_20261107 --pursuit.
- Next-step excerpt (truncated): "Prove the single-test lemma, then try to extend it to adaptive tuples ((a_i + b_i k)P). In paral..."
- Atlas survey text: "conjecture T >= sqrt(N)/D, proof program" (D = degree of the algebraic test).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only: lines 72-74, 189-191). No other hits in crypto or cryptanalysis; the lead's own file was not found, so the mechanism below is my reconstruction.
## 2. Hypothesis
Model: an algorithm queries points Q_i=(a_i+b_i k)P (b_i invertible) and may apply a polynomial test f of total degree <= D to the x-coordinates. Claim H: (i) single test: f(x(Q))=0 holds for at most 2D values of k (Bezout/root count), so a single test cannot do better than ~N/(2D) queries; (ii) for a bivariate test f(x1,x2) of degree D on a pair with b1 != b2, the number of k on which f vanishes is not larger than about the interpolation count M-1 = (D+1)(D+2)/2 - 1, i.e. R = zeros/(M-1) stays O(1) and does not grow with N (so adaptive tuples gain only poly(D), compatible with T >= sqrt(N)/D).
## 3. Preregistered prediction and falsification criterion
- Statistic: R = (number of k where a fitted degree-D polynomial vanishes)/(M-1), where f is the nullspace polynomial interpolating M-1 random k; max and mean over 8 reps per (N, D) cell.
- Prediction: zero Bezout violations in the univariate check (zeros <= 2D); max R <= 1.5 in all cells; null arm max R <= 1.0; mean R at largest N <= 1.5*mean R at smallest N + 0.5.
- Falsified if any Bezout violation, or any R > 4, or R grows with N beyond the above. Otherwise inconclusive. Consistent decides only that no toy-scale evidence of extra vanishing beyond interpolation; falsified would mean tuples have hidden structure to be studied.
- Negative control/null: random-coefficient f of the same degree (expected R near 0). Positive control: the interpolated arm itself (R>=1 by construction, shows detector sees forced zeros) plus the exact-root univariate arm. A/A: second fitted arm "fit_AA" with independent derived seeds; max |mean difference| is reported (noise floor).
## 4. Method
Pure stdlib. For log2 p in {11,13,15} (seed 20261110, fixed derivation), find a prime p and a prime-order curve y^2=x^3+ax+b by point counting (N ~ 2^11..2^15). Precompute x(kP) for all k. Univariate check: random root sets of size D in {2..5}, random (a,b). Bivariate: random (a1,b1,a2,b2), b1!=b2; solve nullspace mod p over monomials of total degree <= D; count vanishing k. Cost est. 1-3 minutes, <100 MB.
## 5. Classification
runnable_local. A stdlib toy can measure the vanishing count; the lemma proof itself is separate desk work.
## 6. What a run cannot establish
It does not prove the lemma or the sqrt(N)/D bound, tests only x-coordinate bivariate tests (not adaptive or higher tuples, nor non-interpolating f chosen by search), toy N <= 2^15, a bound on vanishing count is not a query lower bound in a generic-group or algebraic model. Results are [pcc:unverified].
