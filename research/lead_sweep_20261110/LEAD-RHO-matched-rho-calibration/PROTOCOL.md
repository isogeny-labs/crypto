# LEAD-RHO-matched-rho-calibration

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24). Family: generic.
- Title: "The matched rho reference itself: finite-size bias and the missing prime medium tier".
- Next step (leads.json, truncated): "Seal ecbench bounds for rho.negation, rho.plain and bsgs.negation on prime curves at log2 r 32-4[8]"; cost medium (24+ runs x sizes 32-48 bits).
- Fuller text (atlas_survey.md A5): toy prime rho.negation is 1.453 x floor, alpha 0.398 [0.308,0.476], rho.plain 1.920 [1.633,2.221] (log2 r 15.7-25.3); fit R(N)=1+c/N^beta. Within 1.10x of sqrt(pi/4) means toy excess is set-up and prime gaps shrink; 1.3-1.5x persistent would reopen the negation-map lead.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context, lines 63-67, 250-254);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (grep context, lines 226-290);
  /home/user/crypto/docs/session-logs/2026-11-09.md (grep context only).

## 2. Hypothesis
For a prime-order subgroup of a prime-field curve walked by a 32-jump additive walk with first-repeat (full-memory) collision detection, the finite-size excess of both plain and negation-class rho over the matched random-function null (same state count, same statistic) is <= 1.10 at log2 r = 20, and the negation excess does not grow with size from log2 r = 12 to 20. I.e. the recorded 1.45x toy excess is set-up/finite-size, not an intrinsic property.

## 3. Preregistered prediction and falsification criterion
- Statistic: R = mean(steps to first repeated point) / sqrt(pi*N/2), N = r (plain) or r/2 (negation classes). Excess = R_arm / R_null at the same N (null = random function on N states, so birthday finite-size effects cancel).
- Prediction: plain excess(2^20) <= 1.10 and negation excess(2^20) <= 1.10.
- Consistent_with_prediction: both <= 1.10 and controls pass.
- Falsified: negation excess(2^20) >= 1.30 and >= excess(2^12) - 0.05 (persistent, non-shrinking). Decides that the negation-map excess is intrinsic to the walk at toy size and the lead stays open.
- Inconclusive: anything else, or any control fails.
- Negative control / null: random-function oracle on N states (n_runs x 2 draws), same statistic.
- Positive control: 2-jump walk (known poor mixing, plus cycles); must show excess > 1.3 vs null at largest size.
- A/A arm: plain walk with two disjoint seed sets on the same curve; |z| < 3 required.
- Censoring: runs capped at 30x floor; censored fraction reported (fruitless negation cycles trap and count at the cap, biasing excess upward, which is the effect under test).

## 4. Method
- Pure-stdlib affine EC arithmetic; per size, random curve y^2=x^3+ax+b over prime p ~ 2^bits found by point counting (O(p)), cofactor <= 4, prime r; generator of order r.
- Sizes bits in {12,14,16,18,20}; 400 runs per arm per size; 32 random jump multiples of G; index = hash of x; start at random multiple. Negation arm walks on (x, min(y,p-y)) with no fruitless-cycle fix.
- Seeds: curve rng 20261110+bits; arm seeds 20261110+1000*bits+{1..6}. Deterministic.
- Cost: dominated by point counting (~seconds at 2^20) plus ~1-2M curve additions total; expected < 3 min, < 100 MB.
- Smoke: `python3 -I run.py --smoke` (bits 8,10, 30 runs; verdict forced inconclusive).

## 5. Classification: runnable_local
A stdlib toy decides whether the excess at 2^12..2^20 shrinks, sharpening (not replacing) the 32-48 bit ecbench step.

## 6. What a run cannot establish
- Not the ecbench-sealed, replay-certified 32-48 bit bound; no isolab, no section 5a GAE band, no bsgs.negation arm.
- Full-memory first-repeat counting is not distinguished-point rho; step-cost and storage differ. Group orders 2^12..2^20 only, one curve per size, cofactor curves, 32-jump walk only.
- Extrapolation of beta to 2^32-2^48 is not licensed. All results are [pcc:unverified].
