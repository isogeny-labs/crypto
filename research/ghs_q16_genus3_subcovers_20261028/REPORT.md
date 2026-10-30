# q16 genus-three subcover and nonzero-trace degree-three probe

## Result and scope

The new degree-three screen found **two Frobenius orbits of exact
nonzero-trace source equations** in class `3.16.a_a_ah`. Representatives
have explicit degree-three covers, nonzero subgroup maps, and complete
relation solves. Their full transfer is 1,236–1,430 times slower than
matched rho in this q16 run. These are potentially new **exact cover
instances** outside the published trace-zero genus-three locus, not a
demonstrated new vulnerable family or measured speedup. General cover and
decomposition methods were already published. Exact-equation publication
status remains unverified.

The earlier search found **15 exact q16 source equations outside the earlier
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

A later degree-three check excludes maps to ordinary elliptic curves from
all 60 cataloged models in those three classes. Separately, a
prime-to-subgroup-order isogeny to a genus-three Jacobian **exists
abstractly** for all 66 older source equations. The isogeny has not been
evaluated on source points, so these results remain bounds and feasibility
certificates rather than an additional measured attack.

## New branch-supported degree-three cover locus

Let `k=F16`, `K=F4096`, with the exact polynomial bases below. For an
ordinary genus-three curve `C: y^2+h(x)y=f(x)` with separable quartic
`h`, put `z=y/h`. Let the four roots of `h` in `K` be `r_i`, and choose
one of the 24 permutations of `(2,1,-2,-1)`. Set

`U(x)=product_i (x-r_i)^(e_i)`, `X=lambda*U`,
`beta=lambda*mu`, `b=beta^2`.

If there is `H in K(x)` and `u in K` with

`f/h^2 + X + beta/X = H^2+H+u`,                               (1)

then the rational formulas

`X=lambda*U(x)`, `Y=X*(y/h+H(x))+beta`

give an exact degree-three map `C_K -> E`, where
`E: Y^2+X*Y=X^3+u*X^2+b`. A four-branch ordinary genus-three model
with nonzero reduced residues cannot have a branch-supported degree-three
map of this form with any other zero/pole exponent multiset: the only
remaining possibilities contain an order-three pole, which cannot reduce
to the source's simple Artin–Schreier poles. For a fixed model and exponent
pattern, matching the four reduced residues is an `F2`-linear system in
`lambda,mu`. In this q16 census every such system has full column rank
24, so each model-pattern pair gives **at most one** parameter pair.
After matching, the script checks the complete identity (1); residue
matching alone is not used as a cover certificate. This is an exact
criterion and cardinality bound for the stated branch-supported ansatz,
not a global classification of all GHS covers or correspondences.

The frozen [LMFDB catalog slice](catalog_all_q16_weil_classes.jsonl)
contains 126 ordinary genus-three Weil-restriction-shaped classes and
10,664 listed curve models. The [full scan](degree3_all_q16_scan_receipt.json)
tested 255,936 model-pattern pairs and found 60 exact cover descriptions
across six classes, representing 30 distinct `(u,b)` equations. The
[per-hit prior-family gate](prior_work_degree3_receipt.json) records every
candidate. Twenty-four descriptions have trace-zero `b` and fall in the
published Joux–Vitse locus. Twelve have `b=1`, so the source coefficient
lies in the base field. The remaining 24 descriptions occur in classes
`3.16.a_a_ah` and `3.16.a_a_h`; they have nonzero relative trace and
form four Frobenius orbits of source equations. The `ah` class has group
order 4090 and a 409-order prime subgroup. The `h` class has group order
4104 and largest prime subgroup 19. These are exhaustive counts for this
catalog slice and cover ansatz only.

### Two 409-order nonzero-trace representatives

The table gives exact source curves using polynomial-basis masks for `u,b`
in `E: Y^2+X*Y=X^3+u*X^2+b`. Both cataloged base curves have equation
`y^2+h(x)y=f(x)` as displayed. In both rows the ordered branch exponents
are `(-1,-2,1,2)`, `lambda` and `mu` are the listed masks, and `H` is
given exactly by the rational numerator and denominator arrays in the
map receipts. Each Frobenius orbit contains three source equations;
Frobenius conjugation carries the displayed cover identity to the other
two equations.

| LMFDB model | `h(x)` | `f(x)` | `(u,b)` | `Tr_(K/k)(b)` | `(lambda,mu)` |
|---:|---|---|---:|---:|---:|
| `3.16.a_a_ah`, 5 | `a^2*x^4+a*x` | `x^8+(a+1)*x^3+(a^3+a^2+1)*x^2+x+a^3` | `(1571,1614)` | 3363 | `(2925,280)` |
| `3.16.a_a_ah`, 12 | `a^3*x^4+a*x` | `x^8+a*x^3+(a^3+a^2+a)*x^2+a^3*x+a^3+1` | `(3100,2952)` | 3362 | `(1706,3295)` |

