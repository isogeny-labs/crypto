# LEAD-ICP-hecke-cm-class-point-rank-completion

## 1. Lead as recorded
- Disposition: open_next_step. Family: ic-prime. Title: (empty in leads.json). Source: atlas_survey_20261107 --pursuit.
- Next-step excerpt (truncated): "Compare the rank-completion collector against the pair-index-reuse generic control (not only the ..."
- Cost: est. cheap-medium; first must find missing D79 records. Why it might help: algebraic Hecke relations among known-log CM points could give relations without decomposition; pair-reuse control already gives rank 1 free log, so marginal rank may be 0.
- Atlas survey: CM class points on X_0(11)/F_p (D_K=-39), Hecke marginal rank over pair-reuse 0, 10, 19, 0 on four cells; D79 support-seven/eight and query-order records not found in any repo or PR (2026-11-06). Discriminator: zero marginal rank closes the lead as dominated.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 24-30, 90-104, 235-237 via grep context); /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (row at line 161 only). Not read: cryptanalysis experiments/hecke-sparse-target-20261030, docs/reports/hecke-cm-prioritization-20261106/REPORT.md.

## 2. Hypothesis
In a stand-in model (group Z_n, base points closed under a known eigenvalue lambda of order 4, mimicking Hecke/CM action on known-log points), the marginal rank of eigenvalue ("Hecke") relations over the pair-collision control is 0 whenever pair collisions are dense (mean collisions >= 3m for base size m), and is >= 1 only when collisions are sparse (< m/2). That is, the Hecke arm adds rank only where the generic pair collector is starved, which is the recorded 0/10/19/0 pattern.

## 3. Preregistered prediction and falsification criterion
- Statistic: per cell (n, m), mean and max over 20 seeds of marginal = rank(pair + Hecke relations) - rank(pair relations), rank mod prime n.
- Prediction: max marginal = 0 in every dense cell AND mean marginal >= 1 in at least one sparse cell.
- Falsified: any dense cell with marginal > 0, or all sparse cells with mean marginal < 1. Consistent: neither. Inconclusive: no dense or no sparse cell, or a control fails.
- Negative control: relations from a wrong (non-eigenvalue) multiplier must fail the validity check against hidden logs in every seed.
- Positive control: the sparse cells must show marginal >= 1 (the detector sees Hecke rank when pair collisions are scarce); Hecke relations themselves must be valid in every seed.
- A/A: independent re-instantiation of the pair collector on the same cell (seed+7919); mean |pair_rank difference| reported as noise floor.
- Deciding: consistent supports closing the lead as dominated except in sparse regimes; falsified reopens it for the dense regime.

## 4. Method
Primes n = first prime >= 2^b with n = 1 mod 4, b in {12,14,16,18,20}; m in {16,32,64}; lambda = sqrt(-1) mod n; base = m/4 random seeds with orbits {x, lx, l^2 x, l^3 x}. Pair collector: all m(m-1)/2 pair sums, bucket equal sums, relation e_i+e_j-e_k-e_l. Hecke relations: e_{t+1} - lambda e_t. Rank by Gaussian elimination mod n. 15 cells x 20 reps, seeds 1000003*cell+rep. Cost: seconds, under 50 MB, stdlib only.

## 5. Classification
runnable_local. The stand-in is a stdlib toy that decides the density threshold question without the modular-curve machinery.

## 6. What a run cannot establish
- It does not use X_0(11), D_K=-39 CM points or real Hecke operators; lambda-orbits are a stand-in, so it says nothing about the recorded cells' actual ranks.
- The collector detects sum equality using the hidden logs as an oracle label (as a point-equality test would), with no cost accounting; setup pricing at 0/0.1/1 GAE vs matched rho (section 5a) is not done.
- Does not recover the missing D79 records. All results are [pcc:unverified].
