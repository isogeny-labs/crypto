# q16 genus-three subcover probe

## Result and scope

The search found **15 exact q16 source equations outside the earlier
66-equation frontier** with explicit genus-three covers over `F16`. For all
15, an exact nonzero map carries the largest prime-order source subgroup
into the base-field Jacobian, and a complete relation solve recovers the
same target logarithm used in a matched rho run. These are **eligible
transfer curves**, but on these measurements the complete transfer is
slower than rho. The 15 equations therefore do not support a new measured
vulnerability claim.

The same search also gives a bounded exclusion result: for one cataloged
genus-three curve in the order-3974 isogeny class, no direct degree-four map
to any of the 12 exact source equations was found among the 30
branch-supported exponent patterns defined below. An algebraic obstruction
eliminates 24 of those patterns for **any** target parameter. For each of
three cataloged genus-three class representatives, all 12 Möbius symmetries
of the branch set were tested and only the identity lifts to the curve.

The negative result does not exclude a degree-four map with zeros or poles
away from the branch set, a higher-degree map, a correspondence, or another
curve in the same isogeny class. The direct-cover negative search covers the
12 order-3974 source equations; the automorphism check covers one
genus-three representative in each of three source-order classes.

## Fifteen exact eligible equations and measured costs

Every source has the common form
`E_b: Y^2+X*Y=X^3+w^5*X^2+b` over `K`; `b` is the listed polynomial-basis
mask. The exact base-curve equations, source and image point coordinates,
field embeddings, map checks, and individual receipts are in
[`map15_receipt.json`](map15_receipt.json) and
[`attack15_receipt.json`](attack15_receipt.json). The
[`prior-work receipt`](prior_work_receipt.json) gives each exact equation's
cataloged isogeny class and marks its exact-publication status as
unverified. The measured "full" cost
includes curve construction, point counting, source-to-Jacobian map,
factor-base setup, relation collection, and linear solve. Rho is the mean
wall time of 1,000 independent solves of that *same* source target using
the same Sage runtime. The online cost without curve construction is also
larger than rho for every row (31.1–241 times in this run).

| `b` | `#E(K)` | Prime subgroup | Full transfer, s | Rho mean, ms | Full/rho |
|---:|---:|---:|---:|---:|---:|
| 227 | 4098 | 683 | 1.264 | 0.64 | 1976 |
| 801 | 4098 | 683 | 1.159 | 0.58 | 1994 |
| 962 | 4098 | 683 | 1.102 | 0.67 | 1647 |
| 1384 | 4162 | 2081 | 0.794 | 1.45 | 549 |
| 1419 | 4098 | 683 | 0.898 | 0.60 | 1485 |
| 1609 | 4162 | 2081 | 0.817 | 1.01 | 810 |
| 1706 | 3970 | 397 | 0.714 | 0.69 | 1031 |
| 2114 | 3970 | 397 | 1.178 | 0.65 | 1825 |
| 2209 | 4162 | 2081 | 0.866 | 1.04 | 837 |
| 2915 | 4162 | 2081 | 1.039 | 0.82 | 1269 |
| 2944 | 4098 | 683 | 0.983 | 0.80 | 1224 |
| 3370 | 4162 | 2081 | 0.950 | 1.24 | 766 |
| 3529 | 4162 | 2081 | 0.852 | 0.78 | 1088 |
| 3595 | 4098 | 683 | 1.032 | 0.56 | 1848 |
| 3816 | 3970 | 397 | 0.978 | 0.48 | 2039 |

All 15 full runs returned a verified logarithm. Timing is a local single-run
measurement for the transfer and a 100-run mean for rho; compiler/runtime
choices and setup accounting are recorded in the scripts and receipts. The
rows with `#E=4098`, `4162`, and `3970` lie in already cataloged
[LMFDB classes 3.16.a_a_b](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_b),
[3.16.a_a_cn](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_cn), and
[3.16.a_a_aex](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aex),
respectively. This is prior-work evidence at the isogeny-class level,
not proof that each exact equation or map appeared in print.

