# LEAD-ICE-hybrid-guess-determine-l7

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24); family: ic-engine; source atlas_survey_20261107 --pursuit.
- Title: One-variable specialisation priced as hybrid guess-and-determine on the l = 7 descent cells.
- Next step (truncated in leads.json): "Run the registered stretch cell E8, (m, n, l) = (3, 22, 8): find k* (fixed variables under rule ..." The rest of the rule is not available here.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 47-48, 221-223: groups this lead with mostly binary GF(2) descent engine leads; quotable: descent closure degree 4 through (m,l)=(4,6), degree 5 at (3,7) capped at 169 GB). atlas_tally.json and the cryptanalysis atlas-inputs JSONs only list the id. The lead's own note was not found.
- Mechanism (own knowledge): Semaev summation polynomial S_{m+1} over GF(2^n) is Weil-descended to GF(2) in m*l variables; fixing k of them (guess) lowers solving degree at 2^k repetitions.

## 2. Hypothesis
For a determined random quadratic GF(2) system (n variables, n equations, planted solution; a stand-in for the descended system), the guess-and-determine cost 2^k * cols(D_min(k))^2.37 has a minimiser k*(n) >= 1 for every n in {12, 14} (hybrid beats pure solving), where D_min is the smallest XL degree (<= 4) whose Macaulay matrix yields all n-k variables. Falsified form: k* = 0 at all n >= 12 (the specialisation never pays).

## 3. Preregistered prediction and falsification criterion
- Statistic: k*(n) = argmin over k in 0..4 of the median over 4 seeds of 2^k * cols^2.37 (cost infinite if unsolved at D <= 4).
- Prediction: k*(12) >= 1 and k*(14) >= 1, with the A/A arm's k* within 1 of the main arm's.
- Decides: consistent_with_prediction if both hold and controls pass; falsified if k* = 0 at both n >= 12; otherwise inconclusive.
- Positive control: an all-linear system (3n equations) must be solved at D = 2. Negative control/null: an underdetermined system (n/2 equations) must be unsolved at D <= 4 for every seed. A/A arm: the same configuration with a disjoint seed stream (offset 7777); k* must agree within 1.
- Failure of either control forces "inconclusive".

## 4. Method
Python 3 stdlib only. Random quadratic system over GF(2) (each monomial of degree <= 2 included with probability 1/2, constant fixed to satisfy a planted x0). Guessing fixes the first k variables to their planted values (the correct guess; the 2^k enumeration factor is charged, not run). XL with field equations x^2 = x: rows are equation * squarefree monomials of degree <= D-2, columns degree-ordered; Gaussian elimination with Python big-int bitsets; solved when n-k independent rows lead on degree-1 columns. Sizes n in {10, 12, 14}, k in 0..4, D <= 4, 4 seeds per cell, base seed 20261110 (+1000n+s; A/A +7777; null +5; positive +9). Cost estimated at 1-3 minutes, RAM far below 200 MB. `--smoke`: n = 8, Dmax = 3, 2 seeds.

## 5. Classification: runnable_local
The registered cell E8 (m,n,l) = (3,22,8) needs Groebner/F4 engines and large memory and is not reproducible here. The stdlib toy does decide the generic question the lead's pricing rests on (does a hybrid optimum k* > 0 exist), so it is runnable_local as a proxy.

## 6. What a run cannot establish
- Nothing about the Semaev/Weil-descent systems at (3,22,8): random quadratic systems lack their structure (symmetry, sparsity, degree of S_4).
- XL at degree <= 4 is not F4/F5; the exponent 2.37 and the guess correctness (cost counts 2^k, not 1/2 success) are modelling choices; sizes are tiny (n <= 14) and censoring at D = 4 can inflate k*.
- Proves no speed-up over rho. Results are labelled [pcc:unverified].
