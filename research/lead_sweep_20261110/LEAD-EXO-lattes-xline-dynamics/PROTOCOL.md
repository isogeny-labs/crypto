# LEAD-EXO-lattes-xline-dynamics

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (carries an `obstruction:` block; atlas survey says "proof: forward iterates are group ops, GGM-simulable").
- family: exotic. title: empty in leads.json.
- next-step excerpt (truncated): "Any dependence on k of the F_p-factorisation multiset of phi_d^(n)(x) - x(kP) at a matched curve ..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 71-74, 155, 170, 189 only; the only hit of the rg search; nothing found in /home/user/cryptanalysis was searched beyond this task's scope of reading, so none read).
- The lead's full text is encrypted here; the rest below is from the mechanism (Lattes map phi_d(x) = x([d]Q)).

## 2. Hypothesis
Let E/F_p, P of order r, gcd(r,d)=1 (take d=2), kP not 2-torsion and not O. The roots of phi_d^(n)(x) - x(kP) are the x(Q) with [d^n]Q = +-kP, i.e. x(Q0+T), T in E[d^n], where Q0 = [d^-n mod r](kP) is F_p-rational. Frobenius then acts on the roots as on the fixed torsion torsor Q0+E[d^n] with trivial base point, so the F_p-factorisation degree multiset (the pattern) is independent of k. Claim H: for r odd, d=2, n in {1,2,3}, the pattern is the same for every admissible k and every generator P of every such curve (zero k-dependence). Consequence: the factorisation multiset carries no information about k, so this x-line observable gives no ECDLP leverage beyond a GGM simulator. When gcd(r,d)>1 the pattern does depend on k (coset of kP in d^n E(F_p)); that is the detector's positive control, not the lead.

## 3. Preregistered prediction and falsification criterion
- Statistic: D = number of null curves (r odd) for which >1 distinct DDF pattern (squarefree-part degree/count multiset) occurs over the sampled k; plus M = number whose pattern at an independent generator/k=1 differs from the k-sample pattern.
- Prediction: D = 0 and M = 0 over all null curves (n=1,2,3). Threshold: exact zero (the claim is a theorem-level invariance; any nonzero count is a counterexample or a code bug).
- Outcomes: D=M=0 with controls passing => consistent_with_prediction (lead's next step closes: no k-dependence at these sizes; matches the recorded GGM obstruction). D>0 or M>0 with controls passing => falsified (recheck code first; genuine would be a surprise needing exact-arithmetic replay). Controls failing, timeout, or no null curves => inconclusive.
- Null/negative arm: the odd-order curves themselves (prediction is invariance).
- Positive control 1: curves with 2^n | ord(P): pattern must depend on k in >= 80% of curves.
- Positive control 2: random degree-4 rational map (not Lattes) at random targets: >= 90% of maps show dependence. Shows the DDF instrument can see dependence.
- A/A: positive-control curves, k samples split into two halves by sample order; mean total-variation distance of pattern distributions must be <= 0.35 (noise floor of the same instrument).

## 4. Method
- Python stdlib, seed 20261110 (single random.Random), p = 3 mod 4 prime in [2000,4000) (group orders about 2^11..2^12), random (a,b) nonsingular.
- Build phi_2^(n) = X_n/Z_n by n-fold substitution into the homogeneous duplication forms X^4 - 2aX^2Z^2 - 8bXZ^3 + a^2Z^4, 4Z(X^3 + aXZ^2 + bZ^3). F_k = X_n - x(kP) Z_n, degree 4^n.
- Pattern: gcd with derivative (flag nonsquarefree), then distinct-degree factorisation; record {degree: count}.
- Null arm: n=1: 12 curves x 60 random k; n=2: 12 x 60 (deg 16); n=3: 3 x 5 (deg 64). Cross-check at a second generator (k=1).
- Positive control 1: n=2: 10 curves x 60 k; n=3: 2 x 6. Positive control 2: 10 maps x 60 targets.
- Cost: pure-Python polynomial arithmetic mod p; estimate 3-8 min single core, < 50 MB. Internal deadline 520 s then verdict inconclusive. Smoke: tiny sizes, < 1 s.

## 5. Classification
runnable_local. The question is decided by a stdlib toy: exact finite-field polynomial factorisation at toy sizes.

## 6. What a run cannot establish
- Only d=2, n<=3, p<=4000; says nothing at cryptographic sizes or for d odd / gcd(r,d)>1 beyond the positive control.
- Pattern invariance does not exclude k-dependence of finer objects (the actual factors, the Galois action beyond degrees, or coefficients).
- A null result is the expected consequence of a known GGM simulability argument; it is not an ECDLP speedup and not evidence about other oracles.
- Small n=3 sample; DDF assumes the squarefree part; nonsquarefree cases are only flagged, not analysed.
- All results are [pcc:unverified] and stand only as a toy measurement.
