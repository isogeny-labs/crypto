# LEAD-ICB-cofactor-torsion-quotients

## 1. Lead as recorded
- disposition: open_next_step (leads.json); the 2026-11-01 desk recorded closed_obstructed (research/ecdlp_subexp_desk_20261101/data/candidate_matches.json).
- family: ic-binary. last_reviewed 2026-10-24. source: atlas_survey_20261107 --pursuit.
- title: Spending the cofactor and 2-torsion of binary curves: quotient labels, k-tree filters, 2-torsion translation quotients of the factor base.
- next-step excerpt (truncated in leads.json): "A curve with a large smooth cofactor, or a non-homomorphic sum-compatible filter with more than ..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; only one-line mentions in
  research/prime_field_lead_assessment_20261107/atlas_survey.md (line 204, same excerpt) and
  research/ecdlp_subexp_desk_20261101/data/candidate_matches.json (lines ~374-377). The sessions logs
  2026-10-27 and 2026-11-07 mention the id only. No full lead text was found; the mechanism below is
  reconstructed from the title and general knowledge (every binary curve has a rational 2-torsion point T=(0,sqrt b)).

## 2. Hypothesis
For 2-sum index calculus on binary curves y^2+xy=x^3+x^2+b over GF(2^m), m in {12,14,16,18}, with a
negation-closed factor base F of size about sqrt(N)/2, replacing the decomposition target R by the
quotient class {R, R+T} gives a per-test yield gain g = |S u (S+T)| / (2|S|) (S = set of sums f1+f2) that does
not exceed that of a size-matched random factor base by more than 0.03. That is: the 2-torsion quotient
doubles the labelled yield per trial but costs two tests, and a subspace (x-coordinate) factor base has no
structural affinity to translation by T (x -> b/x is not F2-linear), so the lead's quotient yields no net gain.

## 3. Preregistered prediction and falsification criterion
- Statistic: paired per-seed difference D = g(structured FB) - g(random FB), mean and 2-standard-error CI
  per m (3 curves x 10 seeds each).
- Prediction: for every m, |mean D| + CI <= 0.03 (null effect).
- Falsified: for any m, mean D - CI > +0.03 (structured FB gains from the quotient beyond random), provided controls pass.
  A real gain would be a measured per-test improvement, to be re-checked on larger sizes in phase 3.
- Negative control / null: random negation-closed FB of equal size (g falls with density as about 1 - density/2).
- Positive control: T-closed FB (F = F+T), where S+T is within S so g is about 0.5; detector must give
  mean(g_random - g_Tclosed) - CI > 0.1 at every m.
- A/A arm: two independent structured FBs on the same curve and seed stream; |mean diff| <= CI + 0.01.
- If a control fails the verdict is "inconclusive". Otherwise falsified / consistent_with_prediction / inconclusive as above.

## 4. Method
- GF(2^m) via log/exp tables (primitive polys 0x1053, 0x4443, 0x1002D, 0x40081, asserted primitive).
- Curve a=1, b = Random(1000*m+curve).randrange; enumerate all points exactly (x, y via z^2+z table), N = #E.
- FB sizes about 2^k points with k = round(log2(sqrt(N)/2)) (structured: all points with x in a random
  k-dim F2-subspace; random and T-closed arms size-matched). Sums of all unordered pairs computed exactly,
  so yields are exact densities (no sampling of R). Seeds: Random((m*100+ci)*1000+si).
- Sizes: group orders about 2^12..2^18 (not cofactor-engineered: cofactor is >= 2 from T). Large smooth
  cofactor and non-homomorphic k-tree filters (the excerpt's open conditions) are NOT tested here.
- Cost: stdlib only, one core, estimated 1-3 minutes, < 100 MB RAM (m=18 tables of 2^19 ints).
- Command: `python3 -I run.py` prints one JSON object; `--smoke` runs m=12 only.

## 5. Classification: runnable_local
A stdlib toy decides the cheap sub-question (does the 2-torsion translation quotient of a subspace FB
yield anything over random). Smoke test passed.

## 6. What a run cannot establish
- Nothing about asymptotic exponents, m=83 or larger curves, or the real index-calculus cost with linear algebra and labels.
- Not the excerpt's actual conditions (large smooth cofactor, sum-compatible non-homomorphic filters with
  more than the truncated bound): the full lead text was unavailable.
- Only 2-sum decompositions and x-subspace factor bases; other bases (Semaev summation polynomials with
  different filters, k-trees) are untested. Density is fixed at about 1/8.
- Exact densities on toy sizes are small-sample and a null here does not close the lead. All results are [pcc:unverified].