### Closed-form family and count for this construction

The six surviving exponent patterns reduce to a `V4` cover form. The
following algebraic construction works for `k=F_q`, `q=2^(2m)`, and
`K=F_(q^3)`. Choose a noncube `a` in `k`, a root `r` of
`r^3=a^-1` in `K`, and any `delta in k*`. Set

`h(x)=a^2*x^4+a*x`,
`V(x)=(x-r^q)*(x-r^(q^2))/(x*(x-r))`,
`lambda=delta^2/(a^2*r^2)`, and `b=lambda^4`.

The three nonzero roots of `h` are `r,r^q,r^(q^2)`; their sum is zero.
Multiplying out the two quadratic factors gives
`V+1/V=a*r/h(x)`. With `alpha=delta/(a*r)`, we have
`alpha^2=lambda` and `alpha*(V+1/V)=delta/h(x)`. Consequently

`C_delta: y^2+h(x)*y=delta*h(x)+d0*h(x)^2`

has four distinct simple Artin–Schreier poles and genus three. For any
`u in K` and `d0 in k` with `Tr_(K/F2)(u+d0)=0`, choose `c in K`
with `c^2+c=u+d0`. The formulas for `C_K -> E' -> E_b` in the
representative section below apply with `E_b`'s `X^2` coefficient `u`.
For fixed `a,r,u,d0`, distinct `delta in k*` give distinct `b`:
`b=delta^8/(a^8*r^8)`, and the eighth-power map permutes `k*`
because `q-1` is odd. Thus this **specific construction contains exactly
`q-1` source parameters**. It gives 15 at `q=16`; the identity and
distinctness are checked by [`family_formula.py`](family_formula.py)
for all 15. The Lean theorem `iterated_square_injective` checks the
abstract injectivity step.

