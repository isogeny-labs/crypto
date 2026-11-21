# LEAD-ICB-nonlinear-frob-stable-bases

## 1. Lead as recorded
- disposition: open_next_step (last_reviewed 2026-10-29); family: ic-binary; source: atlas_survey_20261107 --pursuit.
- title: Non-subspace Frobenius-stable factor bases at prime n: trace-spectrum, Hamming ball, multiplicative subgroup, tau-adic, algebraic-group (Couveignes-Lercier) points.
- next_step_excerpt (truncated): "Trace-spectrum V_E is closed (10-12x, 2026-10-28). The Hamming ball B_3 replication (experiments..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines ~209-220 and ~80 only). Other hits (ecdlp_subexp_desk README, atlas_tally) not read; the full B_3 experiment text was not found, so the design is from the entry plus own knowledge.

## 2. Hypothesis
At toy sizes (E: y^2+xy=x^3+x^2+1 over GF(2^n), n in {11,13,15,17}), the Hamming-ball factor base B_3 (x-coordinates of weight <= 3 in a normal basis, Frobenius-stable) has no m=2 decomposition advantage over a size-matched union of random Frobenius orbits: the sumset efficiency ratio R = E(B_3)/mean E(orbit control) lies in [0.90, 1.10] for every n. E(B) = |distinct x-classes of F_i +/- F_j| / (k(k-1)), k = |B|.

## 3. Preregistered prediction and falsification criterion
- Statistic: R per n, plus z = (E_B - mean_ctrl)/sd_ctrl.
- Prediction: all R in [0.90, 1.10] (the weight structure is not additive, so no extra sumset collapse or spread beyond the orbit symmetry both arms share).
- Falsified: any n with R > 1.10 and z > 3 (B_3 gives more distinct sums, a real advantage, or R>1.10 without z>3 also yields "falsified" only if no n is consistent; see run.py). R < 0.90 means B_3 is worse than orbit-random (lead's premise harmed). Either decides the lead.
- Negative control / null: size-matched unions of random full Frobenius orbits (8-16 per n, fixed seeds).
- Positive control: arithmetic progression {jG}, j=1..k, whose sumset is tiny; detector must show ratio < 0.2 to the null mean.
- A/A arm: split orbit controls into two halves; ratio of half means must lie in [0.95, 1.05]. If positive control or A/A fails the verdict is "inconclusive".

## 4. Method
Pure stdlib. GF(2^n) via log tables from a searched primitive polynomial; normal basis by seeded rank test of random theta; points by half-trace (odd n, x != 0); only x of the sum is needed (a=1 affine formulas, both signs of the second point). Seed 20261110 (derived per n as seed*1000+n). Sizes: n=11,13,15,17 with weight 3, controls 16/16/12/8. Cost estimate: under a minute to a few minutes on one core, <100 MB (n=17 tables ~ 2^18 ints). Smoke: n=9, weight 2 (<1 s; too noisy to pass controls, verdict inconclusive is expected).

## 5. Classification
runnable_local: a stdlib toy decides the sumset-efficiency question directly.

## 6. What a run cannot establish
Toy sizes only (group order ~2^11..2^17) and m=2 sumset coverage, not an actual relation-collection or full ECDLP cost. It does not test the trace-spectrum, tau-adic, multiplicative subgroup or Couveignes-Lercier variants, prime-field curves, or linear algebra cost, nor any comparison to rho at the 0/0.1/1 GAE band. Sumset efficiency is a proxy for relation yield. Results are labelled [pcc:unverified].
