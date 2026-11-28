# H2 direct-map obstruction and plane-section quotient check — 2026-11-19

## Result

A regular map from the Hermitian surface `H(3,p²)` to an elliptic curve is
constant on its `F_(p²)`-rational points. Every Hermitian generator line is
a projective line, and a morphism from `P¹` to an abelian variety is
constant. The rational point-line incidence is a generalized quadrangle:
each point lies on a line, and for any point `P` off a line `L`, a point
`R` on `L` is collinear with `P`. Therefore the map has the same value at
`P`, `R`, and any chosen point of `L`. This gives a direct obstruction to
the proposed pointwise push-forward of Hermitian line incidence through
a regular surface-to-elliptic-curve map. It leaves curve correspondences,
rational point encodings, and other non-morphism constructions to be
specified and tested separately.

The [Hermitian generalized-quadrangle description](https://cpb-us-w2.wpmucdn.com/sites.udel.edu/dist/a/6387/files/2017/08/hemisys_2-1j4fiux.pdf)
supplies the incidence property. The constant-map fact for `P¹` and
abelian varieties is stated in [Milne's *Abelian Varieties* notes](https://jmilne.org/math/CourseNotes/AV.pdf)
and [Bhatt's lecture notes](https://www.math.ias.edu/~bhatt/teaching/mat731f17/lectures.pdf).

## A second necessary condition for plane-section quotients

A nonsingular Hermitian plane section is a Hermitian curve `C` of genus
`p(p−1)/2` with `p³+1` rational points over `F_(p²)`. It is maximal, so
every Frobenius eigenvalue on its Jacobian is `−p`; an elliptic quotient
over `F_(p²)` must therefore have trace `−2p` and order `(p+1)²`.
[Pries and collaborators](https://www.math.colostate.edu/~pries/Preprints/15HermitianEOtypeAJM.pdf)
record the maximality and supersingularity of this Hermitian curve.

The independently recounted Legendre fibres from the three completed H2
cells are:

| `p` | `λ` | `#E_λ(F_(p²))` | Trace | Required `(p+1)²` | Meets necessary count? |
| ---: | ---: | ---: | ---: | ---: | --- |
| 7 | 5 | 48 | 2 | 64 | No |
| 11 | 2 | 144 | −22 | 144 | Yes |
| 13 | 2 | 160 | 10 | 196 | No |

The `p=11` count passes this necessary condition; a matching count alone
does not construct a quotient. The `p=7` and `p=13` curves cannot be
elliptic quotients of those maximal Hermitian plane-section Jacobians.
This criterion concerns the plane-section quotient route and is separate
from the surface point-map theorem above.

[`protocol.json`](protocol.json) fixed these three input cells and the
claim boundary before execution. [`run.py`](run.py) uses the accepted
checked Sage launcher to count points two ways: direct square testing and
`EllipticCurve.cardinality()`. Both agree with each other and with the
original H2 result. [`result.json`](result.json), [`runtime-info.json`](runtime-info.json),
and [`run-output.txt`](run-output.txt) retain the run and runtime identity.
The original H2 surrogate rank data remain unchanged.

## Lean proof map

| Mathematical step | Lean declaration | Exact status |
| --- | --- | --- |
| A line-constant map on an incidence geometry with a line through each point and the projection property is constant on all points | `H2DirectMap.map_constant_of_projection` in [`LineMapObstruction.lean`](formal/LineMapObstruction.lean) | Kernel checked, no axioms |
| The rational points and generator lines of `H(3,p²)` satisfy the two incidence hypotheses | — | Established in the written generalized-quadrangle argument; concrete finite-geometry Lean definitions and proof are open |
| A regular morphism from a generator line `P¹` to an elliptic curve is constant | — | Standard abelian-variety theorem cited above; algebraic-geometry Lean bridge is open |
| Maximal Hermitian plane section forces trace `−2p` for an elliptic Jacobian quotient | — | Written Weil-eigenvalue proof above; curve/Jacobian/Frobenius formalization is open |

The Lean theorem preserves all quantifiers and hypotheses of the abstract
structural implication; it does not claim that its concrete algebraic
geometry hypotheses have been formalized. The pin is Lean `v4.36.0` with
no external dependency. The exact build command was `lean
LineMapObstruction.lean` from [`formal/`](formal/); its output is in
[`build-output.txt`](formal/build-output.txt) and reports no axioms.
`lean4checker --fresh` was unavailable on this host. The next formal step
is a concrete Hermitian incidence model; the next experimental step is an
explicit correspondence or quotient map with signed group-law checks,
which the original surrogate did not provide.
