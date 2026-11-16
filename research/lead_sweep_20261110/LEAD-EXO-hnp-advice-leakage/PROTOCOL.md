# LEAD-EXO-hnp-advice-leakage

## 1. Lead as recorded
- Disposition: open_blocked (reviewed 2026-10-24). Family: exotic. Source: atlas_survey_20261107 --pursuit.
- Title: Bit-security / hidden-number conversion of DL leakage, auxiliary inputs and advice.
- Next-step excerpt (truncated in leads.json): "Price the Thue-Morse/Rudin-Shapiro conversion of H-ECDLP-a4be60 (IDEA-20260904-f68c7f: exponent ..."
- No next_step_cost or why_it_might_beat fields are present in the leads.json entry.
- Recorded result (atlas_survey.md line 74): "Cheon aux-input walk exponent 0.23 vs 0.50 ONLY if [alpha^i]G leaked".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-74, 281-283, grep context only); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/candidate_matches.json (excerpt). atlas_tally.json hit not read. No cryptanalysis-repo search was run. The lead's own YAML is encrypted, so the Thue-Morse conversion itself was not read.

## 2. Hypothesis
H: Given the leaked auxiliary points G, aG, a^d G with d | n-1 and d ~ sqrt(n), Cheon's algorithm recovers a in a prime-order group with a fitted exponent of scalar multiplications in log2(n) of at most 0.35 (theory 1/4, the recorded 0.23). Matched generic rho and no-advice BSGS on the same groups fit at least 0.42 (theory 1/2). The recorded 0.23 vs 0.50 gap is therefore a property of the leak, not of a leak-free ECDLP. Without [a^i]G the speedup does not exist.

## 3. Preregistered prediction and falsification criterion
- Statistic: least-squares slope of log2(median operation count) against bits, over n ~ 2^12..2^22 (6 sizes, 25 random (G, a) per size, median per size).
- Prediction: Cheon slope <= 0.35 and rho slope >= 0.42 and BSGS-null slope >= 0.42, with zero recovery failures and zero wrong answers.
- Decides: consistent_with_prediction if all hold. falsified if the Cheon slope > 0.35 (the toy does not show the recorded exponent). inconclusive if any recovery fails, if A/A or rho/BSGS controls break, or if --smoke.
- Positive control: Cheon recovery is verified against the secret (a*G == recovered*G) in every trial. This shows the instrument sees the advantage.
- Negative control / null: no-advice BSGS (d = 1) on the identical (n, G, a). It must keep the 0.5 slope, so the instrument does not manufacture a speedup.
- A/A arm: rho against rho with independent walk seeds on the same instance. The median ratio of medians must lie in [0.7, 1.4].
- Pricing (AGENTS.md section 5a band): Cheon steps are scalar multiplications. They are reported at 1 group op each (ratio_cheon_rho_unit) and at log2(n) group ops each (ratio_cheon_rho_logn). The slopes are insensitive to this. The constant-factor crossover at toy size is reported but not decided on. The 0 / 0.1 / 1 GAE pricing of unpriced native work is not applicable: the groups are abstract Z_n and no native field arithmetic is counted.

## 4. Method
- Group: Z_n (additive, n prime). The algorithms use only addition, scalar multiplication and equality tests on points. rho uses only the point value x mod 16 for its partition, so no discrete log is exploited.
- n is chosen per size as a prime near 2^bits (up to 60 candidates) with a divisor d of n-1 closest to sqrt(n), d >= 4. g is a primitive root.
- Cheon stage 1: zeta = g^d has order k = (n-1)/d. Find m with zeta^m G = a^d G by BSGS of sqrt(k) baby and giant steps. Then a = g^(m+k u).
- Stage 2: find u in [0, d) by BSGS with rho0 = g^k, using sqrt(d) steps.
- rho: Floyd, 16-way additive walk, 3 group adds per iteration, restart on a degenerate collision. BSGS null: ceil(sqrt n) baby steps plus giant steps.
- Seed 20261110, with all sub-seeds derived from it. Sizes bits = 12, 14, 16, 18, 20, 22.
- Cost: factoring by trial division up to 2^11, at most 60 primality tests per size, about 25 * 3000 rho steps per size. Estimated under 2 minutes of one core, under 50 MB of RAM. Limit 10 minutes.

## 5. Classification: runnable_local
The Cheon toy is decidable with the stdlib. Pricing the Thue-Morse/Rudin-Shapiro conversion itself would need the encrypted lead text and is out of scope. This run only re-derives the recorded exponent gap.

## 6. What a run cannot establish
- Nothing about any real curve. It is an abstract-group, generic-model toy with exponents fitted from six sizes. The slope error is wide (about +/-0.05).
- It does not show that [a^i]G leaks anywhere. The recorded conditional ("ONLY if leaked") is assumed, not tested.
- It does not test the Thue-Morse/Rudin-Shapiro conversion or H-ECDLP-a4be60.
- It does not test the HNP lattice side (bit leakage).
- A consistent verdict re-confirms known Cheon theory. It is not an improvement over the index-calculus baseline and does not count as one.
- All results are labelled [pcc:unverified].
