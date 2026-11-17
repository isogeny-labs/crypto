# LEAD-ICE-charged-toy-ic-exponent-vs-rho

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_blocked (reviewed 2026-10-30). Family: ic-engine. Source: atlas_survey_20261107 --pursuit.
- Title: One fully charged toy-scale index-calculus exponent vs matched rho (GOAL-ICEX-001 / G7).
- Next step (truncated in leads.json): "Build the EXP-ICEX-153c34 backend (G1 membership, O5 oracle, S/G/X/V counters, admission-refusin...". Cost: expensive (backend unbuilt, C-7 gate). Why it might beat sqrt(p): "Produces crossover order; empty crossover = exhausted_at_scale".
- Recorded obstruction: a four-size ratio exponent is unidentifiable; no IC arm has a fitted exponent below rho's. E32 (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md section 9.34/9.36) instead fits a crossover order, n* about 2^85.5 at 0 GAE.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 44, 286); /home/user/crypto/research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md (lines 3495-3510, 3735-3750); /home/user/crypto/docs/session-logs/2026-10-31.md (line 193).

## 2. Hypothesis
In a stdlib toy proxy (discrete log in the prime-order subgroup of Z_p^*, p = 2q+1, p of 16..32 bits), with every operation charged (mulmods plus trial divisions weighted w in {0, 0.1, 1} mulmod each, w = 1 being the pessimistic end of the band), a classical smooth-relation index calculus costs more than matched Pollard rho at every ladder size, so the crossover is empty on the ladder and the fitted crossover order lies beyond 32 bits. This tests the shape of the recorded obstruction (empty crossover, exponent not identifiable from a short ladder) in a proxy, not the EC backend EXP-ICEX-153c34.

## 3. Preregistered prediction and falsification criterion
- Statistic: per size b, ratio R_w(b) = median IC cost / median rho cost over 12 seeds; also OLS slopes of log2 cost vs bits with 400-draw seed-bootstrap 95% CIs, and the linear-fit crossover bits.
- Prediction: R_1(b) > 1 at all five sizes (16, 20, 24, 28, 32 bits).
- Consistent_with_prediction: the prediction holds and both gates below pass. Falsified: gates pass and some R_1(b) <= 1 (IC beats rho at w = 1 on the ladder). Inconclusive: a gate fails (or --smoke).
- Positive control: rho slope in log2 cost per bit must be within 0.1 of the theoretical 0.5 (detector sees a known exponent). Gate fails otherwise.
- A/A arm: odd vs even seeds, same instrument; |slope difference| must be < 0.15 (rho) and < 0.3 (IC), the noise floor. Gate fails otherwise.
- Negative control/null: every rho and IC output is verified by G^s == y (a wrong-answer solver is rejected by assertion); w = 0 and 0.1 results are reported unranked as the sensitivity band (AGENTS.md 5a), only w = 1 decides.

## 4. Method
- Group: safe prime p = 2q+1 from Random(seed*1000003 + bits), G = r^2 of random r, secret s random, y = G^s. Seeds 1..12 per size, sizes 16, 20, 24, 28, 32 bits, bootstrap seeds 777 (main), 1 and 2 (A/A halves).
- Rho: Floyd, 3-way partition walk, 3 mulmods per iteration, restart on degenerate collision.
- IC: factor base = primes <= B that are quadratic residues, B = max(30, exp(0.6 sqrt(ln p ln ln p))); relations G^e smooth over the base, sparse incremental Gaussian elimination mod q (every multiply counted), back-substitution, then individual-log by smooth y*G^e. Cost = M + w*D, M = mulmods and elimination ops, D = k trial divisions per candidate.
- Cost estimate: well under 1 minute expected, hard budget 10 minutes, one core, under 50 MB, no network.

## 5. Classification: runnable_local
A stdlib toy of the proxy can decide the stated question. The real next step (the EC backend with G1/O5/S/G/X/V counters) stays blocked on an unbuilt backend and the C-7 gate; this protocol sharpens only the proxy question. run.py checked in with `--smoke` passing.

## 6. What a run cannot establish
- Nothing about elliptic curves: the proxy group is Z_p^*, where classical IC is subexponential in a way no known EC IC is; the ladder is 16 bits wide, far below any real crossover.
- Operation counts, not wall-clock; trial-division weighting is a stand-in for GAE pricing of the unpriced native work.
- Median-of-12 slopes over five sizes carry wide intervals; a fitted crossover is an extrapolation, not a measured crossing.
- Not an improvement over the index-calculus baseline, no replay certificate or validator report; results are labelled [pcc:unverified].
