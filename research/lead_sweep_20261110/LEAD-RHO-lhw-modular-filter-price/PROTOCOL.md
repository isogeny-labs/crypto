# LEAD-RHO-lhw-modular-filter-price

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24; source atlas_survey_20261107 --pursuit). Family: generic.
- Title: Residue-guess price of modular (mod-M) representation filters in prime-order groups.
- Next-step excerpt (truncated): "Run EXP-LHW-9adf78 (residue-guess price on W(m, w) with Mersenne/Fermat M) or write the proof ob..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep hits, lines 67 and 252 only: lists it among LHW leads, "not generic-key leads"); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id/title only). No lead-specific note or EXP-LHW-9adf78 record found; protocol is from the mechanism, not the original experiment.

## 2. Hypothesis
W(m,w) = m-bit integers of Hamming weight w. A mod-M representation filter (guess k mod M, keep a with a mod M fixed) has price governed by the residue collision price P(M,m,w) = M * sum_r p_r^2 (P=1 iff residues uniform). Claim: for Mersenne/Fermat M = 2^s -/+ 1, P exceeds 1.2 times the largest P of generic odd moduli of the same bit size (with ord_M(2) >= m) in at least 75% of grid cells, because 2^i mod M takes only s (resp. 2s) values, so the filter is non-uniform and a guess of the residue is cheaper/costlier than the 1/M nominal price.

## 3. Preregistered prediction and falsification criterion
- Statistic: pass_fraction = share of cells (s in {8,10,12}, m in {24,32,40}, w in {4,6,8}, M in {2^s-1, 2^s+1}; 54 cells) with P_special > 1.2 * max over 12 generic moduli.
- Prediction: pass_fraction >= 0.75 -> consistent_with_prediction (the Mersenne/Fermat price differs materially from the nominal 1/M; says nothing yet about an ECDLP speedup). pass_fraction < 0.25 -> falsified (special moduli are priced like generic ones). Otherwise inconclusive.
- Positive control: w=1, M=2^8-1, m=24: residues cycle through 8 values so P = M/8 exactly (tolerance 1e-9).
- Negative control / null: unrestricted-weight uniform m=40-bit integers (200k seeded samples), special M=2^10-1 vs generic M=2^10+3: prices must agree within 0.25 (the effect must come from weight restriction).
- A/A arm: the 12 generic moduli per cell are split 6/6; median ratio of halves must lie in (0.9, 1.1).
- If any control fails the verdict is forced to inconclusive.

## 4. Method
Exact DP over bit positions: dp[j][r] counts weight-j integers with residue r (add 2^i mod M by rotation). P computed from exact counts (no sampling) for the grid cells. Generic moduli: random odd M in [2^s+1, 2^(s+1)), ord_M(2) >= m, seed 20261110 (random.Random). Control sampling seed 20261111. Cost: about 90 DPs of size m*8*M <= 1.6M list ops each, plus 400k samples; estimated 1-4 minutes, <50 MB, one core, stdlib only. Run: `python3 -I run.py` (smoke: `--smoke`, <10 s).

## 5. Classification: runnable_local
A stdlib exact DP decides the stated statistic at toy sizes (M up to 2^13, m up to 40).

## 6. What a run cannot establish
- It measures residue non-uniformity only; it does not price a full representation/low-weight ECDLP algorithm, group operations, or compare to matched rho (no section 5a band).
- Toy M and m are far from cryptographic sizes; the 1.2 threshold and grid are arbitrary choices fixed before running.
- Non-uniformity could cut either way (cheaper guesses vs worse filtering); the run does not decide net benefit.
- It does not recover the original EXP-LHW-9adf78 design, which was not found. Results are [pcc:unverified].
