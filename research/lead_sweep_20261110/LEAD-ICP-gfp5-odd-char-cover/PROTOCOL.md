# LEAD-ICP-gfp5-odd-char-cover

## 1. Lead as recorded
- id LEAD-ICP-gfp5-odd-char-cover; disposition `open_blocked` (also 20261101); family `ic-prime`; last_reviewed 2026-10-24.
- Title: "Odd-characteristic GHS/Diem cover applicability for EcGFp5, EcMasFp5 and their isogeny classes".
- Next-step excerpt (truncated in leads.json): "Compute the odd-characteristic cover genus (or Diem bound) for both curves plus an isogeny-class".
- Source: atlas_survey_20261107 --pursuit.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 35, 60, 142, 166, 236, 292-315 only).
  The survey places it with TRF-odd-prime-ghs-cover-floor (genus floors 9/49/41 at k=6/10/14) and the closed
  gfp5-symmetrized-gaudry (c_cyc 2^46.3-2^46.6). Not read: ecdlp_subexp_desk atlas_tally.json, any cover-genus computation.

## 2. Hypothesis
For E over F_q, q = p^n, n odd prime, p odd, generic coefficients, the Gaudry/Diem "cover" attack uses the
subfield factor base {P : x(P) in F_p} (about p points, sums of n of them). H: this structured factor base covers
E(F_q) with m-fold sums exactly like a random factor base of equal x-class count, i.e. the subfield structure gives
no odd-characteristic coverage advantage beyond the generic 1/n! decomposition probability (lambda = 2^m C(c,m)/#E).
This is a coverage proxy for the lead's genus/Diem-bound question; it does not compute a genus.

## 3. Preregistered prediction and falsification criterion
- Statistic R = sum over cells of distinct covered group elements (subfield FB) / same for random FB (mean of 3 draws).
- Prediction: |R - 1| < 0.1 -> `consistent_with_prediction` (no structural advantage at toy size).
- Falsified if R > 1.25 (subfield FB covers materially more), provided the positive control is valid.
- Otherwise `inconclusive`; also inconclusive if the positive control fails (R_ap > 0.9) or the time budget cuts cells.
- Null/negative control: random x-classes of the same count c drawn from the same curve (same instrument and code path).
- Positive control: structured FB {kP}: sums collide heavily, so R_ap = covered/null must be <= 0.9 (observed 0.34 in smoke).
- A/A arm: spread (max-min)/mean among the 3 random-FB draws is reported as the noise floor; |R-1|<0.1 is only meaningful
  if that floor is of comparable size, so note it when reading the verdict.
- Also reported: Poisson expected distinct count #E(1-e^-lambda) per cell.

## 4. Method
- Pure-Python F_{p^n} via monic x^n+ax+b (irreducibility by x^{p^n}=x, x^p!=x), log/antilog tables from a random
  primitive element, y^2 = x^3 + a x + b with a, b uniform in F_q; #E counted by squareness via logs.
- Cells: (p,n,m) = (61,3,3) seeds 101-103 (q=226981, #E ~ 2^17.8, c ~ 30, ~32k sums, lambda ~ 0.14) and (499,2,2)
  seeds 201-203 (q=249001, c ~ 250, ~125k sums, lambda ~ 0.5); 3 random-FB draws each; sums over distinct x-classes
  with both signs, so no trivial P-P cancellation. Fixed seeds via random.Random(seed).
- Descriptive only: (p,n,m)=(7,5,5) seed 301 (the n=5 case; lambda ~ 1/120, too small for a statistical test).
- Cost: estimated 2-6 minutes, ~100 MB (tables of ~250k ints); internal 450 s guard, `timeout 660` outside.

## 5. Classification
runnable_local. The stdlib toy decides the coverage proxy; the real next step (cover genus/Diem bound for EcGFp5 and
EcMasFp5, and their isogeny classes) needs the exact curve parameters over F_{p^5}, p = 2^64-2^32+1, plus
function-field/genus computation (not available here), so this run only sharpens the heuristic.

## 6. What a run cannot establish
- No genus or Diem bound for EcGFp5 or EcMasFp5 and nothing about their isogeny classes or p = 2^64-2^32+1.
- Toy sizes (n=2,3) stand in for n=5; at n=5 only the descriptive cell is run. Random curves, not the named curves.
- "Consistent" means only no large (>10%) coverage advantage at these sizes; it does not rule out a cover of small
  genus (a different object). Result must be labelled [pcc:unverified]; no rho comparison or GAE pricing (5a) is made.
