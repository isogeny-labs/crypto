# LEAD-RHO-best-of-candidates-indegree
## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24); family: generic; source atlas_survey_20261107 --pursuit.
- Title: "Step to the better of k candidate images (best of P+R, P-R) to concentrate in-degree".
- Next-step excerpt (truncated): "Build best-of-P+-R into the gpu/nist kernel at W = 16-32 walks per thread with the kernel's own ..."
- Recorded numbers (atlas_survey.md A5): steps 1.129 [1.081,1.184], net per-step 0.95 at 1024 walks/inversion, 1.03-1.12 predicted net at W 8-64; constants only, cannot change exponent (Shoup).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 118, 246); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep only).
## 2. Hypothesis
In a toy additive walk on Z_N (N ~ 2^18), replacing the r-adding step x -> x+a_j by the better (lower pseudo-random hash) of the two candidates x+a_j, x-a_j reduces the mean number of steps to the first repeated point by a factor in (1.05, 1.30), i.e. consistent with the recorded 1.129 step ratio, independent of GPU kernel details.
## 3. Preregistered prediction and falsification criterion
Statistic: R = mean steps(plain) / mean steps(best-of-2), with 95% bootstrap CI (400 resamples), at N ~ 2^18 (and 2^14 reported).
- consistent_with_prediction: CI(R) lies within (1.05, 1.30) and the A/A CI contains 1.
- falsified: upper CI(R) < 1.03 (no in-degree concentration benefit in steps).
- inconclusive: otherwise, or if the A/A arm excludes 1 (noise floor broken).
Controls: A/A = plain walk with a different seed stream (expected ratio 1). Null/negative control = plain r-adding walk (no selection). Positive control = ideal random function x -> hash(x) mod N, reported as plain_over_randfn (expected near 1; shows the instrument tracks sqrt(pi N/2)). Selection is by hash of the candidate, so each map is a deterministic function of the point.
## 4. Method
Python stdlib. Per trial: fresh 32-entry random jump table in [1,N), random start, step index = mix(x)&31; walk until a visited point repeats (dict of seen points); count steps. Arms: plain, best (compare mix(x+a,s2) vs mix(x-a,s2)), aa, randfn; seeds 101, 202, 303, 404 (bootstrap seeds 1, 2). Sizes N = 2^14+1, 2^18+1; 1500 trials per arm per size. Cost: about 1500*4*(~700 steps) per size, well under 2 minutes, memory < 30 MB. Cost note: best-of-2 doubles candidate evaluations; only the step ratio is measured, not net per-step cost.
## 5. Classification
runnable_local: a stdlib toy decides the step-ratio question directly.
## 6. What a run cannot establish
Toy additive group Z_N with a hash, not elliptic-curve points; no distinguished points, no parallel-walk or batched-inversion cost, no GPU/W=16-32 behaviour, no net speed-up (the 0.95 per-step figure is untested), no exponent change. Result is [pcc:unverified], descriptive only.
