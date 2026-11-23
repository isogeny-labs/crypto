# LEAD-RHO-native-step-cost

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: generic. Source: atlas_survey_20261107 --pursuit.
- Title: "Native cost of one rho step: canonicalisation, field arithmetic and admitted CPU timing".
- Next-step excerpt (truncated in leads.json): "Price the Koblitz canonicaliser in field-multiplication equivalents: run EXP-CERTBIN-b4d29c (des..."
- Context recorded in the survey: strong rho 1,196 -> 718 instr/step, floor ~6-7 field mults; constant-factor lead only (cannot change the exponent, Shoup).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines ~65-67, 118, 255). Other hits (atlas_tally.json, candidate_matches.json, README.md) only grepped. EXP-CERTBIN-b4d29c itself was not found, so the mechanism below is my own toy.

## 2. Hypothesis
On toy Koblitz curves y^2+xy=x^3+x^2+1 over F_2^m (m in 13..21, polynomial basis), a rho-style walk on Frobenius+negation classes (canonicaliser = min x over the m-orbit, sign by min y, cost m+1 extra squarings) has a net work ratio R_net = (steps_canon/steps_plain) * (counted cost per canon step / counted cost per plain step) below 1 for every priced cell with canonicaliser-squaring weight w_C in {0, 0.1} (mult-equivalents) and inversion weight in {3, 10}, at m >= 17. I.e. the canonicaliser's native price does not cancel the class-size gain at the unpriced 0 and 0.1 GAE levels.

## 3. Preregistered prediction and falsification criterion
- Statistic: R_net per (m, w_I, w_C) cell from median first-repeat step counts over 24 paired seeds and exact op counts (mul, sq, inv) per step.
- Prediction: max R_net < 1 over all cells with m >= 17 and w_C in {0, 0.1}. Outcome "consistent_with_prediction" if so; "falsified" if every such cell has R_net >= 1; otherwise "inconclusive". The w_C = 1 column is reported (section 5a band) but ordering is only claimed where it holds across the band.
- Null / negative control: the plain arm (no canonicaliser) is R_net = 1 by construction.
- A/A arm: plain vs plain on fresh disjoint seeds; median step ratio must lie in (0.7, 1.4) else the verdict is inconclusive.
- Positive control: canon/plain step ratio must be < 0.5 at m >= 17 (the detector sees the class-quotient gain); otherwise inconclusive.

## 4. Method
- Pure-Python GF(2^m) with op counters; irreducible polys x^13+x^4+x^3+x+1, x^15+x+1, x^17+x^3+1, x^19+x^5+x^2+x+1, x^21+x^2+1. Group order N = 2^m+1-V_m (Lucas sequence), roughly 2^12..2^21.
- Pool of 16 random multiples of a random point G, index = x & 15. Walk P <- P + T[x(P)&15] (affine add, 1 inv + 2 mul + 1 sq). Canon arm applies canon() after every add. Steps counted to the first repeated state (birthday proxy, includes fruitless cycles).
- Seeds: curve/pool/start seed 1000*m+i (i<24), shared by plain and canon arms; A/A seeds 5000000+1000*m+i; bootstrap seed 7. Cost: mul 1, sq 0.1 (priced base), inv in {3,10}, canonicaliser squarings weighted {0,0.1,1}.
- Cost: well under 1 minute estimated at full size, <50 MB RAM, single core.

## 5. Classification
runnable_local. Everything is a stdlib counted-operation toy; no timing or hardware dependence.

## 6. What a run cannot establish
- Not CPU timing, instruction counts, or the repository's EXP-CERTBIN design; counted ops only, polynomial basis only (normal-basis free Frobenius not modelled).
- First-repeat steps are a proxy: no DLP is solved, no distinguished points, and fruitless cycles in the x-only index walk shrink canon steps drastically (smoke at m=9 shows 5 vs 60), which inflates the apparent gain. A pass therefore only says the price does not erase an upper-bound gain.
- Toy m only; says nothing about K-163/K-283 or prime curves. Results are [pcc:unverified].
