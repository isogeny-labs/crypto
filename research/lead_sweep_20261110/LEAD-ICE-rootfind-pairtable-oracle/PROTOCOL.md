# LEAD-ICE-rootfind-pairtable-oracle

## 1. Lead as recorded
- Disposition: open_blocked (last reviewed 2026-10-29). Family: ic-engine. Source: atlas_survey_20261107 --pursuit.
- Title: Root-finding / pair-table decomposition oracle in place of a Groebner solve.
- Next-step excerpt (truncated in leads.json): "Price every native primitive of the rootfind-negfold m = 2 arm and of matched strong rho (table..." Survey completes it: price both arms at 0/0.1/1 GAE, second-host replay, re-run 17-bit Koblitz and 20-bit prime cold sessions. Cost: medium. Blocker: problems/rootfinding-complete-cost-calibration.md.
- Recorded numbers: prime Table B 5.47x-13.5x rho at 9.7-23.4 bits; Koblitz 17-bit about 1.54x. Atlas reading: ratio > 1 across the band means the pair table is exhausted at toy scale.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 37-60, 100-160, 285-290). Other hits (atlas_tally.json, candidate_matches.json) only grep-checked. Nothing in /home/user/cryptanalysis was searched (budget); the problems/ file named above was not read.

## 2. Hypothesis
For prime-order subgroups of toy prime-field curves (group order n = 2^12..2^20), a complete m=2 negation-folded pair-table index calculus (B = ceil(2 n^(1/3)) x-small factor base, all +-F_i +-F_j sums in a hash table, incremental random walk of targets, sparse elimination mod n) costs MORE than negation-fold rho at every size and every native-primitive price w in {0, 0.1, 1} GAE, and log2(IC/rho) grows with log2 n (95% bootstrap CI of the slope above 0), i.e. the exponent gap widens, as the atlas records.

## 3. Preregistered prediction and falsification criterion
- Statistic: R_w(b) = median IC cost / median pooled rho cost at size b, with cost = group additions + w * native ops (table insert/lookup, mulmod in elimination, DP test/canonicalise/index, sqrt for the base); fixed walk-table setup is charged to both. Also S_w = regression slope of log2 R_w on bits, bootstrap 95% CI (200 resamples over replicates).
- Prediction: R_w(b) > 1 for all b, all w, and CI lower bound of S_w > 0 for all w.
- Falsified: R_w < 1 at both of the two largest sizes for any w, provided controls pass. Consistent: prediction holds and controls pass. Otherwise inconclusive (including any control failure or fewer than 2 sizes completed in the 540 s guard).
- Null control: measured relation yield per trial divided by the random-table model (table points in subgroup / n, doubled for +-); pass band [0.3, 1.6] (conservation, KN-FIND-007: no yield beyond random). Failure marks the instrument, not the lead.
- Positive control: every run must recover the planted d (verified dP=Q); and the detector must see native pricing: IC cost at w=1 over w=0 at the largest size > 1.05. Also a free-table IC arm is reported (fraction of cost) to show the table dominates.
- A/A: two rho arms with different derived seeds on the same instances; |median log2 ratio| <= 0.3.
- Order of arms is shuffled per instance with a derived seed.

## 4. Method
- Python 3 stdlib only, single core, fixed string seeds (inst/walk/arm/ord-<bits>-<rep>, boot). Sizes bits in {12,14,16,18,20} (p of that bit length, p = 3 mod 4), 5 replicates each; curves y^2=x^3+ax+b found by naive point counting, #E = h*n with n prime, h <= 8; P of order n, Q = dP.
- Rho: 16-entry adding walk, canonical-sign negation fold, distinguished points at x with t = max(1, bits//2 - 4) low zero bits, restart at 20*2^t steps; collision gives d, verified.
- IC: base of B points with small x, pair table of all distinct +-F_i +-F_j sums, target R walks by random adding of 16 fixed points, hit if R or -R in table; relation f-coefficients plus b*d = -a; sparse incremental elimination with smallest-column pivots, stop when the d pivot appears; d verified.
- Cost estimate: counting at 2^20 about 5-10 s per curve, elimination and table under a minute; total expected 2-5 minutes, hard deadline 540 s, memory under 100 MB. Smoke: sizes 10, 12, 2 replicates, under 1 s (verified).

## 5. Classification
runnable_local. Stdlib toy decides the sign and trend of the priced ratio at toy sizes; the repository's own cold sessions are not needed for this sharpening.

## 6. What a run cannot establish
- Not the recorded Table B or Koblitz numbers: different implementation, affine arithmetic, unoptimised constants, so absolute ratios differ; only sign and trend are tested.
- Prime-order subgroup with cofactor h>1 means base points are not in the subgroup (yield lower by about 1/h), a known bias against IC.
- Native pricing here counts only the listed primitives; it is not the lab's sealed pricing, no second-host replay, no validation gate or matched-rho band, so nothing can enter the leaderboard. Five replicates, five sizes: slope CI is wide.
- Says nothing about degree-bounded algebraic oracles or any asymptotic exponent. All results are [pcc:unverified].
