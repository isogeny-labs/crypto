# GF(8) cubic degree-three cover screen

26 October 2026. Conductor T-5419. The prior
[GF(8) binary cubic screen](../ghs_binary_cubic_isogeny_20261026/REPORT.md)
identified class-C sources of orders `502=2*251` and `514=2*257`
with cataloged genus-three Jacobians but no evaluated transfer.
The exact characteristic-two degree-three Artin--Schreier criterion
and both complete branch-support patterns are established in the
[GF(16) follow-up](../ghs_degree3_neighbor_screen_q16_20261026/REPORT.md).

## Inputs and checks

- Complete normalized ordinary source cohort over Conway `K=GF(512)`
  in [`results_q8.json`](../ghs_binary_cubic_isogeny_20261026/results_q8.json).
- Hash-verified local LMFDB `public.av_fq_isog` archive completed
  25 October 2026. Extract exact rows for labels `3.8.a_a_al` and
  `3.8.a_a_b`, preserving all curve equations and source hashes.
- For every listed smooth genus-three hyperelliptic model with four
  distinct genuine branch points splitting over `K`, check all
  separable degree-two elliptic quotients via the complete lift
  equation and all separable degree-three maps to every normalized
  source in its order class via `t^2+t=f/h^2+R+a2+a6/R^2`.
- Retain every skipped model/pair and every full `R,t` witness.
  Recount exact elliptic group orders. Use constructed genus-three
  positive controls for both degree-three branch patterns.

## Prior-work and claim boundary

[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
cover the direct binary cubic GHS class and isogeny walks.
[Tian--Li--Wang--Yu](https://doi.org/10.1007/s10623-018-0457-6)
construct degree-three covers for cofactor-two curves in odd
characteristic. [Frey--Kani](https://doi.org/10.1007/978-3-642-25261-7_1)
describe prior genus-three correspondences including ordinary
characteristic-two Jacobians. The source orders and Jacobian classes
are already in the previous project report and LMFDB. A map would
still require subgroup-preserving transfer to `GF(8)`, complete target
solve, and same-target rho cost before a vulnerability claim.
