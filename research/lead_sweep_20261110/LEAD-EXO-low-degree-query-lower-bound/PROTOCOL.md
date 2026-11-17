# LEAD-EXO-low-degree-query-lower-bound
## 1. Lead as recorded
- disposition: open_next_step; family: exotic; title: (empty in leads.json); source: atlas_survey_20261107 --pursuit.
- next_step_excerpt (truncated): "Steps (a) desk and (b) exact rank are done (reports ecdlp-leads-desk-20261104, lowdeg-rank-20261..."
- Atlas survey (research/prime_field_lead_assessment_20261107/atlas_survey.md, lines 74, 192; README.md line 198): H1 (T >= sqrt(N)/D for degree-D algebraic queries) withdrawn, false under a full-rank condition, checked exactly at p = 251-2039, 39/39 cells; H1' proved with Delta (description length) in place of D. Remaining: a description-length or local-work-charged bound, a complexity question no GGM argument decides.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only); .../README.md (grep context only). The lowdeg-rank report itself was not found or read.
## 2. Hypothesis
The recorded obstruction (low-degree functions do not have bounded distinguishing power on E(F_p)) has a closed-form cause: the evaluation matrix of the Riemann-Roch space L(D*O) = span{x^m, x^m y} (pole orders 0,2,3,...,D, dimension D) on the N-1 affine points of E(F_p) has rank exactly min(D, N-1). Hence degree alone gives no lower-bound leverage until D approaches N, and any bound must charge Delta (description length), as H1' does.
## 3. Preregistered prediction and falsification criterion
- Statistic: number of cells where rank mod p of the evaluation matrix differs from min(D, N-1).
- Threshold: 0 mismatches in the main arm (12 random curves, p in [2039,4001), D in {8,16,32,48}). Any mismatch: falsified (the exact-rank cause is wrong or the recorded 39/39 is mis-explained). Zero mismatches: consistent_with_prediction, which supports only the rank claim, not any query bound.
- Positive control: tiny curves (p<71) with D = N+5 > N-1 must show rank capped at N-1 (detector sees rank deficiency).
- Negative control / null: planted dependent columns (y^2-x^3-ax-b, and a copy of x) must leave rank at 12 for D=12 (detector sees non-independence).
- A/A arm: independent seed, same statistic and instrument; must also give 0 mismatches.
- Any control failure: inconclusive.
## 4. Method
Stdlib Python. For each cell: pick prime p, random a,b with nonzero discriminant (random.Random seeds 20261110 main, +1 A/A, +2 positive, +3 negative), enumerate affine points via a square table, build the N-1 by D matrix, compute rank mod p by Gaussian elimination. Cost: about N*D^2 <= 4000*2300 per cell, well under a minute total, <50 MB RAM, one core.
## 5. Classification: runnable_local
A cheap exact re-derivation of the recorded rank obstruction; stdlib toy decides it. Note the theoretical expectation is a theorem (a nonzero function in L(D*O) has at most D zeros, so is nonzero on points once D < N-1), so a pass is a sanity re-derivation, not new evidence.
## 6. What a run cannot establish
It says nothing about adaptive query tuples, GGM lower bounds, Delta-charged or local-work-charged bounds, or large N. Different basis (x-degree only, vs pole order) from the recorded 39/39 check, so it does not reproduce that exact instrument. Results are labelled [pcc:unverified].
