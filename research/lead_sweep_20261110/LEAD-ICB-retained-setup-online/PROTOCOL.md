# LEAD-ICB-retained-setup-online

## 1. Lead as recorded
- Disposition: closed_dominated (last reviewed 2026-10-24). Family: ic-binary.
- Title: Retained-setup online IC on Koblitz curves (n = 61, 71, 73, 83): online ratio after an uncharged precomputation.
- Next-step excerpt (truncated in leads.json): "Revisit when: An online IC whose S*T^2 lies at or below r/|Aut| (a non-generic oracle), or advic..."
- Recorded evidence (second-hand, via the status report): precomputed rho / IC online is 3.74x-396x in rho's favour at N83; no row admitted.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; grep hits in /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 308 only), /home/user/crypto/docs/status/IC_STATUS_REPORT_20261031.md (lines 52-55); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json located but not read. The full next_step text is unavailable (atlas YAML encrypted).

## 2. Hypothesis
The recorded obstruction is the Bernstein-Lange tradeoff: for a generic group of order r, any online solver after an uncharged precomputation with S stored points and online time T satisfies S*T^2 >= c*r with c of order 1. Falsifiable claim: in a generic toy walk-and-table model, min over configurations of S*T^2/r is >= 0.5, while a non-generic online IC (retained factor-base logs, smoothness oracle charged 1 op per trial) reaches S*T^2/r <= 0.25. So the revisit condition is met only by a non-generic oracle, never by generic table rho.

## 3. Preregistered prediction and falsification criterion
- Statistic: R = S*T^2/r, with T = mean online group steps per solved instance (restarts included) and S = stored distinguished points.
- Prediction: min R over 9 generic cells x 2 independent seed sets >= 0.5; IC positive-control R <= 0.25; A/A max ratio-of-ratios <= 2.0.
- consistent_with_prediction: all three hold. falsified: generic min R < 0.25 (a generic table beats r/|Aut| with |Aut|=1, contradicting the recorded obstruction). Otherwise inconclusive.
- Negative control / null: the generic walk-table solver itself (it carries no structure, so R should sit at the BL floor).
- Positive control: toy index calculus in Z_p^* (p ~ 2^18, 150 primes, logs retained), a non-generic oracle that the detector must see below threshold.
- A/A arm: the same generic cell run twice with different seeds (A vs A2), measuring noise. Sampling noise is large at 60 trials, so a wide A/A gap makes the run inconclusive.

## 4. Method
- Generic cell: group Z_r (P=1), r prime near 2^14, 2^16, 2^18. Walk x -> x + m[h(x) mod 32]; the 32 multipliers are fixed and independent of the target. Distinguished point (DP) iff h(x) mod W == 0, W = max(2, 0.5*sqrt(r/S)), walk cap 20*W. S = max(4, f*2^(lr/3)) with f in {0.5, 1, 2}. Precompute (uncharged) builds a table of S DPs mapping x to the a in x = aP. Online: for 60 random k, Q = kP, start at Q + cP, walk to a DP, hit the table to recover k = a_t - a_o, check it, else restart. Group-law steps only; no division by the structure.
- IC control: p = next prime >= 2^18, generator g, discrete logs of the first 150 primes by full table (uncharged setup). Online: random e, test whether h*g^e is 150-smooth (charged 1 op), then k = sum - e mod p-1, verified. 40 targets.
- Seeds: A=1001+r, A2=2002+r, IC=3003. Random(seed) throughout, deterministic.
- Cost: about 10^6 to 10^7 Python steps in total, a few minutes at most (a 420 s guard stops the config loop), under 60 MB RAM (the IC dict holds ~262k entries, freed before the online loop).

## 5. Classification: runnable_local
A stdlib toy can sharpen the recorded tradeoff and check that the detector separates generic from non-generic oracles. It does not touch Koblitz curves.

## 6. What a run cannot establish
- Nothing about Koblitz n=61..83, Frobenius/|Aut| orbit reduction, or real IC cost. The toy group is additive Z_r and generic only by discipline.
- The IC control is a multiplicative-group smoothness oracle, an existence proof that non-generic oracles beat the bound, not evidence that one exists for ECs.
- Thresholds 0.5/0.25 are preregistered heuristics, not derived constants. 60 trials per cell give wide intervals. The smoke test only checks that the code runs.
- Not a replay-certified or validated result. All outputs are [pcc:unverified].
