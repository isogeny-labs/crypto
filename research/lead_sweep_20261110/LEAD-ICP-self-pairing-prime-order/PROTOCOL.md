# LEAD-ICP-self-pairing-prime-order

## 1. Lead as recorded
- disposition: closed_obstructed (theorem; last_reviewed 2026-10-24; atlas_tally_20261101). Family: ic-prime.
- title: "Nondegenerate self-pairings or bilinear maps on a prime-order E(F_p) subgroup".
- next_step_excerpt: empty (closed lead). Recorded obstruction (atlas_survey.md l.35, desk README l.58, l.149): self-pairings are 1-dimensional, trivial for embedding degree > 1; no nondegenerate self-pairing on a prime-order subgroup; E[N] rational iff embedding degree 1.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); research/prime_field_lead_assessment_20261107/atlas_survey.md (grep excerpts, l.35, 58, 149); research/ecdlp_subexp_desk_20261101/README.md (l.58); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry). The YAML and the TRF-one-point-self-pairing / TRF-mov-small-embedding-degree notes were not read.

## 2. Hypothesis
For every prime-order group E(F_p) with #E = r prime (r != p, p+1), the reduced Tate pairing target F_p*/(F_p*)^r is trivial, i.e. gcd(r, p-1) = 1 and the embedding degree k = ord_r(p) is > 2 (Hasse forces r | p-1 to mean r = p-1, which is not prime). So no F_p-rational nondegenerate pairing exists on E(F_p). The detector is not blind: for a prime-order-r subgroup inside a non-prime-order E(F_p) with r | p-1 and r^2 not dividing #E, the Tate self-pairing t(P,P) is nondegenerate, bilinear, and transfers DLs.

## 3. Preregistered prediction and falsification criterion
- Statistic S1: number of prime-order curves (negative control, both seeds pooled) with gcd(r,p-1) > 1. S2: number with k <= 2. S3: positive-control rate of t(P,P) != 1.
- Prediction: S1 = 0, S2 = 0, S3 = 1.0 with bilinearity and DL transfer holding in every positive-control case.
- Falsified if S1 > 0 or S2 > 0 (a prime-order curve with rational pairing target exists) or the A/A counts differ.
- Inconclusive if the positive control fails (S3 < 1, bilinearity fails, or no positive-control curves found), because then the null cannot be read.
- Consistent_with_prediction otherwise. This only re-derives a known theorem at toy size; it cannot reopen the lead.
- Negative control (null): random prime-order curves, seed 101. Positive control: curves with r | p-1, r | #E, r^2 not dividing #E (seed 303). A/A arm: same procedures with independent seeds 202 and 404; the negative-control counts must be equal (both 0).

## 4. Method
Python 3 stdlib only. Negative control: primes p in [2000,2100) first 4, [8000,8100) first 3, [30000,30100) first 2; per p, 12 random (a,b), #E by Legendre sum, keep #E prime and not p or p+1; compute gcd and k = ord_r(p) by trial factoring. Positive control: prime r in {13,17,19,23,29,31}, p = 1 + r*m prime (m in 60..129), random curves until r || #E, P = h*random point (order r); affine Miller loop with random auxiliary point S, final exponent (p-1)/r; check t(P,P) != 1, t(2P,P) = t(P,P)^2, t(dP,P) = t(P,P)^d. Seeds 101/202/303/404. Cost estimate: well under 2 minutes, < 50 MB, one core (dominated by O(p) counting at p ~ 3e4).

## 5. Classification
runnable_local. A stdlib toy re-derives the recorded bound (empirical check of the Hasse argument and of nondegeneracy exactly when the target is nontrivial).

## 6. What a run cannot establish
- It does not prove the theorem (the Hasse argument does); it is a toy check at p <= 3e4, not at cryptographic sizes.
- Supersingular r = p+1 (k = 2) cases are excluded; they need a distortion map, not an F_p-rational pairing, and are outside this lead.
- It says nothing about non-F_p-rational pairings or GGM-simulable oracles. A consistent verdict is not a speedup or an improvement over the index-calculus baseline. Results are labelled [pcc:unverified].
