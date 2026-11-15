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

## Amendment A1 (2026-11-14, after Part A data, before any write-up)

- Part A result: `descent_skip` never fired (`pkm_descent_skipped` absent in all three cells) and A2 equals A1 count for
  count. Cause, from reading `cofactor_fiber` (`ic_framework/fiber.rs`): the torsion group is generated from `[r]P`
  over the *base points*; on a `square-bits` base every `[r]P` lies in 2E[8], so the fiber has 4 elements and the oracle
  only tries the 4 live lifts. P2 is falsified by this (A2/A1 = 1.000, not 0.45 to 0.62).
- Post hoc control (not registered), added to check that reading: `bits` lo = 0, t = 5, G = 3, same seed and targets,
  `lifts` = 0 and 1. If the reading is right the `bits` base (both classes) should try 8 lifts per target.
- The opt-in flag is a no-op, so its code is not kept in `src/`: the change is saved as `descent_skip.patch` (base
  `1edbb0262c~1`) and the measured binary's identity is in the README. `src/` is unchanged by this study.

## Amendment A2: Part C, yield per F4 mul-add at 2,000 targets (frozen before its data; Part A data seen)

Part A's 100-target cells return 0 to 2 hits, too few to compare yield. Part C repeats the arms that matter with
2,000 targets, same binary (flag off), seed 2026111402, m = 2, mode full, G = 3, t = 5:

- bases: `bits` (lo = 0), `square-bits@0`, `square-bits@414271`;
- arms per base: `lifts=0` (blind) and `lifts=1`.

**Metric:** hits_verified per 10^9 counted F4 mul-adds, per arm, with exact Poisson 95% intervals on the hit
counts (the mul-adds are deterministic counts). Unpriced native work is the lift additions and the fiber
construction (set-up group ops), reported separately.

**Predictions (direction fixed now):**
- P4: on every base, `lifts=1` multiplies the F4 cost per target by about the number of lifts tried (8 on `bits`,
  4 on `square-bits`) and raises hits per target, but by less than that factor, so hits per 10^9 mul-adds fall.
- P5: of the three bases, `bits` lifts=1 has the highest hits per 10^9 mul-adds among the lifts=1 arms.
No claim against rho: no rho arm is measured.

## Amendment A3: Part C2, replication at 20,000 targets on a fresh seed (frozen before its data; Part C data seen)

Part C (2,000 targets, seed 2026111402; `runs_c_summary.txt`) found 2 to 19 hits per arm, so its intervals overlap.
Observed there, and what it did to the registered predictions: P4 held in point estimate on `bits` (yield per
mul-add 0.329 → 0.124) and on `square-bits@414271` (0.253 → 0.127) but not on `square-bits@0` (0.203 → 0.241); P5
(`bits` lifts=1 highest) was contradicted in point estimate (`square-bits@0` lifts=1 0.241 vs `bits` 0.124). No
interval separated. Part C2 repeats the same six arms with 20,000 targets and a new seed, 2026111403, same binary,
same G and t, no time cap that could truncate an arm.

**Hypotheses, informed by Part C and so a replication, not a fresh prediction:**
- H1: `square-bits@0` lifts=1 has higher hits per 10^9 mul-adds than `bits` lifts=1, with non-overlapping 95% intervals.
- H2: on `bits`, lifts=1 has lower hits per 10^9 mul-adds than lifts=0.
- H3: `lifts=1` multiplies cost per target by 8.0 on `bits` and 4.0 on `square-bits` (structural; Part A and C agree).
Decision: a hypothesis is supported only if its interval statement holds; otherwise it is reported as unresolved,
including when only the point estimate agrees.
