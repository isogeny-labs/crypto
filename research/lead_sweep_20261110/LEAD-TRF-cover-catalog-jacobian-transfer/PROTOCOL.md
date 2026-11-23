# LEAD-TRF-cover-catalog-jacobian-transfer

## 1. Lead as recorded
- disposition: open_blocked (also open_blocked on 2026-11-01); family: transfer; last_reviewed 2026-10-24; source atlas_survey_20261107 --pursuit.
- title: Catalogued cover maps (1,940 prime quadratic pullbacks, 160 binary cubic pullbacks): executable pushforward and subgroup recovery.
- next step (truncated): "Build even-degree (two points at infinity) odd-characteristic Mumford arithmetic and the general ..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 299 only); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (row 4: trace map E(F_p^2) = E(F_p) + E'(F_p); "the F_p part is the original problem"). Nothing found in cryptanalysis (not searched beyond these).

## 2. Hypothesis
For even-degree genus-2 covers C: y^2 = x^6 + a x^4 + b x^2 + c over F_p (two rational points at infinity, odd p), the Jacobian splits as J(C) ~ E1 x E2 with E1: y^2 = x^3+ax^2+bx+c and E2: y^2 = c x^3+bx^2+ax+1, so #J(C)(F_p) = #E1(F_p)*#E2(F_p) exactly. Consequently the pushforward of the E1 DLP into J(C) lands in an isogeny factor of the same size as E1's subgroup: the transfer gives no group-order reduction, and is useful only if a genus-2 index calculus beats rho on the image, which the recorded blocker (Mumford arithmetic) does not address. This toy tests the arithmetic identity the executable pushforward must satisfy.

## 3. Prediction and falsification
- Statistic: fraction of sampled curves with #J(F_p) == #E1*#E2, where #J = (N1^2+N2)/2 - p from point counts N1 over F_p, N2 over F_p^2 (infinity points included).
- Prediction: split arm = 1.0; A/A arm (different seed, same pipeline) = 1.0; null arm <= 0.2.
- Falsified if any split or A/A curve violates equality (decides: the splitting claim or the counting code is wrong; the lead's pushforward premise is then unsupported). Inconclusive if split and A/A are 1.0 but the null exceeds 0.2 (detector cannot distinguish).
- Positive control: the split arm itself (symmetric sextics, theory-guaranteed). Negative control/null: random monic squarefree sextics with an odd coefficient forced nonzero, compared against the same E1/E2 formulas applied to their even coefficients. A/A: second symmetric arm, seed 20261111.

## 4. Method
Pure Python. Primes p = 3 mod 4, 59..139 (F_p^2 = F_p[i]); 60 curves per arm, cycling primes; seeds 20261110 (split), 20261111 (A/A), 20261112 (null). Squarefreeness via gcd(f,f'). Point counts via precomputed square-root multiplicity tables over F_p and F_p^2. Group orders ~p^2 .. 2^14..2^18. Estimated cost under 1 minute, under 50 MB, one core.

## 5. Classification
runnable_local. A stdlib count-identity check decides the toy; full Mumford arithmetic and the 1,940/160 catalog are not needed for it.

## 6. What a run cannot establish
It checks only the order identity for the bielliptic family, not the catalog's pullbacks, Mumford arithmetic, subgroup recovery, or any DLP speedup. Pass does not make the lead viable; results are [pcc:unverified]. The null at small p can match by chance.
