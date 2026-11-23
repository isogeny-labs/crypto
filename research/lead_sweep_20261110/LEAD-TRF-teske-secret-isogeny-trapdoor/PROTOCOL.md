# LEAD-TRF-teske-secret-isogeny-trapdoor

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24). Family: transfer. Cost: "medium (20-arm panel; previous run died on psutil import)".
- Title: Teske-style trapdoor on ordinary prime-field curves: a secret isogeny to an endpoint with a non-invariant ECDLP weakness.
- Next-step excerpt (truncated in leads.json): "Re-run the approved 20-arm panel EXP-ECTD-0cf008 (H-ECTD-e88346: crater/floor pairs with conduct..."
- Why it might beat sqrt(p): "Needs a weakness varying inside one isogeny class; Tate leaves nothing to hide on F_p, class size h~p^(1/2)".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 57-60, 113-116, 273: 40-bit N, five classes homogeneous, EV-ECTD-8c098d "weaken"; run-20261016T0409Z died on psutil import);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id/title only).
  Nothing in /home/user/cryptanalysis was searched beyond this; the EXP-ECTD-0cf008 spec was not located, so the 20-arm panel is not reproduced here.

## 2. Hypothesis
Within one F_p isogeny class (fixed trace t, hence fixed N = p+1-t by Tate), the only varying structure visible to a generic attacker is group shape (here 2-torsion rank, proxy for crater vs lower volcano level). H: generic Pollard rho cost on the largest prime-order subgroup l of E(F_p), normalised for the shared l, does not differ between 2-torsion rank-2 and rank-1 members of the same class: |mean log(steps_rank2 / steps_rank1)| <= 0.15. I.e. the recorded obstruction (Tate leaves nothing to hide) holds at toy size, so a "secret isogeny to a weak endpoint" cannot give a rho-visible weakness.

## 3. Preregistered prediction and falsification criterion
- Statistic: D = mean over matched pairs of log(mean rho steps on rank-2 curve / mean rho steps on rank-1 curve), same p, same N (so same l), 10 rho trials per curve, bootstrap 95% CI (2000 resamples).
- Prediction: CI of D lies inside [-0.15, +0.15]  -> verdict consistent_with_prediction (obstruction holds at toy size).
- Falsified: CI entirely above +0.15 or below -0.15 (a class-internal shape weakness exists for generic rho).
- Otherwise, or if either control fails: inconclusive.
- Null / negative control: matched pairs share N by construction; the null is D = 0.
- A/A arm: the same rank-1 curve run twice with independent seeds; its mean log ratio CI must contain 0.
- Positive control: on rank-2 curves with two prime factors l1 > 4*l2 (l2 >= 5), rho cost on the l1 subgroup vs the l2 subgroup; mean log ratio must exceed half of the theoretical 0.5*log(l1/l2), proving the detector sees a real cost difference.
- Not decided: anything about non-rho attacks, or about the real 20-arm panel.

## 4. Method
Python 3 stdlib only. p = 4093, seed 20261110 (single random.Random). Sample 5000 random short Weierstrass curves y^2=x^3+ax+b; count points by Legendre sum over x; count roots of the cubic (1 or 3) to get 2-torsion rank. Keep curves with even N and largest prime factor > 11; group by N; pair rank-2 with rank-1 curves of equal N; shuffle; take up to 120 pairs. For each curve: l = largest prime factor of N, P = (N/l^v) G for random G, Q = kP, Pollard rho (Floyd, 16-way r-adding walk, affine arithmetic) recovers k (checked against planted k; failure drops the pair). Steps averaged over 10 trials. Cost estimate: roughly 1-3 minutes on one core, < 50 MB RAM, far under 10 minutes.
Smoke: `python3 -I run.py --smoke` (p=211, tiny, < 1 s; verdict there is not meaningful).

## 5. Classification: runnable_local
A stdlib toy decides the sharpened question (does any class-internal shape change generic rho cost). The real EXP-ECTD-0cf008 panel (40-bit N) is not rerun here.

## 6. What a run cannot establish
- Tate guarantees equal N within a class, so this can only confirm the invariance of generic cost; it says nothing about non-generic attacks on endpoint structure.
- 2-torsion rank is a proxy for volcano level, not conductor or endomorphism ring; p = 4093 is toy size and rho constants are noisy at l ~ 10^3.
- Does not reproduce the 40-bit panel, the Teske construction, or any isogeny walk. Results are [pcc:unverified].