This is a bound on the displayed construction, not on every curve exposed
to transfer attacks. The `V4` genus-three cover structure is already
studied in [Nart–Ritzenthaler](https://arxiv.org/abs/0905.0546), and
the published even-characteristic isogeny-condition classification also
applies. The closed formula generates and excludes exact cases, but it
does not establish a new family type or a speedup.

### Explicit transfer for `b=1384`

The source prime subgroup has order `2081`, with `G=(2411,528)` and
`Q=(2812,988)=7G` in polynomial-basis masks. Set `lambda=1609`,
`alpha=sqrt(lambda)=2593`, and `c=932`, where
`c^2+c=w^5+a^3`. With branch roots `(r0,r1,r2,r3)` of masks
`(0,1609,2915,3370)`, define
`V=(x-r2)(x-r3)/((x-r0)(x-r1))`. The even-degree base curve is

`C: y^2+(a^2*x^4+a*x)*y = (a^3+a+1)*x^8 + (a^2+a+1)*x^4 + (a^2+a)*x^2 + (a^3+a)*x`.

Its odd-degree model (`t=1/x`, `y_new=t^4*y`) and exact Mumford
coordinates are recorded in `map4162_receipt.json`. The intermediate
elliptic curve is
`E': Y'^2+X'Y'=X'^3+a^3*X'^2+lambda*X'`.
The degree-two cover `C_K -> E'` is
`X'=alpha*V`, `Y'=X'*(y/(a^2*x^4+a*x))`.
The inseparable degree-two isogeny `E' -> E_b` is
`X=X'^2`,
`Y=X'Y'+X'^3+lambda*X'+c*X'^2+lambda^2`.
The exact rational identity is checked in Sage. On finite-field points,
the latter isogeny is bijective. Pulling back `P'-2P'` through the
separable cover and adding its three Frobenius conjugates gives a
base-field Jacobian class. For the source `G`, its image is nonzero,
has order `2081`, is fixed by `Frob16`, and the image of `Q` equals
seven times that image. The checked image uses Mumford coefficient masks
`u=[3158,373,1,1]`, `v=[3158,2623,373]`. The factor-base solve
used 16 finite curve points and 16 relations in 114 walk steps, yielding
logarithm `7`. Its online map-plus-solve time was `0.122 s`; matched
rho averaged `0.000869 s` over 1,000 trials, a 140-fold slowdown. The
all-15 table uses a separate deterministic run with complete construction
time included, which accounts for its larger `b=1384` ratio.

## Exact fields and curves

Let `k = F_2[a]/(a^4+a+1)` and
`K = F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1)`. The Sage embedding in the
receipt sends `a` to the polynomial-basis mask `1820` in `K`. Set
`h = a^2*x^4+a*x`. The three cataloged hyperelliptic curves have equation
`y^2+h(x)*y=f(x)`:

| Source order | LMFDB class | `f(x)` | `#C(k), #C(k^2), #C(k^3)` |
|---:|---|---|---|
| 3974 | [3.16.a_a_aet](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aet) | `x^8+x^3+a*x^2+(a^2+a+1)*x+a^3+a+1` | `17, 257, 3728` |
| 3978 | [3.16.a_a_aep](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aep) | `x^8+a*x^3+(a^3+1)*x^2+(a^3+a)*x+a^3` | `17, 257, 3740` |
| 4178 | [3.16.a_a_dd](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_dd) | `x^8+a*x^3+a*x^2+(a^2+a+1)*x+1` | `17, 257, 4340` |

The point counts were recomputed with Sage `HyperellipticCurve(...).count_points(3)`.
Their Weil polynomials are respectively `T^6-123*T^3+4096`,
`T^6-119*T^3+4096`, and `T^6+81*T^3+4096`. Evaluating at `T=1`
gives the source orders shown. This **isogeny-class match** supplies no
explicit map from a source point to `Jac(C)(k)`.

The 12 order-3974 source curves are
`E_b: Y^2+X*Y=X^3+w^5*X^2+b`, with polynomial-basis masks
`b = 306, 691, 2082, 2268, 2628, 2896, 2978, 2996, 3347, 3412,
3505, 3978`. The prime subgroup has order `1987`. These equations and
their GHS genus-four subgroup maps are separately recorded in the
[frontier report](../ghs_q16_genus4_frontier_20261028/REPORT.md).

## Branch-supported degree-four obstruction

Write `z=y/h(x)`, so the genus-three curve has Artin–Schreier equation
`z^2+z=f/h^2`. If `beta^2=b`, the elliptic curve is Artin–Schreier
equivalent to `z_E^2+z_E=X+w^5+beta/X`. For a direct cover respecting
the hyperelliptic involutions, consider rational functions

`X=lambda*product_i (x-r_i)^(e_i)`,

where `r_i` are the four distinct roots of `h`, all `e_i` are nonzero,
`sum(e_i)=0`, the positive exponents sum to four, and every `|e_i|`
is a power of two. These explicit restrictions give exactly 30 vectors:
12 permutations of `(4,-2,-1,-1)`, 12 of `(-4,2,1,1)`, and six of
`(2,2,-2,-2)`.

**Local lemma.** Every vector in the first two groups produces an
unremovable order-three pole in `X+beta/X`, for any nonzero `lambda,beta`
and any four distinct branch points in characteristic two. At the branch
point where either `X` or `1/X` has an order-four pole, its leading Laurent
term is `c*t^-4`. The coefficient of `t^-3` is `c` times
`1/(r-r_j)+1/(r-r_k)`, where `r_j` and `r_k` are the two points with odd
exponents. This sum equals `(r_j+r_k)/((r-r_j)(r-r_k))`, which is nonzero
because the branch points are distinct. The other summand `X` or `beta/X`
is regular there. An Artin–Schreier coboundary `H^2+H` cannot remove that
order-three term without introducing a higher-order pole. The source curve
has only conductor-one poles after reduction, so these 24 patterns cannot
give the proposed cover.

The six remaining vectors make `X` a square rational function. They pass
the order-three pole gate, so `search.py` compares the **reduced simple-pole
residue at each of the four branch points** with the first cataloged
order-3974 genus-three model for every `lambda` and every listed
order-3974 `b`. The necessary residues never agree. Across all 30 patterns,
`12*30*(4096-1) = 1,474,200` parameter choices were checked. The receipt
records zero matches and the exact field, curves, roots, residues, and
input hashes. This is a finite necessary-condition certificate for the
specified form of `X`; it is not an exhaustive search over all covers.

For the six square patterns, `descent.py` then solves the Frobenius
equivariance equations for the reduced residues without fixing a cataloged
target curve. Each of the six `48 x 24` binary matrices has rank `20`
and nullity `4`. Its 15 nonzero solutions yield the 15 `b` parameters
in the table above, with empty intersection against the earlier 66-source
frontier. This is an exact finite-field bound for the specified branch set,
degree, and square-pattern form. `audit.py` independently replays the
binary ranks and kernel spans from the receipt using integer XOR arithmetic.
It is not a bound on every cover attack.

## Automorphism quotient check

Every automorphism of a genus-three hyperelliptic curve descends to a
Möbius transformation preserving its branch set. The script enumerates
all `4!` branch permutations, solves for the Möbius coefficients, verifies
the fourth image, and finds exactly 12 maps, all affine. For each of the
three exact curves it tests whether the pullback Artin–Schreier function
differs by `H^2+H` plus an absolute-trace-zero constant. Only the identity
passes. In particular, none of these models has a non-hyperelliptic
involution over `K`, so none has a separable degree-two elliptic quotient
over `K`.

## Prior work and novelty boundary

The six genus-three isogeny classes used in this report and the first
three representative equations are already in
[LMFDB](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/).
The [original GHS/Weil-descent work](https://www.kantmath.de/publications/papers/weil_descent.pdf)
describes the cover-to-Jacobian transfer framework. Earlier
[degree-four genus-three elliptic-subcover work](https://www.researchgate.net/publication/290858033_Some_Elliptic_Subcovers_of_Genus_3_Hyperelliptic_Curves)
explicitly treats characteristic other than two, and
[characteristic-two genus-three curves with many involutions](https://arxiv.org/abs/0905.0546)
already give elliptic quotients in a different configuration. A
[2022 even-characteristic cover classification under the isogeny condition](https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf)
explicitly covers the setting `genus(C)=3*genus(E)`; our 15 exact
examples satisfy this condition by their Weil polynomials. The accessible
abstract does not identify which classification row contains the equations
here, so a new family claim is not justified. The local order-three-pole
criterion may be useful as a sharper exclusion for this characteristic-two
cover form, but its publication novelty has not been established.
Relative to the 66-source frontier, the 15 `b` equations are newly
identified **exact eligible equations**. Their isogeny classes and the
general cover-attack setting are known; their measured full costs are
above rho. There is no newly demonstrated vulnerable family or new attack
method in this result.

## Replay and proof scope

```sh
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/search.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/automorphisms.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/descent.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/map15.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/attack15.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/map4162.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/attack4162.py
python3 research/ghs_q16_genus3_subcovers_20261028/prior_work.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/family_formula.py
python3 research/ghs_q16_genus3_subcovers_20261028/audit.py
lean research/ghs_q16_genus3_subcovers_20261028/PatternAccounting.lean
lean research/ghs_q16_genus3_subcovers_20261028/BranchPole.lean
```

`PatternAccounting.lean` checks the exact search cardinalities and subgroup
order arithmetic. `BranchPole.lean` proves the characteristic-two additive
step that two distinct reciprocals cannot sum to zero, given injectivity of
inversion, and injectivity of iterated squaring. Neither file formalizes the Laurent-series expansion, the Sage
finite-field computation, the curve point counts, or the geometric subgroup
maps. Those have executable Sage checks and exact coordinate receipts.
The next route toward a genuinely new vulnerability is to test
larger fields or covers outside the published isogeny-condition classes,
with the same nonzero-map and end-to-end rho comparison gates. The
q16 measurements here do not cross that performance gate.
