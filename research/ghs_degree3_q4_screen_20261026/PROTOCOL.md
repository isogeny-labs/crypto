# GF(4) cubic degree-three cover screen

26 October 2026. Conductor T-5414. The prior
[GF(4) binary cubic screen](../ghs_binary_cubic_isogeny_20261026/REPORT.md)
found cataloged genus-three Jacobians in non-direct source order classes
58 and 62, but no evaluated maps. The
[GF(16) Artin--Schreier map criterion](../ghs_degree3_neighbor_screen_q16_20261026/REPORT.md)
gives an exact test for degree-three covers from hyperelliptic models.

## Frozen inputs

- Complete 126-model normalized ordinary elliptic cohort over
  Conway `K=GF(64)` in
  [`results.json`](../ghs_binary_cubic_isogeny_20261026/results.json).
- The hash-verified local LMFDB `public.av_fq_isog` archive completed
  25 October 2026. Extract the `curves` arrays for the exact genus-three
  `GF(4)` class labels at source orders 58 and 62; retain archive hashes,
  curve-list counts, and catalog status. These equations are prior data.
- Elliptic sources in each selected order class, with their exact
  normalized coefficients and largest-prime subgroup order.

For every listed smooth hyperelliptic model with four distinct simple
branch points over `K`, test all separable degree-three rational maps
`R in K(x)` in both complete zero/pole support patterns. A map is
accepted only with a rational `t` satisfying
`t^2+t=f/h^2+R+a2+a6/R^2`, and with substitution verification.
Retain all skips and maps. Recount the source group and curve genus.
For the same eight cataloged hyperelliptic models, enumerate all
branch-set projective transformations and solve the exact degree-two
lift equation over `K`, recording every nonhyperelliptic involution.

## Prior work and claim boundary

[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
cover direct binary cubic genus-three GHS and isogeny walks.
[Tian--Li--Wang--Yu](https://doi.org/10.1007/s10623-018-0457-6)
give degree-three cofactor-two covers in odd characteristic.
[Frey--Kani](https://doi.org/10.1007/978-3-642-25261-7_1)
give prior genus-three correspondence methods with ordinary
characteristic-two cases. An explicit map from a cataloged model is
not by itself a new vulnerability: a subgroup-preserving direction,
descent to `GF(4)`, full target solve, and measured cost below
same-target rho are required.
