# LEAD-ICB-factor-base-subspace-choice

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: exhausted_at_scale (last reviewed 2026-10-30; disposition_20261101 same). Family: ic-binary.
- Title: Choice of the factor-base subspace V at fixed dimension (geometric progression, dim(V.V), trace rank, cosets, greedy coverage, HW pools, trace-parity window).
- Next-step excerpt (truncated in leads.json): "Revisit when: A property of V that predicts N_var or yield with a null-controlled effect that gr[ows with size]".
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 77, 303); docs/session-logs/2026-11-01.md (line 1410: "coordinate/subspace effects vanish at scale"). research/ecdlp_subexp_desk_20261101/data/atlas_tally.json matched but not read. Nothing in cryptanalysis was searched beyond the task scope.

## 2. Hypothesis
At fixed dimension d, no structured choice of the F2-subspace V of GF(2^m) (geometric progression, minimal dim(V.V) among 30 draws, trace-0 window) changes the toy 3-sum relation yield (unordered pairs P,Q with x(P),x(Q) in V and x(P+Q) in V) over a random d-dimensional subspace, beyond noise. This is the recorded obstruction (effects vanish at scale), re-tested at m=16, d=8.

## 3. Preregistered prediction and falsification criterion
- Statistic: per curve, log of (mean normalised yield of structured family / mean normalised yield of random V), normalised by |S|^3 (S = points with x in V); then t = mean/se of that log-ratio across 12 random curves (a in {0,1}, random b).
- Prediction: for all three structured families t <= 4.0 (one-sided, above random).
- Falsified: any structured family has t > 4.0 while A/A is within |t| <= 4 and the positive control is valid. Decides: the lead's recorded obstruction does not hold at this toy size, so the lead merits reopening at larger m.
- Consistent_with_prediction: all t <= 4.0. Decides nothing beyond "no toy-scale effect"; does not reopen the lead.
- Inconclusive: A/A |t| > 4 (noise floor/instrument unsound) or positive control enrichment < 5.
- Negative control / null: random_null, independent random d-dim subspaces (40 per curve).
- A/A arm: random_AA, same generator, independent draws (8 per curve), compared with random_null.
- Positive control: V = subfield GF(2^8) with b in the subfield, a=0; sums stay in E(F_{2^8}), so the closure fraction must exceed random landing probability by >= 5x.

## 4. Method
Pure-Python GF(2^16) via exp/log tables (primitive polynomial found by search). Curve y^2+xy=x^3+ax^2+b. Points with x in V by solving z^2+z = x + a + b/x^2 with a precomputed table (y = xz); both y and y+x included. A pair gives x3 = lam^2+lam+a+x1+x2, lam=(y1+y2)/(x1+x2); count if x3 in V. Parameters: m=16, d=8, 12 curves, 40 null + 8 instances per structured family and A/A per curve, master seed 20261110 (single random.Random stream). Expected |S| about 256, about 33k pairs per V, about 300 V total times 12 curves... estimated 2-6 min, one core, memory < 50 MB, no network. `--smoke` uses m=10, d=4, 3 curves (<1 s).

## 5. Classification
runnable_local. A stdlib toy decides the question at toy size; it re-tests the recorded obstruction cheaply.

## 6. What a run cannot establish
- Toy field size (m=16) says nothing about the large-scale yield or N_var of real binary-curve Semaev/Weil-descent index calculus; the atlas result concerns scale.
- The yield proxy is a 3-sum pair count, not the real decomposition (m-ary summation polynomials, large-prime or linear algebra cost); no gap to rho is measured.
- Only three of the atlas's property families are tested (not cosets, greedy coverage, HW pools).
- A null result is not a proof of exhaustion; a positive result is not an improvement over the IC baseline. Results are labelled [pcc:unverified].
