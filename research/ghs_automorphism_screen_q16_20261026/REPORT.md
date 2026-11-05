# Degree-two quotient bound for the GF(16) cubic catalog

26 October 2026. Conductor T-5399. This report follows the
[protocol](PROTOCOL.md) and uses the
[archived catalog extraction](extract_catalog.py),
[128-row curve snapshot](catalog_models_raw.json),
[sparse-family audit](audit_sparse.py),
[full lift screen](screen_degree2.py),
[sparse certificate](sparse_certificate.json),
[nonsparse certificate](degree2_all_nonsparse_certificate.json), and
[degree-three cover screen](screen_degree3_pattern.py),
[degree-three certificate](degree3_pattern_4models.json), and
[Lean count checks](CatalogCounts.lean).

## Exact finite result

The verified local LMFDB `av_fq_isog` archive lists 10,664 genus-three
hyperelliptic equations in the 128 Weil-restriction isogeny classes of
the normalized ordinary `E/GF(4096)` census. There are 3,516 equations
in orders belonging to the known direct class-B GHS closure and **7,148
equations in orders outside it**. The 7,148 lie in 90 catalog-positive
classes; they are curve equations over `k=GF(16)`, not 7,148 distinct
source elliptic curves.

**None of those 7,148 catalog-listed outside-direct hyperelliptic curves
has a separable degree-two morphism to an elliptic curve over
`K=GF(4096)`.** This is a finite, reproducible exclusion of one cover
route. It neither excludes higher-degree maps or Jacobian
correspondences nor evaluates the Tate isogenies in these classes.

| Cataloged genus-three models | Direct-B orders | Outside-direct orders |
| :--- | ---: | ---: |
| All listed hyperelliptic models | 3,516 | **7,148** |
| `h=alpha*x^4+beta*x`, `alpha*beta != 0` | 456 | **876** |
| Sparse `h`, zero coefficient of `x^3` in `f` | 104 | **0** |
| Sparse `h`, nonzero coefficient of `x^3` in `f` | 352 | **876** |
| Other listed models | 3,060 | **6,272** |

The 876 sparse outside-direct equations span 72 source-order classes.
For each one, Sage checks the coefficient pattern, smooth genus three,
all four genuine branch points, and the 11 nonidentity affine
branch-set symmetries. The [family proof](../ghs_cubic_quotient_obstruction_20261026/REPORT.md)
then establishes `Aut(C_bar)=C2`, so these 876 lack a degree-two
elliptic quotient even after extending constants. The remaining 6,272
equations have all four branch roots in `K`; an exact `GF(2)` linear
solve checks every projective branch-set symmetry and finds no
nonidentity lift over `K`.

## Completeness of the finite lift test

For `C: y^2+h(x)y=f(x)` with `deg h=4`, `deg f<=8`, four distinct genuine
finite branch points, and genus three, any automorphism preserves the
unique hyperelliptic involution and descends to a projective linear map
`T(x)=(A*x+B)/(C*x+D)` permuting those four points. Write
`den=C*x+D`, `H=den^4*h(T)`, and `F=den^8*f(T)`; the branch permutation
implies `H=e*h` for some nonzero constant `e`. A lift has the form

```text
x -> T(x),
y -> (e*y+u(x))/den^4,
u(x)^2+e*h(x)*u(x) = F(x)+e^2*f(x).             (1)
```

Here `u` is a polynomial of degree at most four. Indeed, away from
`den=0`, a pole of `u` at finite `x` would create a pole of the image
`y` above a finite image `x`; at `den=0`, multiplying the image `y` by
`den^4` cancels its at-most-fourth-order pole at infinity. At the
original infinity, both terms defining `u` have pole order at most
four. Thus the coefficient search in (1) is complete for lifts defined
over `K`. The audit enumerates all 24 branch-point permutations, tests
which are induced by a projective linear map, solves (1) as a linear
system over `GF(2)` on the 60 bits of the five `K` coefficients of `u`,
and substitutes every found solution back into the curve equation.

