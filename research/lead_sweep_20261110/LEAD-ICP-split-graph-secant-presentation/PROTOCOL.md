# LEAD-ICP-split-graph-secant-presentation

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step. Family: ic-prime. Title: empty in leads.json. Reviewed 2026-11-01.
- Next step (truncated): "Implement native exact triangular factorization or an FGLM-style zero-dimensional root extractor". Cost: est. medium (native extractor + 3-cell prospective panel).
- Why it might beat sqrt(p): "Degree d+1 vs 2d+1 is real, but no complete-cost gain over dense presentation yet".
- Recorded evidence: factor base from rational intersections with low-degree graphs (Boolean offsets); secant presentation has homogeneous first-vanishing degree d+1 vs 2d+1; complete toy candidate used fewer rows than the nongraph control but more than its own dense presentation; 26 F127/d5 extraction caps censored.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 16-30, 85-96, 241-243 via grep); research/prime_field_lead_assessment_20261107/README.md (grep lines 134-139, 300-306). Nothing from cryptanalysis was searched.
- Desk assessment in README: conservation and the curve-free degree law predict no complete-cost gain; "run it with the nongraph, low-x and hash-x controls and let it close".

## 2. Hypothesis
Conservation (KN-FIND-007 as cited): for a same-size base, an affine-cube ("Boolean offsets") x-coordinate graph base has the same m=2 sum coverage (fraction of group elements expressible as P1+P2, P_i in base) as a uniformly random base of equal size B. So the graph base gives no relation-yield gain independent of the presentation degree. This is a proxy for the extractor-independent half of the lead's discriminator; it does not test the d+1 vs 2d+1 degree claim.

## 3. Preregistered prediction and falsification criterion
- Statistic: z = (coverage_arm - mean_null)/sd_null, null = 300 uniformly random size-B point subsets on the same curve (B = floor(sqrt(N))+1, so ~N/2 unordered pairs).
- Prediction: max over curves of |z_graph| < 3.
- Falsified if max |z_graph| >= 4 (graph base differs from random; with controls valid). Consistent if < 3. Between 3 and 4: inconclusive. Falsification would only motivate the native extractor; consistency supports closing the lead's cost-gain branch.
- Negative control/null: random size-B bases (above). Also low-x base and hash-x base (the atlas's named controls) reported alongside.
- Positive control: arithmetic-progression base {P0,2P0,...,BP0} (additive structure); must reach |z| >= 5 on every curve, else the verdict is inconclusive.
- A/A arm: one fresh random base scored against the same null; must have |z| <= 3.5, else inconclusive.

## 4. Method
Python 3 stdlib only. Seed 20261110 (+ prime size). Curves y^2=x^3+ax+b over F_p, p = next prime above {4000, 16000, 60000, 250000}, random (a,b); all points enumerated, N = #E. Graph base: x in {c + sum eps_i w_i : eps in {0,1}^k}, k=ceil(log2 B)+1, c,w_i random, points over these x sorted by x and truncated to B (arm skipped if fewer than B points). Coverage = |{P_i+P_j, i<=j}|/N via affine addition. Cost: <= ~1 min total, <1 GB-free, tens of MB (the p=250000 point list). `--smoke` runs one tiny curve with 8 reps; its verdict is not meaningful.

## 5. Classification
runnable_local. The coverage test is a stdlib toy. The lead's actual next step (native triangular/FGLM extractor, three-cell prospective panel) is not run here.

## 6. What a run cannot establish
Nothing about Groebner/F4 degree, extraction cost, or complete same-base field ops per independent relation, the lead's real discriminator. Coverage at m=2 on toy curves says nothing about exponents. A null result does not exclude other graph choices (non-affine, higher degree). Results are [pcc:unverified].