The [model-5 map receipt](degree3_nonzero_trace_receipt.json) and
[model-12 map receipt](degree3_nonzero_trace_receipt_12.json) record the
field, branch roots, exact `X,H`, source generator, target `Q=7G`,
Mumford image, and Frobenius-fixed subgroup checks. The map is the
pullback of `P-2P` through the cover followed by the `K/k` Jacobian trace.
For model 5, `G=(3802,3009)`, `Q=(952,2216)`, and the nonzero image of
`G` has Mumford masks `u=[2622,1]`, `v=[1641]`. For model 12,
`G=(3047,101)`, `Q=(2425,3200)`, and the image has
`u=[3363,1641,3158,1]`, `v=[1641,1641,372]`. Both images have order
409, lie over `k`, and satisfy `image(Q)=7*image(G)`.

| Model | Complete transfer, s | Relations / walk steps | Matched rho mean, ms | Full/rho |
|---:|---:|---:|---:|---:|
| 5 | 0.709 | 12 / 100 | 0.496 | 1430 |
| 12 | 0.828 | 10 / 92 | 0.669 | 1236 |

Each [attack receipt](attack_degree3_receipt.json) and
[second attack receipt](attack_degree3_receipt_12.json) includes the
relation matrix and 1,000 same-target rho trials; both recover the target
logarithm `7` and verify it on the source and Jacobian. Complete transfer
time includes construction, map, factor-base setup, relation collection,
and linear solve. The times are local single-run measurements for the
transfer and means of 1,000 rho runs, so they are cost evidence for these
small instances, not an asymptotic claim.

