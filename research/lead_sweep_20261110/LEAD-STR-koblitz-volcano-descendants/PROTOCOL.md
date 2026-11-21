# LEAD-STR-koblitz-volcano-descendants

## 1. Lead as recorded
- disposition: open_next_step (disposition_20261101: closed_obstructed); family: structure; last_reviewed 2026-10-24.
- title: Curves below a binary Koblitz curve in its volcano (ECC2K-130, K-163, K-283, m=37..83).
- next-step excerpt (truncated in leads.json): "An explicit K-163 / K-283 floor map is persisted (problems/koblitz-vertical-isogeny-maps-not-per..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 53-66, 258-272; the lead has only the stub "binary Koblitz", plus the closed sibling LEAD-STR-isogeny-class-generic-invariance: floor/crater rho 1.004 [0.990,1.016]); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep hit only). No cryptanalysis search was run; the rest is from the mechanism.

## 2. Hypothesis
For the Koblitz curve E_K: y^2+xy=x^3+x^2+1 over F_{2^m}, End(E_K)=Z[tau] is maximal while Z[pi], pi=tau^m, has conductor f=|A_m| (tau^m=A_m tau+B_m). Curves below E_K in the volcano (when f>1) have the same group order N but no Frobenius endomorphism tau, so they lose the Frobenius-orbit collapse. Claim: the collapse is the only lever, and it equals a factor m in collision rate (sqrt(m) in rho steps) on E_K and exactly 1 on any curve without tau (descendants). Hence descendants give no improvement over plain rho (consistent with the closed class-invariance result).

## 3. Preregistered prediction and falsification criterion
Statistic: R_m = (pair collisions among canonical Frobenius-orbit x-labels) / (pair collisions among plain x-labels), equal sample size S, random uniform points of E_K(F_{2^m}).
- Positive control: Frobenius arm, predicted R_m = m; accept 0.75m <= mean R_m <= 1.3m over 5 seeds for m in {11,13,15,17}.
- Null / A/A: independent second plain sample vs first, ratio in [0.7,1.4]. The plain arm stands in for descendants (no tau, hence no orbit identification); this is theory, not a measured descendant.
- Also: formula #E = 2^m+1-(T A_m + 2 B_m) must equal brute count at m=11,13.
- Outcomes: all in band => consistent_with_prediction (the lever is exactly m and absent below the surface). Control and A/A pass but R_m outside band => falsified (an unexplained lever). Control or A/A failure => inconclusive.
- Volcano table (reported, not part of the verdict): f and its small-prime factors for prime m up to 83; m with f=1 have no descendants (m=13 is one).

## 4. Method
Pure stdlib GF(2^m) arithmetic (polys in run.py), half-trace point sampling (m odd), x-only labels (merges +-P), Frobenius label = min over x^(2^i). S = 3000/8000/20000/40000 for m = 11/13/15/17. Seeds 101,202,303,404,505 (derived rng seed = seed*1000+m). Conductor via exact integer recurrence. Cost: well under 1 min, under 50 MB.

## 5. Classification
runnable_local. A stdlib toy decides the control and sharpens the obstruction. Actual descendants need Velu 2-/3-isogenies over F_{2^m} (not built), so this is a re-derivation of the recorded obstruction.

## 6. What a run cannot establish
No descendant curve is constructed or measured; the null arm is a theoretical stand-in. No K-163/K-283/ECC2K-130 floor map, no rho step costs (collision rate only), no G-model gap, no sub-sqrt claim. Toy sizes only. All results [pcc:unverified].
