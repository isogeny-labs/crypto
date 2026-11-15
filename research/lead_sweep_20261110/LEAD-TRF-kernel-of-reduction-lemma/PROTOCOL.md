# LEAD-TRF-kernel-of-reduction-lemma

## 1. Lead as recorded
- disposition: open_next_step; family: transfer; title: (empty in leads.json); source: atlas_survey_20261107 --pursuit.
- next_step_excerpt (truncated in record): "Write the lemma with the p-adic formal-group channel of anomalous curves excluded explicitly, th..."
- next_step_cost: not recorded in leads.json entry read. why_it_might_beat_sqrt_p: not recorded.
- Context (atlas_survey.md, row A4/item 11): the lemma is "expected to collapse five leads' revisit clauses into one obstruction" (char-0 lift / xedni / function-field lift / global-lift-height leads). Related closed leads: TRF-padic-canonical-section (only homomorphic section is prime-to-p torsion, killed by formal log), TRF-cm-canonical-lift-heights (Pr[Q in red(S)] <= |S|/N), TRF-anomalous-padic-lift (Smart works only at N=p).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep excerpts only, lines 23-71, 266-268). No other hits read. The lemma's full text was not found; this protocol works from the record plus the known mechanism.

## 2. Hypothesis
Let E/F_p have prime order N != p, lifted to E~/(Z/p^2). The only reduction-kernel channel is E_1/E_2 ~ F_p via the formal-group parameter psi. Claim H: for non-anomalous E, the Smart-style statistic r = psi(N Q~)/psi(N P~) mod p, for independent random lifts P~, Q~ of P and Q = kP, equals k mod p with probability 1/p (r - k is uniform on F_p, independent of k), so the formal-group channel carries no information on log_P Q; only for N = p does r = k mod p with probability 1.

## 3. Preregistered prediction and falsification criterion
- Statistic: hits = #{trials: r == k mod p}; secondary: chi-square (15 df) of (r-k) mod p in 16 bins.
- Prediction: null arm hits per arm <= mu + 5 sqrt(mu) + 2 with mu = usable/p; chi2 <= 37.7 per curve; positive control hits = usable (100%).
- Falsified: any null arm exceeds the hit bound (channel leaks log information). Consistent: all criteria hold. Inconclusive: positive control fails (detector blind), chi2 or A/A failure, or no curves found.
- Negative control / null: non-anomalous prime-order curves (N != p, N prime). Positive control: anomalous curves N = p, where Smart's attack must recover k mod p. A/A arm: same protocol, same curve, different seed; hit counts must differ by <= 3.
- Decides: consistent supports writing the lemma "kernel of reduction yields nothing for N != p" at toy size; falsified would reopen the lift leads' revisit clauses.

## 4. Method
- Pure stdlib Python. Curves y^2 = x^3+ax+b over F_p, primes p in [3000, 40000] (group orders ~2^11.5..2^15.3), seed 20261110 (per-arm seeds derived from p).
- Group law: Renes-Costello-Batina complete projective addition, valid over Z/p^2 (checked by positive control recovering k).
- Point counts by Legendre-symbol sum. Positive: 3 anomalous curves; null: 6 prime-order curves; 3000 trials per arm, two arms (A, A/A) per null curve.
- Per trial: random k in [1,N-1], Q=kP over F_p, independent random Hensel lifts to Z/p^2, compute psi(X) = (-X/Y)/p mod p of N*P~ and N*Q~ (kernel points), ratio r.
- Cost: well under 2 minutes, < 20 MB RAM. Smoke: `python3 -I run.py --smoke`.

## 5. Classification
runnable_local. A stdlib toy decides the statistic exactly at toy size.

## 6. What a run cannot establish
- It tests only the mod-p^2 (first formal-group layer) channel, not higher layers, global lifts, heights or xedni/function-field constructions; it does not prove the lemma for all p.
- Uniformity is expected from the algebra (N invertible mod p), so a pass is a sanity check, not new evidence against sqrt(p); it is not an improvement over the IC baseline.
- Toy sizes only; results are [pcc:unverified].