A separable degree-two morphism to a genus-one curve would have a
nonhyperelliptic deck involution over `K`, so absence of nonidentity
lifts excludes such a morphism. The argument is about maps from these
specific models. A different curve or polarization in the same
isogeny class can behave differently.

## Positive control and exact witness

The known direct-B class `3.16.a_a_aex` supplies a positive control:

```text
y^2+(a^2*x^4+a*x)*y=x^8+(a^3+a^2+1)*x^2+a^2+1,
a^4+a+1=0.
```

The screen finds three nontrivial lifts over `K`, each `x -> x+d`,
`y -> y`, with Conway-encoded `d` values `1609`, `2915`, and `3370`.
It verifies each lift by substitution. This control is in the
published direct-cover closure, not a new vulnerable class.

The outside-direct source witness remains

```text
E/GF(4096): y^2+x*y=x^3+K.from_integer(32)*x^2+K.from_integer(306),
#E(K)=3974=2*1987, P=(K.from_integer(307),K.from_integer(2825)).
```

Its four cataloged genus-three models and their independently
recomputed `(17,257,3728)` point counts are in the
[prior certificate](../ghs_cubic_quotient_obstruction_20261026/certificate.json).
All four satisfy the geometric sparse-family obstruction.

## Prior work, bound, and limits

