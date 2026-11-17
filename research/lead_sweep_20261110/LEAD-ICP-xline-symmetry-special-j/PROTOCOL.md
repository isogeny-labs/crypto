# LEAD-ICP-xline-symmetry-special-j

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_unexamined (last reviewed 2026-10-24). Family: ic-prime. Source: atlas_survey_20261107 --pursuit.
- Title: x-line symmetry groups, automorphism folding and summation-polynomial degeneration at special j.
- Next step (truncated in leads.json): "Compute degree-d Macaulay ranks of the digit or chained S_4 system at j = 0, j = 1728 and on the ..." (rest truncated). Cost: cheap (Macaulay ranks at 3 toy primes).
- Why it might help: "Rank drop at j=0/1728 beyond coefficient-block null; at best special curves".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 32, 100-102, 277); /home/user/crypto/research/prime_field_leads_20261031/C5_special_primes/REPORT.md (lines 24-37: symmetry side belongs to this card, not closed). The atlas YAML is encrypted; the truncated remainder of the next step is not known.

## 2. Hypothesis
H0 (prediction): for the chained S_4 system {S_3(x1,x2,u)=0, S_3(u,x3,xR)=0} over F_p (variables x1,x2,x3,u; S_3 the standard Semaev polynomial for y^2=x^3+ax+b), the Macaulay-matrix rank deficit (rows minus rank, at degrees d = 8..12) at j=0 (a=0) and j=1728 (b=0) equals that of generic-j curves and of the coefficient-sparsity null, at p in {97,109,157}. I.e. the automorphism symmetry does not show up as extra low-degree syzygies in the unfolded system. Falsified if a special-j rank drop beyond the null appears.

## 3. Preregistered prediction and falsification criterion
- Statistic: per (p, d) cell, effect = mean deficit(special arm) - mean deficit(generic arm pooled), deficit averaged over 3 random xR per curve; 30 curves per arm.
- Cell hit: |effect| >= max(3 SE, 0.5 syzygy) for j=0 or j=1728 AND the sparsity null arm is not a hit.
- Prediction: zero hit cells. Verdict "consistent_with_prediction" if zero hits and controls pass; "falsified" if hits in >= 2 cells and >= half of all cells; otherwise "inconclusive" (also if any control fails).
- Negative control / null: generic curves (a,b both nonzero random), arms genA vs genB (A/A, split halves; |z| < 3 required in every cell); "smallcoef" arm = a,b in {1,2,3}, same sparsity as generic, non-special j (coefficient-block null).
- Positive control: system {f1, x1*f1} has forced extra syzygies; extra deficit at d+1 must be > 0 (detector can see rank drops).
- Decision: a falsification is only a hint that Macaulay ranks of the unfolded system see Aut(E); it would justify folding (invariants x^2 resp. x^3) and a cost count. Consistent outcome says the unfolded system gains nothing and any benefit must come from folding.

## 4. Method
Pure Python, sparse Gaussian elimination mod p. Build f1=S_3(x1,x2,u), f2=S_3(u,x3,xR) as dict polynomials; Macaulay matrix at degree d = all monomial multiples of f1,f2 of total degree <= d (degree-4 polynomials, so multiplier degree <= d-4 within 4 variables); rank mod p. Seed random.Random(20261110) (single stream, fixed order). Primes 97, 109, 157 (97 and 109 are 1 mod 12; 157 is 1 mod 12; so both special j are curves with F_p-rational extra structure). Special curves: (0,b) and (a,0) with random nonzero b/a, 30 each. Generic: 60 curves with nonsingular check. Cost: roughly 6.7k rank computations at <= 0.05 s, about 2-5 minutes, memory < 50 MB.

## 5. Classification
runnable_local: stdlib toy decides the question for the unfolded chained S_3 system. (Groebner/F4 and the S_4 digit system are not used.)

## 6. What a run cannot establish
- Only the unfolded chained S_3 system at m=3 and toy primes; not the digit system, folded (invariant-ring) system, or larger m; the truncated part of the next step may specify a different system.
- Macaulay rank at degree <= 12 is not solving degree or ECDLP cost; no sub-rho claim follows from either outcome.
- Zero-deficit at small d is expected from generic degree counts; the deficit appears only past the Koszul degree 8, so power is limited.
- Results are [pcc:unverified] and not a validated benchmark row.
