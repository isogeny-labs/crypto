# LEAD-STR-class-representative-ic-selection

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: structure. Source: atlas_survey_20261107 --pursuit.
- Title: Choose the isogenous curve or isomorphic model that minimises charged index-calculus cost.
- Next-step excerpt (truncated in leads.json): "Run a preregistered ecbench IC study with three arms. (a) Floor instance solved on the floor. (b ..." Full text per atlas_survey.md: (b) carried to crater with transport charged, (c) best-of-K isomorphic models on held-out targets; 24-32 bits, l=2 depth 3; success = about 4% S net of transport. Cost: est. medium. Why it might help: ~4% S constant factor; IC/rho still 17-170x.
- Recorded numbers: l=2 floor/crater yield 0.942 [0.928,0.956], S floor/crater 1.045 [1.023,1.067] at 24 bit; H4 model effect tau 7.2%.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 50-54, 99-101, 256-258 via grep context only); README.md of the same directory matched by grep (line 224, not read further). Hits not read: ecdlp_subexp_desk atlas_tally.json, session logs, L1 atlas_update.md. Nothing in cryptanalysis beyond atlas-input JSON hits (not read).

## 2. Hypothesis
Among isomorphic models (x -> u^2 x, u = 1..K) of a prime-order prime-field curve, with the factor base defined as the points whose model x-coordinate is below B = p/16, the size-normalised two-point decomposition yield (distinct sums P+Q covered, per unordered pair of FB points) shows a best-of-K gain no larger than that obtained when each model's FB is replaced by a uniformly random point subset of the same size. I.e. the recorded model effect is FB-size fluctuation and carries no exploitable structure beyond what a size-matched random FB gives.

## 3. Preregistered prediction and falsification criterion
- Statistic: S_obs = mean over curves of (max_u y_u / mean_u y_u - 1), y_u = distinct sums / (n_u(n_u+1)/2), n_u = |FB_u|.
- Null / negative control: same statistic with each FB_u replaced by a random subset of the same size (20 replicates, seeds 100..119). Threshold q99 of null.
- Prediction: S_obs <= null q99 -> consistent_with_prediction (model selection has no size-normalised structural yield gain; the ~4% is not structural). S_obs > q99 -> falsified (a model-dependent structural gain exists; justifies the ecbench three-arm study).
- Positive control: FB = consecutive multiples of one generator G of the same size; must show mean yield below 0.5x that of random FB. If not detected, verdict is inconclusive.
- A/A arm: 6 further null replicates with independent seeds (900..); reported mean and range, must overlap the null band (noise floor); not used for the verdict.

## 4. Method
Python stdlib. Prime p = 2027 (smoke p = 211). Six random curves y^2=x^3+ax+b with prime group order N (about 2^11), curve sampling seed 20261110. K = 12 models. B = p//16. Point enumeration by square table; affine group law with pow(.,-1,p); coverage by set of all P+Q, P,Q in FB (FB closed under negation as x is shared). Obs seed 1, positive control seed 2. Cost estimate: about 25M group ops, a few minutes, under 100 MB, one core.

## 5. Classification
runnable_local. A toy yield statistic can sharpen the question (whether model choice has structure beyond FB size). It does not replace the 24-32 bit ecbench study, which needs the repository launchers.

## 6. What a run cannot establish
Not an ecbench S measurement: no relation-phase linear algebra, no transport cost, no isogeny crater/floor (only isomorphic x-scalings, which on a fixed curve is a relabelling of x), no matched rho band, no replay certificate. Two-point decomposition at N~2^11 says little about Semaev m=3 at 24-32 bits. Note that x -> u^2 x on a fixed curve only changes which points count as small; results are labelled [pcc:unverified] and cannot enter the leaderboard.
