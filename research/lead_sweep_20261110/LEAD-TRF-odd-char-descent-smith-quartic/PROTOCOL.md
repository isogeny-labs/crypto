# LEAD-TRF-odd-char-descent-smith-quartic

## 1. Lead as recorded
- Disposition: open_blocked (also open_blocked on 2026-11-01); family: transfer; last_reviewed 2026-10-29; source atlas_survey_20261107 --pursuit.
- Title: Odd-characteristic Weil descent onto genus 3, then Smith's isogeny to a plane quartic and Diem's q^{4/3} attack.
- Next-step excerpt (truncated in leads.json): "Implement Smith's Props. 2 and 4 / the trigonal construction and test F_q-rationality and non-de..."
- Fields next_step_cost and why_it_might_beat_sqrt_p are absent from the leads.json entry.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58-60, 298-301: lists the lead as open_blocked, an extension-field/Jacobian transfer that "applies to F_{p^k} curves, not E(F_p)"). Other hits (ecdlp_subexp_desk_20261101/data/*.json) were not read. cryptanalysis repo not searched. The lead atlas YAML and research-kb were not consulted (encrypted/unavailable here), so no novelty claim is made.

## 2. Hypothesis
Necessary condition for the lead: the Weil restriction W = Res_{F_{q^3}/F_q}(E) has Frobenius polynomial x^6 - t x^3 + q^3 (a1 = a2 = 0). Claim: over F_5, smooth plane quartics whose Jacobian has this form (equivalently N1 = q+1 and N2 = q^2+1) realise at least 80% of the admissible traces t, |t| <= floor(q^{3/2}), 5 not dividing t. So for small q the F_q-rationality obstruction is not an obstruction on the level of isogeny-class point counts: a Jacobian of a plane quartic in the isogeny class of W exists for (nearly) every t. This does NOT test Smith's constructive map from a given E, nor non-degeneracy of the quartic. It only tests the existence side.

## 3. Preregistered prediction and falsification criterion
- Statistic: coverage C = |{t attained by smooth sampled quartics with a1=a2=0}| / |W|, W = {t : |t| <= floor(5^{1.5}) = 11, 5 does not divide t} (|W| = 18), pooled over two independent arms of 20000 random quartics each (seeds 20261110, 20261111).
- Consistent with prediction: C_pooled >= 0.80, |C_A - C_B| <= 0.15, >= 30 smooth hits.
- Falsified: C_pooled < 0.50 with >= 30 hits (the isogeny class of W is then mostly missed by quartics at q=5, suggesting a real rationality/degeneracy obstruction at this size).
- Otherwise, or if fewer than 30 hits, or any machinery check fails: inconclusive.
- Positive control: point-counting machinery (F_q digit-plane counting) checked against integer brute force on 150 random quartics (must have 0 mismatches), and s3 must be divisible by 3 for every smooth hit (a Newton-identity consistency check; any failure forces inconclusive).
- A/A arm: the two q=5 arms use identical procedure and different seeds; their coverages must agree within 0.15.
- Negative control: not applicable as a discriminating test of a falsifiable gap. Reported instead, for reference only: coverage of a uniform draw from the wider Hasse-Weil window with the same number of hits (null_uniform_coverage). Also reported descriptively: q=7 (12000 samples), not used for the verdict.

## 4. Method
For q in {5, 7}: draw coefficients c in F_q^15 uniformly (Python random.Random, fixed seeds), curve f = sum c_m x^i y^j z^l (degree 4). Count N1 over P^2(F_q) and N2 over P^2(F_{q^2}) with fields as F_q[x]/(irreducible) and per-point monomial digit planes; N1 is tested first and failing curves are dropped early. For curves with N1=q+1 and N2=q^2+1, reject singular curves by checking f=f_x=f_y=f_z=0 on all F_{q^2} and F_{q^3} points (orbits of size >=4 of reducible curves are missed), then N3 and t = (q^3+1-N3)/3. Only q=5 sizes are in the verdict. Cost: smoke 600 samples in about 2 s; full run estimated 2-4 minutes (40000 q=5 samples, 12000 q=7 samples, one-time F_343 setup of a few seconds), memory well under 200 MB; a q=7 arm is skipped if more than 330 s have elapsed. Stdlib only, one core, no network.

## 5. Classification
runnable_local. A stdlib toy can decide the existence-side question; Smith's actual propositions (isogeny construction from E over F_{q^3}) are not implemented, only the point-count shadow of the target class.

## 6. What a run cannot establish
- Not that Smith's construction works, produces an F_q-rational quartic, or is non-degenerate for a given E; not that Diem's q^{4/3} attack is applicable or fast.
- Existence of a quartic Jacobian with the same Frobenius polynomial does not give an isogeny that is efficiently computable or of small degree (index-calculus transfer needs an explicit one).
- q=5, 7 are tiny; coverage at toy size says nothing about asymptotics. Smoothness test is partial (no absolute irreducibility or higher-degree singular-orbit test). Honest cost comparison against matched rho (AGENTS.md 5a) is not made. Results are [pcc:unverified].
