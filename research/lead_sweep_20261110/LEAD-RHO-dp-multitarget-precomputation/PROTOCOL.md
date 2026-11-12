# LEAD-RHO-dp-multitarget-precomputation

## 1. Lead as recorded
- Disposition: closed_known (reviewed 2026-10-29). Family: generic. Source: atlas_survey_20261107 --pursuit.
- Title: Distinguished points, multi-target batching and Bernstein-Lange precomputation (incumbent controls).
- Next-step excerpt (truncated): "Revisit when: A table or batch scheme that beats the Bernstein-Lange l^{1/3}/l^{2/3} tradeoff or ..."
- Files read: research/lead_sweep_20261110/leads.json (entry); research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 140-145, 316-322: "Bernstein-Lange at equal advice is 50-500x faster" than the 13.9x/18.5x retained-setup IC ratios); research/prime_field_leads_20261031/L3_precomputation/atlas_update.md (8 lines; cites a REPORT.md with implied exponents, not read). The cryptanalysis repo was not searched. The atlas YAML is encrypted, so the full revisit condition is unread.

## 2. Hypothesis
The recorded obstruction is that a single-target DLP with free precomputation P and table size T has online cost about N/(T*D) (D = DP spacing), which is the Bernstein-Lange tradeoff. For a random-mapping r-adding walk on Z_N (N prime, 2^12..2^22) with T = D = N^(1/3), precompute is about N^(2/3) steps and the online mean step count scales as N^(1/3). The fitted log2-log2 slope of online steps against N lies in [0.25, 0.42]. The table-free two-sided DP rho slope lies in [0.42, 0.58]. No toy scheme here beats the tradeoff; the run checks that the incumbent behaves as recorded.

## 3. Preregistered prediction and falsification criterion
- Statistic: least-squares slope of log2(mean online steps) on log2 N over 6 sizes (bits 12,14,...,22), 150 trials per size per arm.
- Prediction: slope_BL in [0.25, 0.42]. Falsified if slope_BL is outside that band while all controls pass. That would mean the toy contradicts the recorded N^(1/3) online scaling, or a DP-overhead artefact dominates.
- Null (negative control): table-free two-sided DP rho, same walk and DP rule. Its slope must lie in [0.42, 0.58], which shows the instrument sees sqrt(N).
- Positive control: the same walk with an 8x larger table. Online mean must be lower than the 1x table at every size, which shows the instrument sees the effect of advice.
- A/A arm: the BL arm repeated with a second fixed seed (7777). The absolute slope difference must be < 0.08.
- Decision: if any control fails, the verdict is inconclusive. If controls pass and the slope is in band, the verdict is consistent_with_prediction. If controls pass and the slope is out of band, the verdict is falsified. The verdict is also reported with the tradeoff ratio online*T*D/N per size (expected roughly constant).
- Cost pricing at 0, 0.1 and 1 GAE (AGENTS.md section 5a) is not applicable. This is a toy step-count scaling test over one abstract group, not a cross-method comparison.

## 4. Method
- Group: Z_N, N = smallest prime above 2^bits, generator 1, secret x random in [1, N). Point p, tracked offset a (p = a + b*x, b in {0,1}).
- Walk: 16 fixed random jumps (seeded), index = ((p*2654435761)>>7)&15, p' = p + jump. DP: p mod D == 0, D = round(N^(1/3)). Walk cap 20*D steps, then discard or restart.
- Precompute: m = round(N^(1/3)) chains from random a0*G (b=0), table maps DP point to a. Online: restart from x+a0 (b=1) until the DP is in the table, then x = a_table - a, checked against the true x. The step budget is 60*sqrt(N).
- Null: alternate trails from aG and from xG+aG, store DPs, and solve on a cross-type collision.
- Seeds: 20261110 (main) and 7777 (A/A); per-size rng = seed*1000+bits. Python 3 stdlib only.
- Cost: about 6 sizes x 2 seeds x 150 trials x 3 arms, each trial at most a few thousand steps at 2^22. Estimated 1 to 4 minutes on one core, memory under 50 MB, no network.

## 5. Classification: runnable_local
A stdlib toy decides the recorded scaling directly (a cheap re-derivation of the closed bound). Smoke test: `python3 -I run.py --smoke` passes in under 1 s.

## 6. What a run cannot establish
- It does not show that no better table or batch scheme exists, nor reopen the lead. The revisit condition needs a scheme beating the l^(1/3)/l^(2/3) tradeoff.
- Z_N with additive jumps has no group-operation cost, inversion batching, negation or automorphism structure, so there is no real ECDLP cost model.
- Toy sizes, a fixed walk family and a heuristic random-mapping assumption. The slope band is a preregistered judgement and small sizes carry DP-overhead bias.
- Results are [pcc:unverified] assertions: no replay certificate, validator report or VALIDATION.json, and they cannot enter the leaderboard.
