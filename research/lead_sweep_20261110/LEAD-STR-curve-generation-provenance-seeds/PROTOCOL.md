# LEAD-STR-curve-generation-provenance-seeds

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: exhausted_at_scale (last reviewed 2026-10-30). Family: structure.
- Title: Manipulated generation of standard curves (NIST P-curve seeds, Brainpool, BN, rigidity).
- Next-step excerpt (truncated in leads.json): "Revisit when: The historical generator code or logs are recovered, or a new publicly testable we..."
- Recorded obstruction (atlas_survey.md line 54, atlas_tally.json line 152): all ten NIST prime seeds explained; 21 public weak-class screens found nothing; a hidden weak class would need density >= ~2^-30 and be invisible to those screens.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context, lines 51-55);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep context only).

## 2. Hypothesis
Recorded obstruction, toy re-derivation: seed grinding against a hidden predicate of density 2^-6 (a keyed hash of the seed, independent of curve arithmetic) costs a geometric number of trials with mean 2^6 (within 15%), and the resulting curves are statistically indistinguishable from random-seed curves on five public screens (prime order, not anomalous, embedding degree > 30, twist large factor, order large factor). That is, public screens cannot see a hidden class, so a clean screen result bounds only visible weaknesses.

## 3. Preregistered prediction and falsification criterion
- Statistic: two-proportion z for each of the 5 screen pass rates, ground arm G (n=2000) vs random arm A (n=3000); mean grinding trials.
- Threshold: |z| < 3.3 on all 5 screens (Bonferroni-ish, 5 screens, alpha ~0.05) and |mean_trials/64 - 1| < 0.15.
- Outcomes: all hold -> consistent_with_prediction (screens are blind to the hidden class at this size). Any |z| >= 3.3 in G vs A -> falsified (grinding leaks into public screens, contradicting the recorded invisibility). Controls failing -> inconclusive.
- Null/negative control: random-seed curves (arm A) are the null population for G.
- A/A arm: A vs A2 (disjoint seed range, same instrument); must also satisfy |z| < 3.3 on all screens, else inconclusive.
- Positive control: arm W grinds seeds for anomalous curves (#E = p), a visible weakness; the not_anomalous screen must fail on 100% of W while passing > 90% of A. Otherwise inconclusive.

## 4. Method
- Prime p = largest prime <= 2^14 (16381). Curves y^2 = x^3 - 3x + b, b = SHA-256("seed<k>") mod p, singular b skipped. Order by exhaustive count via precomputed square-count table.
- Hidden predicate: first 6 bits of SHA-256(fixed key || seed) are zero (density 2^-6).
- Arms: A seeds 0.., A2 seeds 1e7.., G seeds 2e7.. (grind until predicate), W seeds 3e7.. (grind until #E = p, n=150). Fixed key and seeds; no randomness beyond these.
- Screens: N prime; N != p; order of p mod N > 30; max prime factor of twist order > 50; max prime factor of N > 50.
- Cost: about 30k point counts at ~4 ms, est. 2-4 minutes, stdlib only, <20 MB RAM.

## 5. Classification
runnable_local. Stdlib toy decides whether the invisibility claim holds at toy scale and checks the 2^-d grinding cost.

## 6. What a run cannot establish
- Nothing about actual NIST/Brainpool seeds, the real 2^-30 density bound, or recovery of historical generator code (the lead's reopen condition).
- The hidden predicate is independent of the curve by construction, so a null is nearly guaranteed; this checks the instrument and the cost accounting, not whether a real backdoor class exists. A real weak class correlated with a screened property would be visible by design; unknown classes are untested.
- Toy size p ~ 2^14; screens' power at 256 bits is not measured. Results labelled [pcc:unverified].
