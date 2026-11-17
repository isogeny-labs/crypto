## Reading (verdicts by the protocol's rules; measured unless marked derived)

Verdict wording follows PROTOCOL.md §4. "Within the control distribution"
means no arm or group left the Bonferroni band and the across-arm tests did
not detect heterogeneity; it is not "no effect exists" below the stated
resolution (per curve about 18 %, per stratum about 6.5 %, class-wide
about 2 % at these sample sizes).

### P1 (prime field, class of 347,004 curves, 104 measured over 12 strata)

- **R1, per curve:** every one of the 104 curves is within the control
  distribution (largest |z| 3.03 against the threshold 3.49; none flagged).
  Pooled mean 1.020 × √(πn/2) with the 32-adding walk's expected 1.016.
- **R2, across curves:** heterogeneity not detected (KW p = 0.089, ANOVA p =
  0.101, variance ratio 1.18: the spread of the 104 curve means is 18 %
  larger than the pooled single-solve spread predicts, inside what 104
  means of 100 skewed solves produce). The A/A replica (z = 0.70) and the
  three isomorphic models (z = 2.58, 1.00, 0.54) are inside the band; the
  first isomorphic model is the most extreme arm of the whole class mode,
  which is the right calibration: a change of coordinates reaches |z| = 2.6
  at 100 seeds.
- **R3, by stratum / level / distance / degree / kernel field:** all 12
  endomorphism-ring strata (conductors 1 … 315) are within ±5 % of the
  reference with |z| ≤ 1.6; the three distances from the root, every
  parent degree ℓ ∈ {3, 5, 7, 13, 19, 31, 41, 43, 47} and every kernel
  field degree d ∈ {1, 2, 3, 4, 5, 9, 15, 21, 23} likewise (|z| ≤ 1.6). The
  curves reached through the hard isogenies (ℓ = 31 … 61, kernel abscissae
  in F_{p^15} … F_{p^23}) cost the same as those reached through ℓ = 3.
- **R4, positive controls:** `plant-neg` 0.725 [0.651, 0.804] (expected
  0.707, z = −7.5): detected. `plant-r4` 1.298 [1.172, 1.440] (Teske's
  heuristic 1.137, z = +4.0): detected, and larger than the heuristic (the
  4-adding walk is further from random than the heuristic assumes; the
  dry runs and P0 show the same). The detector sees a 28 % effect at |z| ≈
  7 and a 14–30 % one at |z| ≈ 4 on a single arm.
- **R5, generators:** 6 curves × 100 generators × 25 seeds, 15,000 solves,
  all correct. On every curve the spread of the 100 generator means
  matches the null (sd 19,852–21,863 against 21,275–21,873 expected;
  variance ratios 0.84–1.04; KW p 0.44–0.92); no generator flagged (family
  600, largest |z| 3.76 against 3.93); Spearman correlations of the
  generator mean with r, bits(r), popcount(r) and x(P_i) are all |ρ| ≤
  0.12. Which generator of ⟨P⟩ carries the instance does not matter.
- **R6, shift:** 5 sources × 104 targets × 25 seeds, 13,000 solves, all
  correct. η²(target) = 0.0085, η²(source) = 0.0002; KW by target p =
  0.19, by source p = 0.37; the five sources' pooled means are 1.007–1.029
  × reference (|z| ≤ 1.3). Two of the 520 cells sit beyond the Bonferroni
  band on the low side (s1→v1343 at 0.72 and s4→v625 at 0.70, Welch z ≈
  −4.5, Mann–Whitney z −2.9 and −3.1): both targets are unremarkable in
  the other four rows and in the class mode (v625 1.013, v1343 1.036), so
  this is the low tail of 520 small cells, not a target or source effect.
  Where the instance was drawn does not matter.

### P0 (j = 0 crater, class of 38,472 curves, 45 measured over 12 strata)

- **R1–R3:** all 45 curves within the control distribution (largest |z|
  2.42 against 3.26); heterogeneity not detected (KW p = 0.16, variance
  ratio 1.13); every stratum (conductors 1 … 36), every level of the
  2-volcano (0 … 4) and of the 3-volcano (0 … 2) within ±6 % with |z| ≤
  1.7. Controls inside the band (iso 1.17, 0.87, 0.40; A/A −0.83).
- **R4:** `plant-neg` 0.634 [0.567, 0.709] (z = −10); `plant-r4` 1.265
  [1.136, 1.411] (z = +3.7); **`fold-aut` 0.439 [0.401, 0.481] against the
  expected 1/√6 = 0.408** (z = −25): the order-6 automorphism group of the
  crater curve, folded into the walk, cuts the operation count by 2.28×.
  That fold exists only on the j = 0 curve: every other curve of the class
  has Aut = ±1, and the unfolded walk costs the same on all 45. The
  isogeny carries the instance away from the one curve where the cheap
  endomorphism lives; it does not create one.

### B1 (binary field F_2^37, class of 327,080 curves, 40 measured over 6 strata)

- **R1–R3:** all 40 curves within the control distribution (largest |z|
  3.02 against 3.23); heterogeneity not detected (KW p = 0.16, variance
  ratio 1.11); the six endomorphism-ring strata (conductors 1, 7, 17, 49,
  119, 833: the crater and every order down to Z[π]) within ±12 % with
  |z| ≤ 1.8 (the crater is one curve); every level of the 7-volcano (0, 1,
  2) and of the 17-volcano (0, 1) within ±5 %; curves reached through the
  hard 23- and 31-isogenies (kernel abscissae in F_{2^{37·11}} and
  F_{2^{37·5}}, 1.3–2.4 s each to find) cost the same as those reached
  through ℓ = 7 (ratios 0.986 and 1.036, |z| ≤ 1.3). The three
  representation controls (the same curve in three other polynomial bases
  of the field) sit at 1.04, 0.90, 1.04 (|z| ≤ 2.6) and the A/A replica at
  1.16 (z = 2.2): a change of field basis or of seeds moves a 100-seed mean
  as much as any curve does.
