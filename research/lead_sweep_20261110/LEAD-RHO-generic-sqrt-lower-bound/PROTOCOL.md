# LEAD-RHO-generic-sqrt-lower-bound

## 1. Lead as recorded
- disposition: closed_obstructed (atlas_tally_20261101; last_reviewed 2026-10-24); family: generic.
- title: "Beat the square-root exponent with any generic collision algorithm". next_step_excerpt: empty in leads.json.
- Recorded obstruction (survey): theorem, Shoup/Nechaev; generic arms fit alpha <= 0.53, bsgs.negation 0.529 [0.485,0.596]; with advice S, S*T^2 >~ N.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 63-67, 136-137);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 43-48).

## 2. Hypothesis
In an opaque-label (generic) cyclic group of prime order N, with cost counted as group-law oracle calls, both BSGS and a distinguished-point r-adding rho
solve the DLP with a log-log cost slope alpha whose 95% bootstrap CI contains 0.5, and neither uses fewer than 0.5*sqrt(N) calls on average at any size
2^12..2^22. I.e. the recorded obstruction (Omega(sqrt N)) is reproduced by a counted toy, and no sub-sqrt generic arm appears.

## 3. Preregistered prediction and falsification criterion
- Statistic: alpha = least-squares slope of log2(mean ops) on log2 N over sizes 2^12,2^14,...,2^22 (24 seeds/size), 95% CI by bootstrap over seeds (400 resamples);
  plus ratio = mean ops / sqrt(N) per size.
- Falsified if, for either generic arm, CI upper bound of alpha < 0.45 OR any size has mean ratio < 0.5 (a sub-sqrt generic algorithm in the toy;
  would indicate a harness leak, since the theorem forbids it, so it would first trigger a leak audit, not a claim).
- Consistent if both CIs contain 0.5 and the controls pass. Otherwise inconclusive.
- Positive control: "clear" solver that reads the label (labels = cleartext exponents, non-generic): must give alpha CI upper < 0.3 (detector can see sub-sqrt cost).
- Null/negative control: the opaque-label Feistel encoding itself is the null (no readable structure); there is no separate null population.
- A/A arm: BSGS rerun with a disjoint seed family B; |alpha_A - alpha_B| must be < 0.1, else inconclusive.
- Controls failing => verdict inconclusive regardless of arms.

## 4. Method
- Group Z_N, N = largest prime <= 2^k, k in {12,14,16,18,20,22}. Labels are a keyed 4-round Feistel permutation (cycle-walking) of the exponent;
  algorithms only call add(l1,l2) (counted), hash/equality of labels. Key, secret x, and walk constants derive from fixed string seeds "lsw20261110-..." via random.Random.
- BSGS: m=isqrt(N)+1 baby steps (table), giant step -m*g via counted double-and-add, search until hit.
- Rho: 8-adding walk (small-exponent steps, setup counted), distinguished points with dbits=floor(log2(N)/2)-3, jump-restart after each DP, collision solves for x. Cap 60*sqrt(N)+1000 ops (failures are counted and reported).
- 24 seeds per size, 4 arms (bsgs, rho, bsgs_AA, clear). Cost: pure Python, estimated well under 2 minutes, < 50 MB (BSGS table ~2^11 entries at 2^22). Single core, stdlib only.

## 5. Classification
runnable_local. A stdlib toy can reproduce the recorded bound's scaling; run.py implements it (`--smoke` for trivial sizes).

## 6. What a run cannot establish
- It does not prove the Shoup/Nechaev bound (a lower bound over all algorithms); two algorithms and one encoding cannot.
- Toy sizes 2^12..2^22: the rho slope carries lower-order bias (setup, DP overhead, heavy-tailed trails); the CI is wide. Prime-order Z_N with a random labelling is not an elliptic curve; says nothing about curve-specific (encoding-reading) methods, which the theorem excludes by construction.
- No advice/preprocessing (S*T^2 >~ N) arm, no negation map, no matched-rho GAE pricing (AGENTS.md 5a), no replay certificate or validation gate: not a leaderboard or benchmark row. Result labelled [pcc:unverified].
