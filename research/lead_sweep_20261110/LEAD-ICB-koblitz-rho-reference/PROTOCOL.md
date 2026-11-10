# LEAD-ICB-koblitz-rho-reference

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24). Family: ic-binary.
- Title: "The matched Koblitz rho reference for binary IC rows: step pricing, signed-Frobenius canonicalisation, a 131-bit walk".
- Next-step excerpt (truncated in leads.json): "Run the strong rho fixture and ParallelRho on the same Table A targets in the same process and p..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 210, one-line listing only); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/candidate_matches.json (title match only). No lead body text found; the mechanism below is from the title plus general knowledge (Koblitz curves, Wiener-Zuccherato / Gallant-Lambert-Vanstone equivalence classes).
- The full next step (Table A targets, strong rho fixture, ParallelRho, 131-bit walk) needs repo harnesses and large targets, not available here.

## 2. Hypothesis
On the Koblitz curve E: y^2+xy=x^3+x^2+1 over GF(2^m), m in {11,13,17}, a Pollard rho walk (16-way additive, Floyd) that canonicalises each point to the minimum of its 2m-element Frobenius-and-negation orbit (tracking the scalar multiplier +/-lambda^j mod n) needs fewer group-step evaluations than the negation-only walk by a factor R_m = mean(neg)/mean(frob) of at least 0.6*sqrt(2m). That is, at least 60% of the ideal sqrt(2m) saving survives fruitless cycles and restarts. This is the quantity a matched Koblitz rho reference must price in.

## 3. Preregistered prediction and falsification criterion
- Statistic: R_m (ratio of mean f-evaluations to a verified solve over 200 shared targets, per m), with a 95% bootstrap CI (500 resamples, seed 20261209).
- Prediction: R_m >= 0.6*sqrt(2m) for all three m, i.e. >= 2.81, 3.12, 3.50 for m = 11, 13, 17.
- consistent_with_prediction: all point estimates meet the threshold and all controls are valid.
- falsified: controls valid and, for some m, the upper CI limit of R_m is below the threshold.
- inconclusive: any control invalid, or the threshold lies inside a CI.
- Negative control / null: the "none" arm (no canonicalisation) against the "neg" arm. Positive control: negation-only must give mean(none)/mean(neg) in [1.1, 1.8] (ideal sqrt2), proving the instrument sees a known orbit-size effect.
- A/A arm: "neg_AA", the same negation-only algorithm with a different derived seed stream; mean(neg)/mean(neg_AA) must have a 95% CI containing 1.
- Validity also requires zero failed solves (every returned k is checked by k*G = Q) in the frob arm.
- Interleaving: the four arms run on each target in turn, with per-(m,target,arm) derived seeds.

## 4. Method
- Field: GF(2^m) by exp/log tables on primitive polynomials 0x805 (m=11), 0x201B (m=13), 0x20009 (m=17). Curve a=1, b=1, group order N = 2^m+1-V_m (Lucas V_0=2, V_1=1, V_k=V_{k-1}-2V_{k-2}). n is the largest prime factor of N (cofactor 2). The code asserts N*P = infinity for a random point P.
- G = (N/n)*P. lambda is found by brute force as the root of l^2-l+2 = 0 mod n with Frob(G) = l*G.
- Arms: none (raw point), neg (canonical rep of {P,-P}), neg_AA, frob (min over 2m Frobenius images and their negations; the multiplier is applied to the (a,b) coefficients).
- Walk: 16 random R_i = u_i*G + v_i*Q, index from a hash of the canonical x. Floyd tortoise/hare, 3 f-evaluations per iteration. On a degenerate collision (db = 0) or a cap of 180*sqrt(n) evaluations, restart from a new random start; evaluations accumulate across restarts.
- Targets: Q = k*G with k derived from seed 20261110, m and target index. T = 200 targets per m. Smoke mode: m in {7,11}, T=4, always labelled inconclusive.
- Cost: pure Python, single core. Estimated 1 to 5 minutes (the frob arm dominates, 2m canonicalisations per step); memory under 30 MB (m=17 tables). No network, no S3, no launchers.

## 5. Classification: runnable_local
A stdlib toy decides the quantity: the achieved saving of signed-Frobenius canonicalisation in a rho walk on Koblitz curves of group order ~2^10..2^17. `python3 -I run.py --smoke` passes (0.04 s).

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
- It does not reproduce the lead's actual next step: Table A targets, the strong rho fixture, ParallelRho, a 131-bit walk, or step pricing in GAE or the native counted unit. It does not provide the matched rho reference for any leaderboard row.
- Toy sizes (m <= 17, small n) cannot show asymptotic behaviour. The R_m threshold of 0.6 is a preregistered choice, not derived. Floyd's constant and the fruitless-cycle rate depend on the 16-way partition and the restart rule.
- Native cost per step (canonicalisation of 2m images against one point addition) is not priced here; step counts are not time. Per AGENTS.md §5a, any cross-method claim still needs the 0 / 0.1 / 1 GAE pricing band.
- No improvement over index calculus is shown by this run, and it is not a validation-gate artefact.