- **R4:** `plant-neg` 0.771 [0.702, 0.847] (z = −6.4); `plant-r4` 1.300
  [1.177, 1.435] (z = +4.2). Detected.
- **R5:** 6 curves × 64 generators × 25 seeds, 9,600 solves, all correct.
  Per curve the spread of generator means is 1.07–1.15× the null (KW p
  0.12–0.42); one generator of 384 (v0, generator 59, r = 51151694715) is
  beyond the Bonferroni band on the low side (0.68 [0.54, 0.83], Welch z
  −4.5, Mann–Whitney −3.3), the all-arm KW p = 0.039. Its curve's other 63
  generators and the same r-features show nothing (|ρ| ≤ 0.27); recorded
  as the one flagged cell of this family, consistent with the low tail of
  384 means of 25 skewed solves, not reproduced elsewhere.
- **R6:** 5 sources × 40 targets × 25 seeds, 5,000 solves, all correct.
  η²(target) = 0.0089, η²(source) = 0.0003; KW by target p = 0.15, by
  source p = 0.56; no cell flagged (largest |z| 3.17 against 3.66).

### K (Koblitz E_0 over F_2^41, class of 708,153 curves, 11 measured over all 4 strata)

- **The class.** `D = 703889² · (−7)`, `703889 = 409 · 1721`, `h(−7) = 1`: the
  Koblitz curve is alone on the crater of both volcanoes, every horizontal
  edge is a loop, and the class is the crater plus `h(409²·(−7)) = 410`,
  `h(1721²·(−7)) = 1,722` and `h(703889²·(−7)) = 706,020` curves (the last
  counted directly as well as by the formula). For odd `m` every prime of
  the conductor is `≡ ±1 (mod 8)`, so there is no 3-, 5-, 7- or
  13-isogeny to walk: the only way off the crater is a 409- or a
  1721-isogeny. Their division polynomials have degree 83,640 and
  1,480,920; the torsion route found three kernels of each at each vertex
  through abscissae in `F_2^{41·204}` and `F_2^{41·215}` (eigenvalue orbit
  degrees 204 and 215), 54 Montgomery ladders of 8.3–8.8 kbit, 39 s each
  on a loaded core; a 409-kernel search took 197 s and a 1721-kernel search
  978 s (three kernels each), so the transport to a conductor-703889 curve
  is 9–15 minutes against a 0.46 s solve. Every edge verified on points;
  `#E` re-counted on all 11 curves; the duals of the sampled edges, found by
  pushing a second torsion abscissa through them, carried the shifted
  instances back to the crater.
- **R1–R3:** all 11 curves within the control distribution (largest |z|
  2.38 against 2.84); heterogeneity not detected (KW p = 0.21, ANOVA p =
  0.076, variance ratio 1.70 on 11 curves); the four strata — the Koblitz
  crater (0.996), conductor 409 (1.009), 1721 (1.001) and 703889 (0.978)
  — within ±2.5 % of √(πn/2) with |z| ≤ 0.8; both levels of both volcanoes
  likewise. Representation controls (three other polynomial bases) 0.98,
  1.03, 0.96; A/A 0.97. The generic walk does not know it is on a Koblitz
  curve.
- **R4:** `plant-neg` 0.769 [0.705, 0.838] (z = −5.9); `plant-r4` 1.358
  [1.221, 1.508] (z = +4.7). **`fold-frob` 0.155 [0.141, 0.170] against the
  expected 1/√41 = 0.156 (z = −48); `fold-frobneg` 0.113 [0.103, 0.124]
  against 1/√82 = 0.110 (z = −53).** Each folded step costs 41 squarings
  more (1.92 and 2.05 µs per step against 0.50 µs), and the fold still
  wins in wall time: 275 ms and 212 ms per solve against 459 ms. The fold
  needs `τ: (x, y) ↦ (x², y²)` to be an endomorphism, which it is only when
  the curve is defined over F_2; every other curve of the class has
  `b ∉ F_2`, no such map, and the same plain cost. The 409- and
  1721-isogenies carry the instance away from the only curve where the
  √(2m) speed-up exists.

- **R5:** 3 curves × 64 generators × 25 seeds, 4,800 solves, all correct.
  Generator means spread as the null predicts on every curve (sd 94,978–
  102,029 against 95,141–102,210 expected; variance ratios 1.00–1.04; KW p
  0.34–0.72); no generator flagged (family 192, largest |z| 3.52 against
  3.65); |ρ| ≤ 0.20 with every generator feature.
- **R6:** 3 sources (the crater, a conductor-409 and a conductor-1721
  curve) × 11 targets × 25 seeds, 825 solves, all correct. η²(target) =
  0.016, η²(source) = 0.0008; KW by target p = 0.36, by source p = 0.49; no
  cell flagged (largest |z| 3.11 against 3.17).

### Across the four classes

Everything the protocol allowed the data to say about the curve, the
stratum, the level, the route, the generator and the source came back
*within the control distribution*; every positive control came back
*detected*, at the size the theory of the fold predicts. The spread of
curve means was 1.1–1.7× the pooled single-solve prediction (1.18, 1.13,
1.11, 1.70 on 104, 45, 40, 11 curves); the largest is the smallest family,
and no single curve carried it. The 32-adding walk itself ran 0.5–2.0 %
above √(πn/2) in every class, consistent with Teske's `1/√(1 − 1/32)`.


