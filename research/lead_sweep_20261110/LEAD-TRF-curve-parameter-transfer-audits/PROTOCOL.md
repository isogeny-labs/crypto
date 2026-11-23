# LEAD-TRF-curve-parameter-transfer-audits
## 1. Lead as recorded
- Disposition: closed_known (last reviewed 2026-10-29). Family: transfer.
- Title: Transfer-safety audits of deployed curves: embedding degree, anomaly, twist, subfield/descent, seed provenance.
- Next-step excerpt: "Revisit when: an audit finds a deployed curve with small k, trace one, or a composite-degree bin".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 326, same excerpt only); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (index entry only). No other material found.
## 2. Hypothesis
The recorded obstruction is that generic prime-field curves are transfer-safe: among random curves over F_p, the rate of small embedding degree (1<k<=6, with respect to the largest prime factor r of #E) and of trace one (#E=p) is no larger than the rate predicted by treating p mod r as a uniform element of (Z/r)*. So an audit of non-adversarial parameters yields no revisit trigger.
## 3. Preregistered prediction and falsification criterion
- Statistic: z = (hits - expected)/sqrt(expected), pooled over two arms; expected = sum over curves of sum_{d<=6, d|r-1} phi(d)/(r-1).
- Prediction: z <= 3 (consistent). z > 3 falsifies the null heuristic (a hidden structure beyond chance).
- Negative control / null: the uniform-element expectation above (exact, per curve).
- Positive control: supersingular y^2=x^3+x, p=3 mod 4, must all be flagged k<=2 with #E=p+1. Any miss makes the verdict inconclusive.
- A/A arm: two arms with identical procedure, different seeds; |z_AA| > 3 makes the verdict inconclusive.
## 4. Method
Stdlib Python. Primes in [1500,2500); per arm 1200 random nonsingular curves y^2=x^3+ax+b (seeds 20261110 and 20261111). #E by Legendre sum (about 2000 steps per curve), r by trial division, k by iterated multiplication mod r. Anomalous (#E=p) counted separately. Cost: about 2.4M inner steps, well under 1 minute, small RAM. `--smoke` uses p in [200,400) and 40 curves.
## 5. Classification
runnable_local. A stdlib toy can test the null heuristic in seconds.
## 6. What a run cannot establish
It says nothing about deployed curves, twists, descent or seed provenance. The p is about 2^11 and the orders are tiny, so asymptotic claims do not follow. The hit rate is low, so power is limited. The null model is a heuristic. Results are [pcc:unverified].
