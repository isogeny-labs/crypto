# LEAD-EXO-gag-holant-semaev-chain

## 1. Lead as recorded
- disposition: open_next_step; family: exotic; title: (empty in leads.json); source: atlas_survey_20261107 --pursuit.
- next_step_excerpt (truncated in leads.json): "Two paper falsifiers and one toy falsifier. (A) Show contraction-only algorithms are GGM-simulab..."
- Atlas survey line: "predicted falsifiers A,B,C; Cai-Chen dichotomy predicts none". Sibling lead holant-decomposition-count-network (closed_dominated): QTT rank at p=1021 is 1024 (ceiling) under x-labelling, 1023-1024 under random relabelling, 109-121 under DL-labelling.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context, lines 71-74, 180-186). The only other hit, docs/kb-retrieval-queries.json, was not read. Full next-step text (falsifiers A, B) is not available here.

## 2. Hypothesis
Toy falsifier C (my reading, since the excerpt is cut off): the Holant network of Semaev-style summation constraints over a factor base is a tensor whose cut matrix has no x-coordinate-labelling compressibility. Claim: for prime-order E(F_p), p in 151..251, the middle-bond rank (over GF(1000003)) of the one-step decomposition-count tensor M[x(P)][x(P+R)] = #{R in FB}, under binary-digit labelling by x, is at least 0.95 times the rank under random relabelling. Only the DL labelling, which needs the secret, compresses it.

## 3. Preregistered prediction and falsification criterion
- Statistic S = median over 8 curves of rank_x / mean(rank_rand1, rank_rand2).
- Consistent_with_prediction: S >= 0.95 and controls valid. Falsified: S < 0.9 and controls valid (x-labelling compresses, so a contraction-only algorithm gains from public labels). Otherwise inconclusive.
- Negative control / null: two random injective labellings of the same class set (seeds s+1, s+2).
- Positive control: DL labelling (class index of kG), a known compressor (p=1021 precedent 109-121 vs 1024). Valid iff median rank_dl / rank_null <= 0.7.
- A/A arm: rank_rand1 / rank_rand2, median; valid iff |A/A - 1| <= 0.03.
- If either control fails the verdict is inconclusive.

## 4. Method
- For each curve (seed 1000+i, p in {151,163,181,199,211,223,241,251}), draw random y^2=x^3+ax+b with prime group order N, a random generator G.
- Classes = x-classes {+-kG} plus infinity (about N/2 total). Factor base FB = all points with x < max(3, p/8) (both signs).
- M[a][b] = #{R in FB : class(P_a + R) = b}.
- Label each class with 8 bits (hi 4 bits, lo 4). Reshape to a matrix with rows (hi_i, hi_j) and columns (lo_i, lo_j), 256 x 256 (the middle TT bond, ceiling 256). Rank by Gaussian elimination mod 1000003.
- Cost: about 4 ranks x 8 curves, each about 256^3/3 operations, so a few minutes of single-core stdlib Python (estimated, the full run was not executed), memory under 50 MB. Group orders are about 2^7.5, below the 2^12 floor, because dense rank is cubic. This is a limit on the claim.

## 5. Classification
runnable_local. A stdlib toy can decide the labelling question. Falsifiers A and B are paper/literature items and are not covered.

## 6. What a run cannot establish
- Nothing about falsifiers A and B (GGM-simulability of contraction-only algorithms, Cai-Chen dichotomy), whose text is truncated here.
- Not asymptotic: orders about 2^8, one-step tensor only, one cut, one labelling family (binary digits of x), one fixed FB shape. No claim about an actual chain of S3 constraints or full contraction cost, and no cost in GAE or rho-matched comparison (AGENTS.md section 5a).
- Rank is a structure statistic, not a cost. A null result does not rule out other labellings or decompositions.
- Results are labelled [pcc:unverified].
