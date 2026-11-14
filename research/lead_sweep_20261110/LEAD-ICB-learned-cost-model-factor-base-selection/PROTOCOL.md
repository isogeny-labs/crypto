# LEAD-ICB-learned-cost-model-factor-base-selection

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-29). Family: ic-binary. Source: atlas_survey_20261107 --pursuit.
- Title: Learned (gated, log-linear) cost model that selects the factor-base subspace at fixed dimension on binary and Koblitz toy fields, and its transfer across curves.
- Next step (truncated excerpt): "Extend the frozen gated ridge beyond the measured range with the same protocol and nulls: l >= 6".
- Files read: research/lead_sweep_20261110/leads.json (entry); research/prime_field_lead_assessment_20261107/atlas_survey.md (line 213, one-line listing only). Other hits (atlas_tally.json, candidate_matches.json, ATLAS_INPUTS.json) are index files, not read. The frozen ridge's features and the original protocol were not found, so this protocol is a stand-in, not the frozen model.

## 2. Hypothesis
A gated log-linear ridge model, trained only on factor-base subspaces of dimension l in {3,4,5} of GF(2^13) on one binary curve, ranks the relation yield of held-out subspaces of dimension l in {6,7,8} on a different curve with mean within-l Spearman rho >= 0.5 and above the 95th percentile of the permuted-label null. Yield is the number of distinct points P+Q (P, Q distinct factor-base points) divided by the number of pairs.

## 3. Preregistered prediction and falsification criterion
- Statistic: mean over l in {6,7,8} of Spearman(predicted, measured log yield) on the transfer curve, first draw (A1).
- Threshold: >= 0.5 and > null p95.
- Verdicts: consistent_with_prediction if both hold and the controls are valid; falsified if controls are valid but either fails; inconclusive if a control is invalid.
- Null: ridge refit on training labels permuted (100 permutations, seed 7), evaluated on the same held-out data.
- Positive control: ridge fitted in-sample on the test data; Spearman must be >= 0.5 (shows the features can see the effect).
- A/A arm: a second independent draw of test subspaces (A2); |A1 - A2| mean Spearman must be <= 0.2 on both curves.
- Baseline reported, not gated: rank by factor-base size alone. If the baseline matches the ridge, the learned model adds nothing beyond counting points.

## 4. Method
- Field GF(2^13), polynomial x^13+x^4+x^3+x+1, log/exp tables. Curves y^2+xy=x^3+x^2+b; b = the two smallest b >= 3 with Tr(b)=1 (curve A and curve B).
- Factor base: all affine points with x in a random F_2-subspace V of dimension l; half of the draws contain 1 (gate).
- Features: 1, log|FB|, fraction of V with trace 0, gate (1 in V), gate times trace fraction. Ridge lambda 1e-2, normal equations in pure Python.
- Sizes: 36 subspaces per (curve, l, draw); the training set has 108 rows on curve A. Seed 20261110 for training; test draws use seeds derived from l, curve and draw.
- Cost: pairs up to about 2^15 per l=8 subspace, a few million group additions in total, expected about 1 to 4 minutes, under 50 MB. Standard library only.

## 5. Classification
runnable_local. A stdlib toy decides the question. It extends the range to l >= 6, but with a stand-in feature set on GF(2^13), not the frozen model.

## 6. What a run cannot establish
- It does not reproduce the frozen gated ridge, its features or its nulls, which were not recovered.
- The toy field is non-Koblitz, with a pair-sum yield proxy instead of real Semaev or index-calculus relation yield.
- It does not test selection at cryptographic sizes, and it says nothing about beating rho (no matched rho, no section 5a band, no replay certificate).
- Two curves and small l limit transfer evidence. Results are labelled [pcc:unverified].
