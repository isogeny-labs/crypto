# LEAD-ICP-construction-normal-form

## 1. Lead as recorded
- disposition: open_unexamined (last reviewed 2026-10-24); family: ic-prime; source: atlas_survey_20261107 --pursuit.
- title: "Normal form and calibrated instruments for proposed prime-field attack systems (dedup, result classes, estimator bias, dynamic range)".
- next-step excerpt (truncated in leads.json): "Compute the H-PFDR-6189d0 normal form over every PFDR/PATH proposal and hypothesis, and run the ..."
- atlas_survey.md item 18: "a normal-form dedup / estimator-bias check (indirect). No evidence yet."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 30-44, 274-288 only). Hits not read: README.md in the same dir, research/ecdlp_subexp_desk_20261101/data/atlas_tally.json. H-PFDR-6189d0 and the PFDR/PATH proposal texts were not found or read, so the full normal form cannot be computed here; this protocol tests only a toy instance of its "dedup + estimator bias" part.

## 2. Hypothesis
Toy stand-in: a proposed attack system is specified by a curve (a,b) over F_p and a factor base (FB) rule. (i) Naive dedup by (a,b) counts about (p-1)/2 or more distinct "proposals" per isomorphism class, while the normal form (isomorphism class) counts one. (ii) An FB rule that is not isomorphism-invariant (x-coordinate interval after the scaling x -> u^2 x) gives a two-point decomposition coverage (|{F1+F2}|/|E|) that varies across isomorphic copies, but behaves like a random FB of the same size: var_iso / var_randomFB lies in [0.5, 2]. So a normal form must include the FB rule, and an estimator that ignores this has an irreducible dispersion that equals that of random FBs.

## 3. Preregistered prediction and falsification criterion
- Statistic R = pooled within-curve variance of coverage across isomorphic copies with interval FB, divided by pooled within-curve variance of coverage for random x-set FBs of equal size B.
- Consistent: 0.5 <= R <= 2. Falsified: R < 0.1 (interval FB effectively invariant, so no FB term is needed in the normal form) or R > 10 (FB rule imposes structure far beyond random). Otherwise inconclusive.
- Null / negative control: random x-set FBs of size B (the null for R's denominator).
- A/A arm: a second independent set of random-FB draws; ratio of the two variances must be in [0.5, 2], else the verdict is inconclusive.
- Positive controls: (a) one FB x-set transported along every isomorphic copy gives coverage variance exactly 0 (detector sees invariance); (b) FB size increased by 25% must raise mean coverage by more than 3 standard errors (detector sees an effect).
- Outcome decides only whether the toy FB-rule dependence is real; it does not decide any PFDR/PATH proposal.

## 4. Method
Python 3 stdlib only. Primes p in {2039, 4093}; 3 random curves y^2=x^3+ax+b per prime (seed 20261110, one random.Random stream); order by point enumeration. For each curve, 40 distinct squares t=u^2; interval FB = points whose x satisfies (t*x mod p) < B, B = p // 128 (both signs of y); coverage = distinct sums F1+F2 (i<=j) over the order. Random FB: B x-values sampled uniformly. Naive dedup count = distinct (a t^2, b t^3) over the copies versus 1 normal-form class. Cost: about 720 coverage computations of roughly 500 pair-additions each; well under a minute, below 50 MB.

## 5. Classification
runnable_local. Everything above is stdlib arithmetic at group orders about 2^11 to 2^12. Limit: the actual H-PFDR-6189d0 normal form needs the PFDR/PATH proposals, which were not located in the bounded search.

## 6. What a run cannot establish
- It says nothing about H-PFDR-6189d0, the PFDR/PATH proposals, result classes or dynamic range; only a toy dedup/dispersion analogue was tested.
- m=2 coverage at p about 2^12 says nothing about asymptotic relation yield or beating rho.
- Isomorphic copies share one group, so "invariance" for the transported FB is by construction (a sanity check of the detector, not a finding).
- Few curves and fixed seed; the thresholds were fixed by me before running, not derived from a power analysis.
- Results are labelled [pcc:unverified].
