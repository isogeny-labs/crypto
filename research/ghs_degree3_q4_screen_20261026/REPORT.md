# GF(4) cubic degree-two and degree-three cover bound

26 October 2026. Conductor T-5414. The [protocol](PROTOCOL.md),
[hash-verified catalog extraction](extract_catalog.py),
[frozen catalog rows](catalog_models_raw.json), [exact screen](screen.py),
[certificate](certificate.json), and [Lean arithmetic](Counts.lean)
extend the characteristic-two cover analysis to the two leading
non-direct GF(4)/GF(64) classes in the earlier
[binary cubic screen](../ghs_binary_cubic_isogeny_20261026/REPORT.md).

## Exact finite bound

Let `k=GF(4)` and `K=GF(64)` in the Sage Conway models. The complete
normalized ordinary source census has nine class-C elliptic equations
of order `58=2*29` and six of order `62=2*31`. Each has the form
`E(a,b):v^2+u*v=u^3+a*u^2+b`, with `a=K.from_integer(8)`. The
Conway encodings of `b` are:

| Source order | `b` values | LMFDB genus-three class |
| ---: | :--- | :--- |
| 58 | `2,4,16,19,22,35,50,52,55` | [`3.4.a_a_ah`](https://www.lmfdb.org/Variety/Abelian/Fq/3/4/a_a_ah) |
| 62 | `8,27,41,43,47,63` | [`3.4.a_a_ad`](https://www.lmfdb.org/Variety/Abelian/Fq/3/4/a_a_ad) |

The verified local LMFDB archive lists four genus-three hyperelliptic
equations in each class. All eight equations pass the smooth genus-three,
four distinct `K`-rational branch points, and nonzero simple-pole gates;
the certificate retains every exact equation. **None has a separable
degree-two elliptic quotient over `K`, and none has a separable
degree-three map over `K` to any normalized elliptic source in its
corresponding order class.** For these eight listed models and their
corresponding source isogeny classes, a separable curve morphism over
`K`, if one exists, therefore has degree at least four.

| Finite audit | Count |
| :--- | ---: |
| Listed genus-three hyperelliptic models | 8 |
| Exact source/model pairs | 60 |
| Degree-three four-support functions | 1,440 |
| Degree-three five-support functions | 29,280 |
| Degree-three maps | 0 |
| Catalog-model skips | 0 |

The degree-two screen enumerates all projective branch-set symmetries
and solves the complete polynomial lift equation over `GF(64)`. The
degree-three screen applies the Artin--Schreier class criterion and
the two complete branch-support patterns proved in the
[GF(16) analysis](../ghs_degree3_neighbor_screen_q16_20261026/REPORT.md).
The simple-pole coefficient fixes the scalar of each rational map.
Constructed genus-three degree-three covers to the order-58 source
pass for both patterns; the five-support positive control has a
hidden double point with Conway encoding `8`, and the enumerator
recovers its map. These controls are over `K`; they do not demonstrate
descent to `k` or an attack advantage.

## Prior work and limits

The two Jacobian isogeny classes and all eight hyperelliptic equations
are prior LMFDB data. [Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
cover direct binary cubic GHS and isogeny walks.
[Tian--Li--Wang--Yu](https://doi.org/10.1007/s10623-018-0457-6)
give cofactor-two degree-three covers in odd characteristic, and
[Frey--Kani](https://doi.org/10.1007/978-3-642-25261-7_1)
provide prior correspondence methods applicable to ordinary
characteristic-two genus-three Jacobians. This class-wide exclusion
is a new finite calculation in this project; its publication novelty
has not been established.

The bound concerns the eight listed hyperelliptic models and the
degree-two and degree-three **curve-map** routes. It leaves unlisted
models, nonhyperelliptic quartics, other polarizations, degree-four
or higher covers, and non-cover isogenies/correspondences open. It
does not supply a subgroup-preserving transfer, a complete target
solve, or a same-target cost comparison. The number of new GHS
vulnerabilities demonstrated here is zero.

`lean Counts.lean` checks the finite candidate counts and coprimality
of the source subgroup orders with three. It does not formalize the
branch classification or certify an algebraic map.