[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
provide the direct binary cubic cover and isogeny-walk context.
[Nart--Ritzenthaler](https://arxiv.org/pdf/0905.0546) provide explicit
characteristic-two genus-three elliptic quotients from involutions.
[Borodin--May](https://arxiv.org/pdf/2508.16885) survey genus-three
Jacobian existence obstructions. The LMFDB equations themselves are
prior catalog data. The new contribution within this project is the
complete archived-model cross-index and its checked quotient exclusion;
publication novelty of the coefficient theorem or finite partition has
not been established.

The degree-three cofactor-two route also has explicit prior work:
[Tian--Li--Wang--Yu (2018)](https://doi.org/10.1007/s10623-018-0457-6)
construct genus-three hyperelliptic degree-three covers and give attacked
examples **in odd characteristic**. Thus neither the cofactor-two condition
nor degree three is a novel attack family by itself. [Tian (2020/2023)](https://arxiv.org/pdf/2012.07173)
constructs and evaluates isogenies from cubic Weil restrictions to
genus-three Jacobians without a covering morphism. Its displayed
construction uses a short Weierstrass model `y^2=x^3+a4*x+a6` and a
symplectic basis of full geometric 2-torsion, so those formulas cannot be
applied unchanged to the present ordinary binary curves. This is a
methodological gap to test, not evidence of an unreported vulnerability.
[Frey--Kani (2011/2012)](https://doi.org/10.1007/978-3-642-25261-7_1)
also describe correspondence methods from ordinary characteristic-two
genus-three hyperelliptic Jacobians to nonhyperelliptic Jacobians. Thus
an isogeny-only route must be checked against that earlier construction
before it can receive a new-method label.

The bound here can also be stated positively as a **minimum map degree**:
any separable morphism from one of the 7,148 listed outside-direct models
to an elliptic curve over `K` has degree at least three. For the 876 sparse
models, the same lower bound holds after every constant-field extension.
The zero count for degree two is exact within the frozen catalog, but the
number of source elliptic curves vulnerable through degree three or an
evaluated isogeny is not determined by this screen.

## Exact degree-three cover gate for the binary witness

For the exact source `E/K: v^2+u*v=u^3+a2*u^2+a6` and a listed
`C/K: y^2+h(x)*y=f(x)`, put `z=y/h(x)`. A separable degree-three
map `C -> E` defined over `K` exists if and only if there are a
separable rational function `R in K(x)` of degree three and a rational
function `t in K(x)` satisfying

```text
t(x)^2+t(x) = f(x)/h(x)^2 + R(x) + a2 + a6/R(x)^2.       (2)
```

The resulting map is `u=R(x)`, `v=R(x)*(z+t(x))`. Equation (2) is an
exact Artin--Schreier class-matching test for any proposed degree-three
cover, rather than a point-count or isogeny-class proxy. To see the
necessity, choose one `K`-rational hyperelliptic branch point of `C`
(all four are `K`-rational in the screened models), translate its image
on `E` to the identity, and use that the hyperelliptic involution acts
as negation on the Jacobian. Then `u` is invariant under the involution,
so `u=R(x)`; the two Artin--Schreier coordinates differ by an invariant
function `t(x)`. The quotient diagrams give `deg R=deg(C -> E)=3`.
Conversely, (2) defines an inclusion of function fields that extends to
a morphism of smooth projective curves of degree three. This criterion
provides a complete finite search for degree-three maps to the exact
source `E` in the four cataloged witness models:

```text
source E/K: a2=K.from_integer(32), a6=K.from_integer(306)
source order: 3974=2*1987
catalog class: 3.16.a_a_aet
models: 4
four-support R functions tested: 4*24 = 96
five-support R functions tested: 4*4*2*(4096-4+1) = 130976
degree-three maps C -> this E over K found: 0
```

Here is the completeness argument for that search. Write
`b=sqrt(a6)`. The Artin--Schreier class of `E` is represented by
`u+a2+b/u`, which has one simple branch pole at `u=0` and one at
`u=infinity`. Its pullback by `R` can branch only at zeros and poles of
`R`. A zero or pole of multiplicity three creates a pole of odd order
three, which cannot match the four simple branch poles of any screened
model. The remaining multiplicity partitions of three are `(2,1)` and
`(1,1,1)`. A simple preimage always branches. A double preimage may
branch simply or become unbranched after an Artin--Schreier shift.
Consequently four simple branch poles can arise in exactly two patterns:

1. Both fibers have `(2,1)` and all four preimages branch. Up to a
   nonzero scalar and permutation of the four known branch points,
   `R=(x-r0)^2*(x-r1)/((x-r2)^2*(x-r3))`.
2. One fiber has `(2,1)` with its double point unbranched, and the
   other has `(1,1,1)`. The hidden double point lies in
   `P^1(K)` outside the four branch points; it may be infinity. There
   are four choices of the lone simple preimage, two fiber
   orientations, and `4096-4+1=4093` hidden points per model.

In both patterns the coefficient of the simple pole at a known branch
point fixes the scalar of `R` uniquely. The script compares that
coefficient at every remaining simple preimage. It then tests (2) by
exact rational-function arithmetic over `K`; all 96 functions in the
first pattern fail (2), and none of the 130,976 functions in the second
pattern even passes the other simple-pole residue equations. The
script checks its fast residue formulas against direct rational
Laurent coefficients, and a constructed genus-three degree-three
cover to the *same* `E` passes the full Artin--Schreier test as a
positive control.

This excludes degree-three covers **to the exact witness elliptic
curve** from those four listed genus-three models over `K`. It does not
exclude a degree-three map to a different elliptic curve in its isogeny
class, a map of degree at least four, an unlisted genus-three model, or
an isogeny-only transfer. In particular, it is an obstruction result,
not a newly demonstrated discrete-log vulnerability.

The local archive reports 3,124 complete hash-verified chunks, with
completion on 25 October 2026; it is a cross-time mirror rather than a
transactionally consistent database snapshot. The extractor matches all
128 labels to the frozen source census and checks each curve-list length
against `hyp_count`. The two classes with Jacobian status zero have no
listed hyperelliptic equations and are **unknown**, not ruled out.

The 7,148 exclusion concerns the catalog-listed hyperelliptic models
over `GF(16)` and separable degree-two maps defined over `GF(4096)`.
For the 6,272 nonsparse models it does not exclude a map over a larger
constant extension. It does not classify nonhyperelliptic genus-three
curves, all polarizations, degree-three or higher covers, or arbitrary
correspondences. No evaluated map from the outside-direct source
subgroups, full transfer attack, or same-target rho timing follows from
this result. The rigorous numerical bound on **demonstrated** new
GHS-vulnerable source elliptic curves remains zero.

`lean CatalogCounts.lean` checks the count identities only. The written
automorphism argument and Sage finite-field replays are not encoded as
Lean proofs of algebraic geometry.
