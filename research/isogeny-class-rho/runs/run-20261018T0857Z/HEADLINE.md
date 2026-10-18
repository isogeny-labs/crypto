**Headline (measured; the per-rule verdicts are in "Reading").** Four
classes, 85,425 rho solves on 200 curves plus controls, every solve
returning the planted `k`, every isogeny edge verified on points:

- **The curve does not matter to a generic walk.** Across 104 curves of a
  347,004-curve prime-field class (12 endomorphism-ring strata, reached
  through 3-, 5-, 7-, 13-, 19-, 31-, 41-, 43-, 47- and 61-isogenies), 45
  curves of a `j = 0` class, 40 curves of a 327,080-curve binary class and
  the Koblitz class over `F_2^41`, no curve, stratum, volcano level,
  distance, isogeny degree or kernel-field degree left the preregistered
  band; the across-curve tests found no heterogeneity; the between-curve
  spread of means is 1.1–1.2× what the single-solve spread predicts.
  Isomorphic models and other field bases of the root move a 100-seed mean
  as much as any isogenous curve does.
- **Neither does the generator or the source curve.** 600 generators of
  `⟨P⟩` on P1 (and 400, 384 on P0, B1) with the same secret: per-generator
  means spread exactly as the null predicts (variance ratios 0.84–1.15,
  KW p ≥ 0.12 per curve), no correlation with `r`, its size or weight or
  `x(P_i)`; instances drawn on five source curves and carried to every
  target: `η²` of the source 0.0002–0.0012, of the target 0.008–0.011,
  neither detectable. Isolated flagged cells (2 of 520, 1 of 384, 1 of 180)
  are the low tail of many 25-solve means and do not recur for the same
  curve in any other row.
- **The detector works, and the one real effect is the crater's own.**
  The planted negation fold gave 0.63–0.77 of the unfolded mean (expected
  0.71, |z| 6–10); the planted 4-adding walk 1.27–1.30 (|z| ≈ 4). On the
  `j = 0` crater the automorphism-folded walk cost 0.439 [0.401, 0.481] of
  the plain walk (expected 1/√6 = 0.408, z = −25); on the Koblitz crater the
  Frobenius-folded walk <!-- K-FOLD -->. Those folds exist only on the
  crater curve; every curve the isogenies lead to has lost them and costs
  the same as the rest. In wall time the Koblitz fold still wins by 1.7× (τ) and 2.2× (τ and negation), even though each folded step costs 41 squarings more (1.9–2.0 µs per step against 0.50 µs).
- **Hard isogenies are findable and change nothing.** The 409- and
  1721-isogenies of the Koblitz curve (division polynomials of degree
  83,640 and 1,480,920, out of reach) were found through torsion abscissae
  in `F_{2^{41·204}}` and `F_{2^{41·215}}` in about 1 min (409) and 5 min (1721) per kernel on a loaded core (three kernels per search: 197 s and 978 s per search); the
  curves they reach (conductor-409, -1721 and -703,889 strata of a
  708,153-curve class) cost 0.978–1.009 of the reference by stratum (|z| ≤ 0.8), and the crater curve itself 0.996. End-to-end,
  finding the isogeny is the only cost that depends on the route: 7–74 ms
  for the Elkies edges of P1 against a 98 ms solve, 1.4–3.1 s for the 23-
  and 31-isogenies of B1 against a 0.19 s solve.

Nothing here is an index-calculus baseline change (CLAUDE.md success
criterion): rho against rho, one instrument, and the only speed-ups are the
crater curves' own endomorphisms, which an isogeny removes.
