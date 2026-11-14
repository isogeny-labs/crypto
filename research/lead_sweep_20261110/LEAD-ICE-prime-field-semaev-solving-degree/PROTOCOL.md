# LEAD-ICE-prime-field-semaev-solving-degree

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_known (last reviewed 2026-10-30). Family: ic-engine.
- Title: "Groebner solving degree / eliminants of prime-field S_3 / S_4 decomposition systems".
- Next-step excerpt (truncated): "Revisit when: A prime-field factor base whose membership equation has bounded degree independent ..."
- Files read: research/lead_sweep_20261110/leads.json (entry);
  research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md (lines 1-60);
  research/prime_field_leads_20261031/L1_lower_bound/atlas_update.md (lines 1-10).
  All under /home/user/crypto. Recorded obstruction: for the direct (dense) presentation,
  D_sol >= v (A3) and rank M_D >= C(m+v-s, m) (A2), giving cost >= c_m * r.

## 2. Hypothesis
For the direct presentation with m=2 (S_3(x1,x2,x_R)=0 with x_R fixed, plus F(x1)=F(x2)=0, F = prod over a random
abscissa set V of size v), the Macaulay solving degree (smallest degree D at which the degree-<=D Macaulay row space
contains the final x1-eliminant) is >= v for at least 80% of instances for v in 3..6, p ~ 100..127. I.e. the
S_3 structure gives no degree saving below the factor-base size, matching the recorded obstruction.

## 3. Preregistered prediction and falsification criterion
- Statistic: pooled fraction f of instances with D_sol >= v (D_sol read off the Macaulay rank/echelon, deg <= v+2).
- Consistent: f_semaev >= 0.80. Falsified: f_semaev < 0.50 (S_3 yields savings, the obstruction is not tight at toy
  size). Otherwise inconclusive. Valid only if both controls pass; else inconclusive.
- Negative control / null: S_3 replaced by a random bivariate polynomial of bidegree (2,2), same F1,F2 (expected f near 1;
  shows the effect comes from the factor-base equations, not S_3 specifically; reported, not gating).
- Positive control: x2 factor base planted to a single point (F2 linear); detector must see savings: f_pos < 0.5 (gating).
- A/A: second independent seed set on the identical S_3 arm; |f - f_AA| <= 0.15 (gating).

## 4. Method
Pure Python 3 stdlib. Curve y^2=x^3+ax+b over F_p (a,b,x_R random), S_3 specialised at x_R. Macaulay matrices for
D=1..v+2 built from the 3 generators times all monomials, mod-p Gaussian elimination with an elimination order
(x2-containing monomials above x1-only), minimum-degree univariate x1 element read from pivots. Sizes: v in {3,4,5,6},
12 instances per v per arm, primes {101,103,107,109,113,127} cycling, 4 arms, seeds random.Random(f"20261110N/arm/v/t")
with arm seeds 1..4. Cost estimate: a few minutes on one core, <200 MB, internal 540 s budget (then inconclusive).
Group orders are not used; the toy is the polynomial system only.

## 5. Classification
runnable_local: a stdlib linear-algebra toy can sharpen the question (the Macaulay solving degree of the S_3 toy system
against v). run.py checked with `python3 -I run.py --smoke` only.

## 6. What a run cannot establish
Only m=2, tiny p and v, a Macaulay (not F4/F5) degree with a fixed extra-degree window v+2, and no auxiliary-variable
tower or S_4. It cannot say anything about asymptotic cost, PKM towers (GAP A4), or comparison to rho. No pcc claim
exists: all results are [pcc:unverified].
