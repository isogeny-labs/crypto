# LEAD-TRF-pairing-target-field-recost

## 1. Lead as recorded
- Disposition: open_next_step (reviewed 2026-10-24). Family: transfer. Source: atlas_survey_20261107 --pursuit.
- Title: Recompute the post-MOV target-field DLP cost (exTNFS in GF(p^12)) for BLS12-381 and BN254 with memory charged.
- Next-step excerpt (truncated in leads.json): "Run the certified toy tower-field DLP that RQ-PAIR-f2e31f already specifies (H-PAIR-62e9cc: copr..."
- Survey note: "no ECDLP content for non-pairing curves; 33 contracts zero runs".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 57-60, 268-282); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep context only). RQ-PAIR-f2e31f / H-PAIR-62e9cc text was not found, so the protocol below is my own reading, not the lead's frozen spec.

## 2. Hypothesis
With the exTNFS time estimate for GF(p^12) anchored to 2^128 for a 3072-bit prime field and memory charged as T*M^w with M = sqrt(T), w in {0, 0.5, 1}:
(a) BN254 beats matched rho (2^126.8 steps) at w=0 but loses at w=1, so its rank against rho flips inside the band;
(b) BLS12-381 is within 4 bits of rho at w=0 and does not beat rho at w=0.5.
This is a cost-model claim, not a measured attack.

## 3. Preregistered prediction and falsification criterion
- Statistic: log2 cost of SexTNFS (c=1.526) minus log2 rho, per curve and w.
- Prediction: (a) and (b) both hold. Consistent_with_prediction if both hold and the controls pass. Falsified if either fails with controls passing (a failed (a) means BN254 order does not flip; the rank is then decided across the band). Inconclusive if a control fails.
- Positive control: toy Pollard rho (verified logs) in an order-r subgroup of the norm-1 torus of GF(p^2) (embedding degree 2, tower field). Mean Floyd steps / sqrt(pi r/4) must lie in [1.0, 6.0] for every size (Floyd costs ~3 steps per iteration, so ~3-4 expected). This shows the step counter and the instrument work.
- A/A arm: same instrument, different seeds and different r; |ln ratio| between arms < 0.5 at every size.
- Negative control / null: not applicable to the model arm (it is deterministic arithmetic); the rho toy is the null for "cost = sqrt(r) scale".
- Cross-method comparison must also hold under the section 5a 0 / 0.1 / 1 GAE band; here only the memory band w is varied and the native-work price is not applied, so no ranking claim is made.

## 4. Method
- Python 3 stdlib only, one core, well under 10 min (estimated < 2 min).
- Toy: for bits in 12, 15, 18, 20: find prime r ~ 2^bits and prime p = h*r - 1 (p = 3 mod 4), GF(p^2) = GF(p)[i]/(i^2 - n). Generator g of the order-r subgroup of the norm-1 torus. 60 rho trials per arm (16-way r-adding walk, Floyd cycle finding), log verified by g^log == h. Seeds: arm A 1000+bits, arm B 5000+bits (RNG random.Random).
- Model: log2 L_Q(1/3, c) without o(1), offset fixed by anchoring a 3072-bit prime field (c=(64/9)^(1/3)) to 2^128. Q = p^12 for p of 254 and 381 bits; c in {1.526 (SexTNFS), 1.923 (MexTNFS)}; memory-charged cost = T + w*T/2 (log2). Matched rho = sqrt(pi r/4), r of 254 and 255 bits.
- Smoke: `python3 -I run.py --smoke` (one size, 6 trials).

## 5. Classification
runnable_local. The model recost and a toy tower-field rho control fit stdlib limits. The real exTNFS DLP in GF(p^12) is not attempted.

## 6. What a run cannot establish
- No exTNFS was run; the o(1) term is replaced by one anchor, so the 2^99 and 2^127 figures are only as good as that anchor and the c values (published estimates differ by several bits).
- The memory charge T*sqrt(T)^w is a stand-in, not a measured memory model, and not the section 5a GAE pricing.
- The toy is GF(p^2), not GF(p^12) with a tower of the lead's certified specification; RQ-PAIR-f2e31f was not read.
- Says nothing about ECDLP on non-pairing prime-field curves. Result is [pcc:unverified].
