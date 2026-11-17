# Degree-three cover bound for an order-3974 binary cubic isogeny class

26 October 2026. Conductor T-5410. The [protocol](PROTOCOL.md),
[Sage screen](screen_neighbors.py), [certificate](certificate.json),
and [Lean count checks](NeighborCounts.lean) extend the
[exact-curve cover criterion](../ghs_automorphism_screen_q16_20261026/REPORT.md)
from one source elliptic equation to every normalized ordinary equation
of order 3974 in the frozen `GF(4096)` census.

## Finite result

Let `k=GF(16)`, `K=GF(4096)`, both in Sage's Conway basis. The source
isogeny class has **twelve** `K`-isomorphism classes represented by

```text
E_b/K: v^2+u*v=u^3+K.from_integer(32)*u^2+K.from_integer(b),
b in {306,691,2082,2268,2628,2896,2978,2996,3347,3412,3505,3978}.
```

The census covers both absolute-trace representatives of `a2` and
every nonzero `a6`, and the replay independently calls Sage's
cardinality routine for all twelve equations. Each has
`#E_b(K)=3974=2*1987`. Every elliptic curve `K`-isogenous to the
exact witness is ordinary, has this order, and is `K`-isomorphic to
one of these twelve normalized forms.

The [LMFDB class 3.16.a_a_aet](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aet)
has four catalog-listed genus-three hyperelliptic models over `k`.
Their exact equations are in the [frozen LMFDB snapshot](../ghs_cubic_quotient_obstruction_20261026/lmfdb_curves_3974.txt)
and the preceding [four-model certificate](../ghs_automorphism_screen_q16_20261026/degree3_pattern_4models.json).
**None of these four curves admits a separable degree-three morphism
over `K` to any elliptic curve in the source's `K`-isogeny class.**
The preceding geometric proof also excludes separable degree-two
elliptic quotients of these four curves over every constant extension.
Consequently a separable morphism from one of these four catalog
models to an elliptic curve in the order-3974 source class over `K`,
if one exists, has degree **at least four**.

| Exact search count | Value |
| :--- | ---: |
| Source equations × listed Jacobian models | 12 × 4 = 48 |
| `(2,1)/(2,1)` four-support rational functions | 1,152 |
| `(2,1)/(1,1,1)` five-support rational functions | 1,571,712 |
| Five-support functions passing simple-pole residues | 0 |
| Degree-three maps found | 0 |

## Why this is a complete degree-three test for these 48 pairs

For `C:y^2+h(x)y=f(x)` and
`E_b:v^2+u*v=u^3+a2*u^2+a6`, a map of degree three can be translated
to take a `K`-rational hyperelliptic branch point of `C` to the identity
of `E_b`. It then commutes with the two involutions. Its `u` coordinate
is a separable rational function `R(x)` of degree three, and the
Artin--Schreier classes must match:

```text
f(x)/h(x)^2 + R(x) + a2 + a6/R(x)^2 = t(x)^2+t(x),
t in K(x).
```

Writing `b=sqrt(a6)`, the source class is represented by
`u+a2+b/u`, with simple branch poles at zero and infinity.
The four branch points of every listed `C` are distinct, finite,
`K`-rational, and simple. Pullback branch poles can only lie over
zeros or poles of `R`. Multiplicity three would leave an order-three
pole and is impossible. The only zero/pole partitions that yield four
simple branch poles are `(2,1)/(2,1)` with all four preimages
branching, or `(2,1)/(1,1,1)` with its double preimage unbranched.
The lone hidden double point in the second pattern is `K`-rational
because it is unique and Galois-stable; the screen includes infinity.

At one known simple preimage, equality of the local simple-pole
coefficients determines the scalar of `R`. The first pattern has 24
branch-point assignments per pair. The second has four assignments,
two orientations, and `4096-4+1=4093` hidden-point choices per pair.
The Sage replay checks all of them. It solves the remaining full
Artin--Schreier equation by eliminating even poles and testing the
constant absolute trace. The prior screen checks the fast residue
formulas against direct Laurent coefficients. A constructed
genus-three degree-three cover to the exact order-3974 source passes
for **each** branch pattern. In the five-support control, the hidden
double point has Conway encoding `8`; the enumerator recovers a map
with scalar `1701` and verifies the Artin--Schreier identity. These
controls make an all-negative parser explanation implausible. The
certificate hashes every frozen input and script.

## Prior work and claim boundary

[Tian--Li--Wang--Yu (2018)](https://doi.org/10.1007/s10623-018-0457-6)
construct degree-three covers for cofactor-two elliptic curves in odd
characteristic. [Tian (2020/2023)](https://arxiv.org/pdf/2012.07173)
constructs an evaluated cubic Weil-restriction isogeny route, with
displayed formulas requiring full geometric 2-torsion.
[Frey--Kani (2011/2012)](https://doi.org/10.1007/978-3-642-25261-7_1)
describe genus-three correspondences with a characteristic-two
ordinary-Jacobian application. The four curve equations are already
cataloged. This finite class bound is new within this project;
publication novelty has not been established.

The degree-four lower bound applies to separable curve morphisms from
the **four listed** hyperelliptic models to the **twelve normalized**
elliptic sources over `K`. It does not rule out a different
polarization or unlisted curve in the Jacobian isogeny class, a
nonhyperelliptic genus-three curve, a higher-degree cover, or an
isogeny/correspondence that is not induced by a curve morphism.
No evaluated source-to-Jacobian subgroup map or end-to-end attack cost
is implied. The number of newly demonstrated vulnerable source curves
from this screen is zero.

`lean NeighborCounts.lean` checks the finite counts and
`gcd(3,1987)=1`; it does not formalize the branch classification,
finite-field computations, or existence of an isogeny map.
