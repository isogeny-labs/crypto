# Protocol: does a descent-class skip of dead cofactor lifts pay, and is a one-bit class label enough for frame selection? (frozen before data)

2026-11-14. Follows `research/curve25519_miniature_descent_class_20261111` (the open points of its
README). Everything below is on the k = 20 Curve25519 miniature (p = 1048573, A = 926, #E = 8·131203).
All results will be [pcc:unverified]. No rho arm is measured, so there is no cross-method claim; the
distance to the matched rho is therefore not reported, and I say so rather than leave it out.

## Part A: the F4 engine, `lifts=1` on the `square-bits` base

**Code change (opt-in, default off):** `pkm-semaev:...,lifts=1,descent_skip=1` skips a lift R + K
when chi(x_W(R+K) − A/3) = −1 on a `square-bits` base (every base point is in 2E(F_p), so no sum of
base points can equal such a lift). One Legendre symbol per lift, counted as `pkm_descent_legendre`.
`curve25519_family f4cost` takes it as an optional position 14 (`0|1`). The reading of the existing
lift loop (`pkm_semaev.rs`, non-shared branch) is that its only skip is for an already-tried abscissa,
so without the flag every dead lift costs an F4 call.

**Arms** (same binary, same targets): A0 `lifts=0`; A1 `lifts=1`; A2 `lifts=1,descent_skip=1`.
**Cells:** `square-bits@414271` t = 5, G = 3; `square-bits@984071` t = 4, G = 3; `square-bits@0` t = 5,
G = 3. 100 targets, seed 2026111401, m = 2, mode full, the study's cap and nnz settings
(`900 2 full <lifts> 30000000`-style trailing arguments as in `curve25519_torsion_leads_20261103`).
**Measured (counted, deterministic):** hits, hits_exact, hits_verified, F4 calls, F4 mul-adds and per
target, `pkm_lift_targets_tried`, `pkm_descent_skipped`, `pkm_descent_legendre`, set-up group ops.
Unpriced native work (the Legendre symbols) is reported at 0, 0.1 and 1 GAE per operation (§5a).

**Predictions:**
- P1 (soundness): A2 and A1 return the same hits, hits_exact and hits_verified in every cell. A
  difference falsifies the theory (a class −1 lift decomposed) and stops Part A.
- P2: A2's F4 calls are between 0.45× and 0.62× of A1's in every cell (about half the lifts are dead,
  and most targets fail, so all of their lifts are tried).
- P3: the Legendre-symbol work is 8 per target, negligible against F4 mul-adds at any priced value.

**Decision rule:** the skip "helps" in a cell if A2's F4 mul-adds per target are below A1's. It
"pays against blind" if A2's mul-adds per hit are below A0's. Counts are deterministic so no interval
is given; the three cells are reported separately and not pooled. Whether `square-bits` itself is
worth using stays as in the torsion-leads study (coverage about 2×, F4 cost per relation higher).

## Part B: is a one-bit class label enough for frame selection? (exact census, Python)

For `bits` frames `lo` (uniform in [0, p)) at t = 5, 2,000 frames (seed 20261114):
- exact coverage C(lo): number of distinct abscissae of points ±P_i ± P_j that lie in the order-ℓ
  subgroup (base points are those of `bits@lo` on the curve);
- label scores from the coset labels ℓ(P) ∈ Z/8 (ℓ = discrete log of [ℓ-subgroup index] in the
  quotient): S3 = number of signed pairs with ℓ_i ± ℓ_j ≡ 0 (mod 8) (the full label, the parent
  study's information); S1 = number of signed pairs with ℓ_i ± ℓ_j even (the one-bit class only).
- Spearman rank correlations rho(S3, C) and rho(S1, C); and, over 2,000 draws of a random pool of
  100 frames, the mean C of the frame chosen by C, by S3, by S1, and of a random frame.

**Prediction:** rho(S3, C) > rho(S1, C) > 0; mean C of the S1 choice is between a random frame and
the S3 choice. The one-bit label "suffices" if the S1 choice reaches at least 90% of the S3 choice's
mean coverage. Cost note: S1 needs one Legendre symbol per column, S3 needs [r]P and a discrete log
per column; a saving is not claimed without a priced comparison.

## Deviations (stated now)

No Conductor task (not installed), no isolab-eks or S3 resume checkpoint (the AWS route is absent in
this container); counted F4 mul-adds are deterministic and were run locally, as in the earlier
torsion studies. research-kb unavailable; the prior-work check is the repository reading recorded in
the miniature README. No figure or PDF if matplotlib is missing.
