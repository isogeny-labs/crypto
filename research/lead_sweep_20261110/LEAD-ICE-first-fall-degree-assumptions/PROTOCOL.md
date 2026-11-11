# LEAD-ICE-first-fall-degree-assumptions

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_blocked (last_reviewed 2026-10-29). Family: ic-engine.
- Title: First-fall-degree / Assumption-1 arguments for subexponential binary ECDLP (Semaev 2015, Nagao, Petit-Quisquater).
- Next-step excerpt (truncated in leads.json): "The uniform degree-two bridge is refuted in the reviewed finite Boolean model (n = n' = 4, x_R ="
- Session log 2026-11-06 adds: next is reading Kosters-Yeo and Hodges-Petit-Schlather (paper full text, absent here).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 288, table row only);
  /home/user/crypto/docs/session-logs/2026-11-06.md (line 859, Idea 4). No hits in cryptanalysis. Atlas YAML encrypted, not read.

## 2. Hypothesis
For Weil-descended Semaev S_3(x1,x2,x_R)=0 over GF(2^n) with x1,x2 in an n'-dimensional F_2-subspace V (n'=n/2, so 2n' = n Boolean unknowns, n quadratic equations), the Boolean Macaulay first-fall degree D_ff (first degree D<=5 at which the rank of {monomial*f_i, deg<=D-2} falls below min(rows, columns)) is lower than for random quadratic systems of identical shape by at least 1 on average, at every tested n in {6,8,10}. This is the quantity a first-fall-degree "Assumption 1" would need; the recorded refutation of a *uniform* degree-two bridge says it should not hold uniformly at degree 2, but a lowered D_ff would be the weaker structured effect.

## 3. Preregistered prediction and falsification criterion
- Statistic: delta = mean D_ff(structured) - mean D_ff(random null), per n. D_ff censored at 6 (= DMAX+1).
- Prediction (consistent_with_prediction): delta <= -1.0 at every n, controls valid.
- Falsified: delta > -0.5 at every n (no meaningful drop; structured system behaves like random at this toy scale), controls valid.
- Otherwise inconclusive. Also inconclusive if controls fail.
- Null / negative control: random quadratic systems (each monomial of degree <=2 kept w.p. 1/2), same n equations, same unknowns.
- Positive control: random system with equation 2 replaced by a duplicate of equation 1 (forced rank defect); must give mean D_ff <= 3.
- A/A arm: second independent set of random-null seeds; |mean D_ff(null) - mean D_ff(null2)| <= 0.5 at every n.
- Secondary (reported, not decisive): D_solve (first D with full column rank).

## 4. Method
- Python 3 stdlib only. GF(2^n) with fixed irreducibles (n=6: x^6+x+1; 8: AES poly; 10: x^10+x^3+1). V = span(1,t,...,t^(n'-1)); x1, x2 get Boolean unknowns a_i, b_i, i<n'. x_R random nonzero constant, a6 random nonzero, per instance.
- S_3 for binary curve y^2+xy=x^3+a2 x^2+a6 taken from memory as (x1x2+x1x3+x2x3)^2 + x1x2x3 + a6 (NOT verified against a source here). Symbolic Boolean polynomials (x^2=x). n coordinate equations; system is quadratic.
- Macaulay matrix per D=2..5 over GF(2), columns = multilinear monomials of degree <=D, rows = monomial(deg<=D-2)*f_i, bitset-int elimination.
- Configs (n, n', instances): (6,3,30), (8,4,30), (10,5,8). Seeds: base=1000*n; structured base+i, null base+100+i, A/A base+200+i, planted base+300+i. Wall guard 420 s skips later configs.
- Cost estimate: well under 5 min, <100 MB. Smoke: n=5, 3 instances, <1 s.

## 5. Classification: runnable_local
A stdlib toy decides the hypothesis at toy sizes without Groebner engines. Paper reading (Kosters-Yeo etc.) remains a separate desk step.

## 6. What a run cannot establish
- Results are [pcc:unverified]. Toy n<=10 with D<=5; says nothing about asymptotic first-fall degree or subexponential ECDLP.
- n'=n/2 is not the n'~n/m regime of Semaev/Petit-Quisquater; D_ff (rank-defect proxy) is not the Dinur-Kosters-Hodges definition of first fall degree and saturates in overdetermined Boolean systems.
- S_3 form is unverified; a wrong S_3 would invalidate the structured arm.
- Random-null density 1/2 differs from the structured system's monomial support; a delta may reflect sparsity, not algebraic structure (no sparsity-matched null).
- Does not re-derive the recorded n=n'=4 refutation.