Prior-work boundary: [Joux–Vitse Appendix A.2](https://eprint.iacr.org/2011/020.pdf)
describes the characteristic-two trace-zero genus-three cover locus.
[Magma's GHS documentation](https://www.math.uzh.ch/sepp/magma-2.25.2-ds/HandbookVolume10.pdf)
documents the general trace-conorm construction, and
[Iijima et al.](https://globals.ieice.org/en_transactions/fundamentals/10.1093/ietfec/e88-a.1.97/_p)
give a broader extension framework. The displayed nonzero-trace covers
are outside the **specific** Joux–Vitse trace-zero locus, but the
literature check does not establish a globally new cover family or an
unpublished exact equation. The new finding is an explicit, reproducible
nonzero-trace cover and subgroup-transfer instance in a previously
unsearched local branch-supported degree-three locus.

The checked Lean additions in `PatternAccounting.lean` verify finite
search accounting and the 409-order factorization. They do not formalize
the function-field identity, branch-divisor classification, Sage
finite-field enumeration, or benchmark. Those obligations are checked
by the runnable Sage scripts and the retained receipts.

## Fifteen exact eligible equations and measured costs

Every source has the common form
`E_b: Y^2+X*Y=X^3+w^5*X^2+b` over `K`; `b` is the listed polynomial-basis
mask. The exact base-curve equations, source and image point coordinates,
field embeddings, map checks, and individual receipts are in
[`map15_receipt.json`](map15_receipt.json) and
[`attack15_receipt.json`](attack15_receipt.json). The
[`prior-work receipt`](prior_work_receipt.json) gives each exact equation's
cataloged isogeny class, checks the published trace-zero cover criterion,
and marks its exact-equation publication status as unverified. The measured "full" cost
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
measurement for the transfer and a 1,000-run mean for rho; compiler/runtime
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
to transfer attacks. Every listed `b` satisfies
`Tr_(K/k)(b)=b+b^16+b^256=0`. The characteristic-two genus-three
hyperelliptic construction under precisely this condition appears in
[Joux–Vitse, Appendix A.2](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf).
The `V4` genus-three cover structure is also studied in
[Nart–Ritzenthaler](https://arxiv.org/abs/0905.0546). The closed formula
parameterizes 15 exact cases **inside a published family** and does not
establish a new family type or a speedup.

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
[Joux–Vitse characteristic-two genus-three work](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf)
already covers every one of the 15 new local `b` masks through its
relative-trace-zero condition. The exact-equation publication status remains
unverified, but the cover-family classification is settled as prior work.
Other
[degree-four genus-three elliptic-subcover work](https://www.researchgate.net/publication/290858033_Some_Elliptic_Subcovers_of_Genus_3_Hyperelliptic_Curves)
explicitly treats characteristic other than two, and
[characteristic-two genus-three curves with many involutions](https://arxiv.org/abs/0905.0546)
already give elliptic quotients in a different configuration. A
[2022 even-characteristic cover classification under the isogeny condition](https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf)
explicitly covers the setting `genus(C)=3*genus(E)`; our 15 exact
examples satisfy this condition by their Weil polynomials. The accessible
abstract does not identify which classification row contains the equations
here. The local order-three-pole
criterion may be useful as a sharper exclusion for this characteristic-two
cover form, but its publication novelty has not been established.
Relative to the 66-source frontier, the 15 `b` equations are newly
identified **exact eligible equations**. Their isogeny classes and the
general cover-attack setting are known; their measured full costs are
above rho. There is no newly demonstrated vulnerable family or new attack
method in this result.

## Non-trace-zero genus-three transfer gate

The earlier 66 exact q16 source equations have `Tr_(K/k)(b) != 0`, verified
per equation in [`isogeny_feasibility_receipt.json`](isogeny_feasibility_receipt.json).
They lie outside the Joux–Vitse trace-zero construction above. Their Weil
restrictions nevertheless share a Frobenius polynomial with a cataloged
genus-three Jacobian over `k`, as checked against the frozen
[LMFDB model and polynomial snapshot](catalog_models.json):

| Source order | Trace `t=4097-#E` | Prime `r` | Jacobian class | `disc(f) mod r` |
|---:|---:|---:|---|---:|
| 3974 | 123 | 1987 | [3.16.a_a_aet](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aet) | 587 |
| 3978 | 119 | 17 | [3.16.a_a_aep](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aep) | 8 |
| 4178 | -81 | 2089 | [3.16.a_a_dd](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_dd) | 1702 |

Here `f(T)=T^6-t*T^3+16^3`, and resultant arithmetic gives
`disc(f)=3^6*16^6*(t^2-4*16^3)^3`. Since `r` does not divide the
discriminant, `f mod r` is squarefree. The r-adic Tate modules of the Weil
restriction and the Jacobian are free rank-one modules over the same finite
etale algebra `Z_r[T]/(f)`; hence they are Frobenius-equivariantly
isomorphic. [Tate's homomorphism theorem](https://doi.org/10.1007/BF01404549)
then supplies an isogeny of degree prime to `r`, injective on the selected
prime subgroup. This is an **existence theorem**; the point map and its cost
are still missing. [`IsogenyGate.lean`](IsogenyGate.lean) checks the finite
count and three modular discriminant values. It does not formalize Tate's
theorem or the geometric isogeny.

The degree-three certificate (stored by checksum in
[`.artifacts.json`](.artifacts.json)) tests all 60 cataloged genus-three
models in these classes. In every model, `h` has four simple roots in `K`,
and the four reduced residues of `f/h^2` are nonzero. Suppose a degree-three
map from one of these curves `C/K` to an ordinary elliptic curve
`E: Y^2+XY=X^3+uX^2+b` exists. Choose a rational Weierstrass point of `C`
as the Jacobian base point and translate the elliptic target so its image
is the origin. The hyperelliptic involution on `C` then maps to negation on
`E`, so the pullback of the elliptic `X` coordinate is a rational function
`R(x)` of degree three. In Artin–Schreier coordinates the target has form
`z_E^2+z_E=R+u+sqrt(b)/R`. Since each of the four branch roots of `C` has
a nonzero reduced simple-pole residue, each must be a zero or pole of `R`.
The zero and pole divisors each have degree three. Their nonzero orders at
the four roots are therefore a permutation of `(3,-1,-1,-1)`, its negative,
or `(2,1,-2,-1)`; there is no degree left for zeros or poles elsewhere.
The order-three cases cannot have the same reduced conductor-one poles as
`C`. For each of the 24 remaining permutations, the script forms the
`48 x 24` binary residue-matching matrix for
`R=lambda*product(x-r_i)^e_i` with arbitrary independent `lambda,mu`, where
`mu=sqrt(b)/lambda`. Every matrix has rank 24 and its augmentation by the
required residues has rank 25. Thus the **1,440 matrix exclusions plus
the divisor argument exclude every degree-three map from these 60 exact
models to an ordinary elliptic curve over `K`**. The receipt retains every
matrix and RHS; `audit.py` independently replays the ranks and checks the
nonzero residues. This does not exclude a different genus-three model in
the same isogeny class, a higher-degree cover, or an isogeny correspondence.

## Replay and proof scope

The 1.25 MB degree-three matrix receipt is kept in the artifact store.
Run `python3 tools/artifacts/artifacts.py pull research/ghs_q16_genus3_subcovers_20261028`
before an audit-only replay, or regenerate it with the Sage command below.

```sh
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/search.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/automorphisms.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/descent.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/map15.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/attack15.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/map4162.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/attack4162.py
python3 research/ghs_q16_genus3_subcovers_20261028/prior_work.py
python3 research/ghs_q16_genus3_subcovers_20261028/isogeny_feasibility.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/degree3_branch_supported.py
TMPDIR=/Volumes/SSD990/llm/tmp /Volumes/SSD990/cryptanalysis/third_party/sage-binary/sage -python research/ghs_q16_genus3_subcovers_20261028/family_formula.py
python3 research/ghs_q16_genus3_subcovers_20261028/audit.py
lean research/ghs_q16_genus3_subcovers_20261028/PatternAccounting.lean
lean research/ghs_q16_genus3_subcovers_20261028/BranchPole.lean
lean research/ghs_q16_genus3_subcovers_20261028/IsogenyGate.lean
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
