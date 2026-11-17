# LEAD-RHO-cheon-deployed-applicability

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_blocked (2026-11-01 and last reviewed 2026-10-24). Family: generic. Source: atlas_survey_20261107 --pursuit.
- Title: "Where Cheon stops on deployed parameters: exact r+-1 divisor census times priced auxiliary supply".
- Next-step excerpt (truncated in leads.json): "Approve a revised protocol for the typed quantity of H-AUXIN-6db354 (per-branch, supply-capped E".
- Atlas note: P-256/secp256k1 E(r) ~0.254 at d~2^128; "values need d-degree supply, not a vulnerability".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 295-297);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 85-93). The lead YAML is encrypted; H-AUXIN-6db354 was not found.

## 2. Hypothesis
For random primes r of 12..22 bits, the exact census "r-1 or r+1 has a divisor d in [r^0.4, r^0.5]" (the divisor supply a Cheon-type auxiliary-input attack needs) is satisfied by a large and slowly varying fraction E(bits): E(22 bits) in [0.70, 0.90] and |E(22) - E(12)| < 0.10. So at toy size the divisor census alone is not the binding constraint; the binding constraint is the priced supply of d auxiliary points [alpha^i]G (the recorded obstruction).

## 3. Preregistered prediction and falsification criterion
- Statistic: E(bits) = fraction of 400 random primes per size with a qualifying divisor, with binomial SE.
- Consistent: E(22) in [0.70, 0.90] and |E(22)-E(12)| < 0.10, with all controls passing. Falsified: either condition fails with controls passing. Inconclusive: any control fails (and always for --smoke).
- Negative control: random odd integers (not conditioned on primality) tested the same way; reported beside E. It is a reference, not a gate.
- Positive control: planted primes with r-1 = d*k, d ~ r^0.4; the detector must flag 100%.
- A/A arm: the same prime sample split in halves; |difference| <= 3 SE required.
- Decides: only whether the census is a toy-scale bottleneck. It does not price supply.

## 4. Method
Python 3 stdlib only. Miller-Rabin with fixed bases (deterministic below 3.3e24), trial-division factoring of r-1 and r+1, full divisor enumeration, window test. Sizes 12,14,16,18,20,22 bits; 400 primes per size; seeds 20261110+bits (primes), 20261111+bits (null), 20261113+bits (positive control). Cost: well under 1 minute, under 50 MB, one core.

## 5. Classification: runnable_local
A stdlib toy decides the census question; the priced-supply part is not run.

## 6. What a run cannot establish
- Nothing about deployed sizes (P-256 order ~2^256) or the 0.254 figure there; toy sizes only.
- It prices no auxiliary supply and runs no Cheon attack, so it says nothing about vulnerability.
- The window [r^0.4, r^0.5] is my choice, not H-AUXIN-6db354's typed quantity, which I could not read.
- Thresholds were set after a smoke look at sizes 12-22 (E near 0.8), so the prediction is partly calibrated, not blind.
- Results are labelled [pcc:unverified].
