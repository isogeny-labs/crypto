# LEAD-TRF-cm-canonical-lift-heights

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (atlas_tally_20261101; last_reviewed 2026-10-24). Family: transfer.
- Title: "Serre-Tate canonical lift with CM: does the CM order shrink the heights of lifted points enough for xedni to bite?"
- next_step_excerpt: empty in leads.json. Recorded obstruction (atlas_survey.md A4/item 20): "cost_model; twelve audited L9 mechanisms, Pr[Q in red(S)] <= |S|/N"; "CM canonical lift heights closed over twelve audited mechanisms".
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58, 156); research/prime_field_leads_20261031/C1_lift_torsion/atlas_update.md (head); grep of research/ecdlp_subexp_desk_20261101/data/atlas_tally.json. The atlas YAML and the twelve L9 mechanism notes were not available.

## 2. Hypothesis
For the CM curve E: y^2 = x^3 + 17 over Q (rank 2, independent points (-2,3), (-1,4)) and primes p = 1 mod 3 with #E(F_p) = N in 2^12..2^18, let S be the set of reductions of rational points a*P1 + b*P2 with |a|,|b| <= k, closed under the CM automorphism group mu_3 (x -> w x). CM gives no extra coverage beyond that orbit factor: for a uniformly random Q in E(F_p), Pr[Q in S] = |S|/N (no enrichment), where |S| is the actual distinct count (already including the factor up to 3 from mu_3, and sign symmetry from the set). Equivalently, the recorded bound Pr[Q in red(S)] <= |S|/N is tight and CM structure does not make small-height lifts cover more than a random set of equal size.

## 3. Preregistered prediction and falsification criterion
- Statistic: per prime, hit rate h = fraction of 20000 uniform random Q (x-uniform, random y sign) lying in S; pooled ratio R = sum h / sum(|S|/N); Stouffer z of (h - |S|/N) over primes with binomial sd.
- Prediction: R <= 1.25 and Stouffer z < 4 (no enrichment). Falsified if z > 4 AND R > 1.25 (CM-lifted small-height points are enriched in the reduction, i.e. the bound is not the operative obstruction). Inconclusive if a control fails.
- Null/negative control: a random point set of identical size per prime (expected hit rate |S|/N).
- Positive control: Q drawn from S itself must hit with rate exactly 1.0 (detector sees membership); all primes must pass.
- A/A arm: two independent Q streams (different derived seeds) against the same S; Stouffer |z_AA| <= 4 required.
- Decides: consistent -> the closure (no height-shrinking benefit visible as coverage) stands at toy size; falsified -> reopen the lead for a structural reason (coverage above |S|/N).

## 4. Method
Pure stdlib. Primes: first 6, 5, 4, 2 primes p = 1 mod 3 above 2^12, 2^14, 2^16, 2^18. N by brute-force counting. k chosen so |S| ~ 0.1 N (roughly, using 6(2k+1)^2 ~ 0.4 N before collisions). S built by affine group-law additions mod p plus mu_3 images. Seeds: seed = p for set building and null set; Q streams use 1000*p + {1,2,3,7}. Cost: well under 1 minute-ish CPU expected (point counting O(p) dominates), RAM < 100 MB, one core.

## 5. Classification: runnable_local
A stdlib toy decides it; run.py with --smoke passes.

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
Reduction of a subgroup combination is a homomorphic image, so |S|/N coverage is nearly a tautology; the test checks only that nothing unexpected (e.g. correlated structure with mu_3 orbits) arises at toy size. It does not examine actual canonical heights, Serre-Tate lifts, non-CM or large-rank curves, xedni's rank-lowering step, or the twelve audited L9 mechanisms, and says nothing at cryptographic size or about beating rho. Only one curve and rank 2.
