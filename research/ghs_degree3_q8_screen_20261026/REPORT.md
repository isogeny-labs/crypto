# GF(8) cubic degree-two and degree-three cover bound

26 October 2026. The [protocol](PROTOCOL.md), [catalog extractor](extract_catalog.py),
[frozen rows](catalog_models_raw.json), [Sage screen](screen.py),
[certificate](certificate.json), and [Lean arithmetic](Counts.lean) audit the
two leading non-direct GF(8)/GF(512) classes from the earlier
[binary cubic screen](../ghs_binary_cubic_isogeny_20261026/REPORT.md).

## Exact finite bound

Let `k=GF(8)` and `K=GF(512)` in Sage's Conway models. The complete
normalized ordinary source census contains 18 class-C elliptic equations
of order `502=2*251` and 18 of order `514=2*257`. Each is
`E(1,b): v^2+u*v=u^3+u^2+b`. The Conway encodings are:

| Source order | `b` values | Cataloged genus-three class |
| ---: | :--- | :--- |
| 502 | `9,65,100,112,137,143,147,149,186,230,269,288,308,331,343,345,426,502` | [`3.8.a_a_al`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_al) |
| 514 | `2,4,16,82,101,142,148,165,169,187,196,231,256,268,289,342,344,396` | [`3.8.a_a_b`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_b) |

The hash-verified local LMFDB archive lists 15 and 9 genus-three
hyperelliptic equations in these classes. All 24 pass the smooth genus-three,
four distinct `K`-rational branch point, and nonzero simple-pole gates.
**None has a separable degree-two elliptic quotient over `K`, and none
has a separable degree-three map over `K` to any of the 18 normalized
elliptic sources in its corresponding order class.** Thus any separable
curve morphism from one of these 24 listed models to an elliptic curve in
its source isogeny class has degree at least four, provided the target is
represented by the normalized census or a degree-one isomorph thereof.

| Finite audit | Count |
| :--- | ---: |
| Listed genus-three hyperelliptic models | 24 |
| Exact source/model pairs | 432 |
| Degree-three four-support functions | 10,368 |
| Degree-three five-support functions | 1,759,104 |
| Total degree-three functions | 1,769,472 |
| Skipped pairs; maps found | 0; 0 |

The degree-two screen enumerates projective branch-set symmetries and
solves the polynomial lift equation. The degree-three screen uses the
Artin--Schreier criterion and both branch-support patterns derived in the
[GF(16) analysis](../ghs_degree3_neighbor_screen_q16_20261026/REPORT.md).
Its constructed covers to an order-502 source pass for both patterns;
the five-support control's hidden double point has Conway encoding `8`,
and the enumerator recovers two representations of its map. These controls
are over `K` and are not evidence of descent to `k` or attack advantage.

## Prior work and limits

The isogeny classes and all listed equations are prior LMFDB data.
[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
analyze direct binary cubic GHS and isogeny walks. The odd-characteristic
cofactor-two degree-three construction of
[Tian--Li--Wang--Yu](https://doi.org/10.1007/s10623-018-0457-6)
and [Frey--Kani's genus-three correspondences](https://doi.org/10.1007/978-3-642-25261-7_1)
remain relevant prior routes. This finite exclusion is a new calculation
in this project; publication novelty has not been established.

The bound covers the 24 **listed hyperelliptic models** and degree-two
and degree-three **curve-map** routes. It does not cover unlisted models,
nonhyperelliptic quartics, other polarizations, degree-four or higher
covers, or non-cover isogenies/correspondences. In particular, it neither
identifies a new vulnerable family nor supplies a subgroup-preserving
transfer, full target solution, or same-target rho comparison.

`lean Counts.lean` checks exact candidate counts and subgroup-order
coprimality with three. It does not formalize the geometric completeness
argument or certify a curve map.
