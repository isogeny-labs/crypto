# LEAD-ICP-gfp5-rho-reference-audit
## 1. Lead as recorded
- Disposition: closed_known (last reviewed 2026-10-24). Family: ic-prime. Source: atlas_survey_20261107 --pursuit.
- Title: Rho ruler and SafeCurves-style parameter audit for EcGFp5/EcMasFp5 (affine F_{p^5} rho cost, inversion paths, partition, multi-instance, interval kangaroo).
- Next-step excerpt (truncated): "Revisit when: A deployment with narrow nonce intervals or many keys on one curve, or a walk-mixi..."
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only, lines 33-61, 140-174, 230-314); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (one entry). No dedicated lead note found; the rest is from knowledge of the mechanism.
## 2. Hypothesis
The rho reference such an audit relies on is sound at toy scale: an r-adding walk with r=20 on a cyclic group of prime order N, with hashed-point partition, reaches its first repeated point in mean steps within 10% of the random-map birthday value sqrt(pi N/2) for log2 N in {12,16,20}. A poorly mixing walk (r=2) exceeds it by >15%, so the instrument can see mixing defects.
## 3. Preregistered prediction and falsification criterion
- Statistic: ratio = mean steps to first revisit / sqrt(pi N/2), 300 trials per size.
- Prediction: |mean(ratio r=20, arms A and B) - 1| <= 0.10 at every size.
- Falsified if any size exceeds 0.10 with the controls valid. This would mean the matched-rho ruler carries a walk-mixing excess at toy scale, reopening the "walk-mixing" revisit condition.
- Null control: ideal random map (uniform resampling); must satisfy |ratio-1| <= 0.15.
- Positive control: r=2 walk; must be >1.15x the r=20 ratio, otherwise the verdict is inconclusive.
- A/A arm: two independent seeds of the r=20 walk; |A-B| <= 0.15, otherwise inconclusive.
- Does not test: inversion paths, multi-key, interval kangaroo (the other revisit conditions).
## 4. Method
Generic cyclic group Z_N (P=1, Q=d), N in {4093, 65521, 1048573}. Walk x <- x + M[h(x) mod 20], h(x)=(x*2654435761)>>8, M random multiples a*P+b*Q. Start is random; count steps until a point repeats (dict). Seeds are fixed: 20261110*1000+10*log2N+arm. Cost: about 300*1300*4 steps at the largest size, so well under a minute and a few MB. Python stdlib only.
## 5. Classification
runnable_local. A stdlib toy re-derives the recorded rho ruler cheaply.
## 6. What a run cannot establish
It uses a generic group, not F_{p^5} curve arithmetic, so it says nothing about inversion cost, SafeCurves parameters, or real EcGFp5 orders. It is a toy-scale mixing check only, not a distinguished-point or multi-instance result. Results are labelled [pcc:unverified]. It is not an index-calculus improvement.
