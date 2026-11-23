# LEAD-ICB-orbit-linear-algebra

## 1. Lead as recorded
- disposition: exhausted_at_scale (disposition_20261101 same); family: ic-binary; last_reviewed 2026-10-24.
- title: "Linear algebra of the orbit-folded relation matrix (equivariant blocks, sparse filters, Wiedemann, CADO-style merge)".
- next-step excerpt (truncated): "Revisit when: Factor bases of 10^6+ columns or a large-prime arm that makes linear algebra a mat[rix-bound phase]".
- Files read: research/lead_sweep_20261110/leads.json (entry; no next_step_cost/why fields present);
  research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 75-80 and 303-306, status listing only);
  research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md (grep context at line 5775 only: mentions the lead as "the relation matrix, not this solve").
  Other hits (atlas_tally.json, docs/session-logs/2026-11-02.md) not read. No dedicated lead note found.

## 2. Hypothesis
Recorded obstruction: at toy scale, orbit folding is only a column-count reduction, so linear algebra is not where the time goes. Exact claim tested: for a relation matrix over Z_r (r prime, r = 1 mod 3) with a free 6-element automorphism orbit (+-, lambda^k, lambda^3=1), folding to orbit-representative columns (entries c*s*lambda^k) gives a dense Gaussian-elimination cost (counted mulmods) that is smaller than the unfolded solve by |Aut|^3 = 216, within a factor [0.7, 1.3], at the largest size, while both solves recover the hidden logs exactly.

## 3. Preregistered prediction and falsification criterion
- Statistic: median over 5 seeds of ops_unfolded / ops_folded at m = 36 orbits (n = 216 unfolded columns, n+8 rows each).
- Prediction: statistic in [151, 281] (0.7x to 1.3x of 216). Outside that range, with controls valid: falsified (the cubic fold gain is not realised in this construction; the recorded obstruction would then need a different accounting). Inside: consistent with the cost model only.
- Positive control: folded and unfolded solves both recover all hidden logs on every main-arm seed (detector can see a correct fold).
- Negative control / null: fold with a wrong eigenvalue (fixed non-root mu = 12345); folded solve must fail to recover (rank n, no kernel).
- A/A arm: same protocol, seeds shifted by +1000; relative difference of median ratio vs main must be <= 10%.
- If any control fails: verdict inconclusive.

## 4. Method
Pure Python 3 stdlib. r = least prime >= 2^20 with r = 1 mod 3; lambda = g^((r-1)/3) (nontrivial). Per trial: m hidden random orbit logs y_i; unfolded columns (i,k,s), k in 0..2, s in {+1,-1}, hidden log s*lambda^k*y_i. Each of n+8 relations has weight 4, random columns, random coefficients, last coefficient fixed so the hidden-log sum is 0 mod r. Folded row: coefficient c*s*mu^k added to column i (mu = lambda for main/A-A). Dense Gauss elimination mod r with an explicit mulmod counter (n-c per row update); require rank n-1 and pivot columns 0..n-2; normalise last column and compare to hidden logs. Sizes m in {6,12,24,36}; seeds 101..105 (main), 1101..1105 (A/A), 101..103 (null). Cost estimate: roughly 30-60 s, under 100 MB, one core, no I/O.

## 5. Classification
runnable_local. A stdlib toy decides the cost-ratio question directly. run.py supports --smoke (m=3, below 1 s; smoke verdict is not a result).

## 6. What a run cannot establish
- Labelled [pcc:unverified]. Counts mulmods in dense elimination only; it does not test sparse filtering, Wiedemann/Lanczos, CADO-style merge, or the revisit condition (10^6+ columns, large-prime arm).
- Relations are synthetic (hidden logs known), so relation yield, collection cost, and real curve automorphism orbits (fixed points, non-free orbits) are not modelled.
- The 216 prediction is arithmetic (n^3 scaling); confirming it does not show an improvement over the IC baseline or any distance to matched rho, and the recorded exhausted_at_scale disposition is unchanged by it.
