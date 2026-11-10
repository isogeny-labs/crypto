# LEAD-TRF-anomalous-padic-lift

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- id: LEAD-TRF-anomalous-padic-lift; disposition: closed_known (also closed_known on 2026-11-01); family: transfer
- title: Smart / Satoh-Araki / Semaev p-adic lift for anomalous (trace one) curves; last_reviewed 2026-10-29; source atlas_survey_20261107 --pursuit
- next-step excerpt: "Revisit when: never for generic curves; only as a positive control or for a curve with #E = p"
- survey note: "Smart certificate p=771853 C_special 157 vs matched rho 779".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (A4 line 58, listing line 325);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (p-adic lift rows, lines 57-61).
  No other hits opened (data/*.json hits not read).

## 2. Hypothesis
The recorded obstruction is exactly the condition #E(F_p) = p. Claim: (a) on anomalous curves over prime fields of 12..22 bits, the Smart lift (lift E, P, Q to Z/p^2, compute p*P~, p*Q~, take the formal-log ratio psi(pQ~)/psi(pP~) mod p) recovers the planted d with success >= 95%, using about 3*log2(p) Jacobian operations over Z/p^2, a fraction of the matched rho reference sqrt(pi p/2) that shrinks with size; (b) the identical code on non-anomalous curves (point order r != p, multiplier r in place of p) recovers d only at chance rate (about 1/p), so the lead gives nothing for generic curves.

## 3. Preregistered prediction and falsification criterion
- Statistic: success counts of d_recovered == d (verified against the planted d), per arm and pooled over sizes; null pooled successes.
- Prediction: A and A' (anomalous, disjoint seeded instances) success >= 0.95 each and |A - A'| <= 0.10; null pooled successes <= 5 (expected total about 0.01); Smart ops / rho_ref decreasing with bit size.
- Decisions: falsified if either anomalous arm < 0.95 (the positive control fails, or the lift is wrong) or null successes > 5 (the lift leaks on generic curves, which would reopen the lead); inconclusive if degenerate instances (psi undefined) exceed 20% or arms differ by > 0.10; otherwise consistent_with_prediction (closure re-derived).
- Positive control: the anomalous arm itself (detector sees a known effect). Negative control / null: random non-anomalous curves, r = exact point order from BSGS in the Hasse interval, r != p, r >= 1000, same code path and sizes. A/A arm: A vs A' (same procedure, independent draws from one RNG stream) giving the noise floor.
- Rho pricing (AGENTS.md 5a, 0/0.1/1 GAE) is not applicable: the rho reference is analytic, Smart ops are counted in Jacobian additions/doublings only, and no cross-method rank is claimed.

## 4. Method
- Python 3 stdlib only. Seed 20261110 (per-size RNG = 20261110*1000 + bits). Sizes: random prime in [2^k, 2^(k+1)) for k in 12,14,16,18,20,22; 25 instances per arm per size (smoke: k=12, 3 instances).
- Anomalous curve search: random (a,b) mod p, random point P, accept if p*P = O (then #E = p, as p > 7 by Hasse). Planted d uniform in [1,p), Q = dP.
- Lift: same integers a,b; x kept, y by one Hensel step mod p^2. Jacobian double-and-add over Z/p^2 (no inversions). psi(R) = (X*Z/Y mod p^2)/p mod p (formal log up to sign, which cancels in the ratio). d = psi(nQ~)/psi(nP~) mod p with n = p (anomalous) or n = r (null).
- Cost: dominated by anomalous curve search (about sqrt(p) tries; roughly 1 minute at 2^22). Estimated total under 5 minutes, <50 MB RAM, one core.
- Output: one JSON object with per-size rows, statistic, thresholds, verdict, wall time.

## 5. Classification: runnable_local
A stdlib toy decides the question directly: the Smart lift is a few modular operations, curve order is checked by p*P = O and BSGS.

## 6. What a run cannot establish
- Toy sizes only; says nothing about cryptographic sizes beyond the standard asymptotics (poly(log p) for #E = p).
- It re-derives a known closure (Smart 1999, Satoh-Araki, Semaev): it is no new improvement over index calculus and no change to any leaderboard.
- Null uses same-prime curves where lifting is a generic map; it does not rule out other, different lifts of non-anomalous curves (covered by the separate p-adic canonical section closures).
- rho comparison is analytic, not measured; counts are Jacobian operations over Z/p^2, whose per-op cost is a constant factor larger than over F_p.
- All results are [pcc:unverified] (no replay certificate, validator or VALIDATION.json).
