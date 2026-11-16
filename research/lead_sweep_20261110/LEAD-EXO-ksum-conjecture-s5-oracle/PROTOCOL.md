# LEAD-EXO-ksum-conjecture-s5-oracle

## 1. Lead as recorded
- disposition: open_next_step; family: exotic; source: atlas_survey_20261107 --pursuit; title: (empty in leads.json).
- next_step_excerpt (truncated): "Write the reduction as a cost_model/literature obstruction, conditional on the k-SUM conjecture,"
- Atlas gloss: "conditional obstruction under k-SUM; Kane-Lovett-Moran caveat"; README: "covers every free-S_5-oracle proposal in one row"; argues no structureless oracle can beat generic cost. The atlas notes the lead still cites "R ~1.3 flat" from the dl-character-sum lead, stale after the symmetric-null re-read (inference).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry, no cost/why fields); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-74, 187; grep context only); .../README.md (lines 44, 202-203 grep context). No hits in cryptanalysis. Knowledge base not queried (not available to this subtask).

## 2. Hypothesis
If the Semaev S_k relation test is a structureless oracle (FB values behave as independent uniform elements of a group of order N), the cheapest generic k-SUM meet-in-the-middle relation phase (table of floor(k/2)-subsets, scan of the rest, B relations needed, plus B^2 linear-algebra charge, minimised over B) costs N^e(k) with e(3)=1, e(4)=e(5)=2/3, e(6)=3/5, i.e. always above the rho exponent 1/2. So a free S_5 oracle gives no sub-sqrt(N) relation phase without extra structure.

## 3. Preregistered prediction and falsification criterion
- Statistic: least-squares slope of log2(mean over 3 seeds of min-over-B cost) against log2 N, N prime at 12,14,...,22 bits, per k.
- Prediction: k=5 slope in [0.55, 0.80] (analytic 2/3); k=6 slope >= 0.5.
- Falsified: k=5 slope < 0.55 (controls valid), or k=6 slope < 0.5. Consistent: k=5 slope in [0.55,0.80], k=6 >= 0.5, controls valid. Otherwise inconclusive (including invalid controls or slope > 0.80, which would only show the toy grid is too coarse).
- Negative control / null: the unstructured arm itself (uniform FB) is the null; k=3 (slope ~1) and k=4/6 show the predicted ordering.
- Positive control: planted-structure arm, k=5, FB values drawn from [-N^0.25, N^0.25] (a structured oracle); must give slope < (k=5 slope - 0.10), proving the instrument sees a sub-generic cost.
- A/A arm: k=5 rerun with disjoint seeds (111,112,113 vs 11,12,13); |slope difference| < 0.08 else inconclusive.
- Comparison to matched rho is not made here: the cost unit is table/scan operations, not priced GAE (AGENTS.md 5a band not applied). Only the exponent is compared with 1/2.

## 4. Method
- Group Z_N (N prime), FB of B uniformly random residues (seeded strings "main-k-bits-seed"). Relation = a disjoint a-subset and b-subset (a=k//2, b=k-a) with a-sum + b-sum = 0 mod N. Scan stops when B relations are found; cost = table size + b-subsets scanned + B^2. Cap 4e5 ops per (N,B) cell; capped cells discarded.
- B grid: round(N^e), e = 0.15, 0.18, ..., 0.45 (11 values); minimum cost taken per (N, seed).
- k in {3,4,5,6}, plus k=5 A/A and k=5 planted arms; bits 12..22 step 2; seeds 11,12,13 (A/A: 111..113 via +100 offset in code: seeds+100 gives 111,112,113).
- Cost: pure Python dict MITM, estimated 1-5 minutes total on one core, < 200 MB (tables <= 4e5 entries); wall guard well under 10 minutes.
- Command: `python3 -I run.py` (full), `--smoke` for the 3-size check.

## 5. Classification
runnable_local. The stdlib toy decides whether the measured exponent matches the analytic MITM ladder, which is the quantitative core of the conditional obstruction.

## 6. What a run cannot establish
- It does not prove the k-SUM conjecture or any lower bound; MITM is one algorithm, and Kane-Lovett-Moran style algebraic-decision-tree caveats (and Wagner k-tree with extra list size) are not covered.
- Toy sizes 2^12..2^22 with a coarse B grid; fitted slopes carry finite-size bias.
- Uniform-random FB ignores real S_5 geometry (curve structure, small-x sets, automorphisms), and the planted arm only shows detector sensitivity.
- Unit-cost operation counts are unpriced; no matched-rho comparison or Section 5a band. Results are labelled [pcc:unverified].
