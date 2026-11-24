# Prime31 summation-polynomial degree and novelty gate

Date: 2026-11-23. Scope: the two ordinary elliptic curves over
`p = 2^31 - 1` already used in this repository. This is a mathematical
screen, not a measured relation-finding improvement.

## Geometric obstruction for the fixed abscissa map

Let `R` be a fixed point with `2R != O`. The exact four-summand locus in
abscissae is the image of the morphism

`(P1,P2,P3) -> (x(P1),x(P2),x(P3),x(R-P1-P2-P3))`.

Because `E^3` is irreducible, its image closure is irreducible. For three
generic input abscissae, each of the eight sign choices for `P1,P2,P3`
gives a distinct fourth abscissa. Indeed, equality of two output
abscissae implies either equality of the signed sums or that their sum is
`2R`. The former holds generically only for identical sign choices; for
opposite choices the latter would require `2R=O`, and all other choices
impose a nongeneric algebraic equation. Thus the exact hypersurface has
degree eight in the fourth abscissa. The specialized Semaev `S5` supplies
that hypersurface and is irreducible up to scalar under this condition.
This argument is an inference from the geometry, not a theorem explicitly
stated for this specialization in the cited papers.

This excludes a lower-degree polynomial in the **same four abscissae**
whose zero set over the algebraic closure is exactly that locus. It does
not exclude selector-restricted equations, auxiliary variables, rational
branch conditions, or a different morphism. A lower input degree or a
lower partial F4 degree is not by itself a lower completed solving degree.
Faugere et al. prove uniqueness of the summation polynomial for a fixed
morphism and classify degree-two maps as translated abscissa maps followed
by a Mobius transform (Propositions 2 and 5, respectively).

Source: Faugere, Huot, Joux, Renault and Vitse,
[Symmetrized Summation Polynomials](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf).
Kosters and Yeo's [Notes on summation polynomials](https://arxiv.org/abs/1503.08001)
is a further caution against identifying first-fall degree with completed
solving degree.

## Degree-three `y`-coordinate screen on B677

The ordinary `y:E -> P1` map has degree three because `y` has a pole of
order three at the identity. On the B677 curve, the fibers at `y=3,8,27`
each have three distinct rational points. For each of the two fixed public
targets `G` and `Q`, taking all 27 triples from these fibers and evaluating
`y(R-P1-P2-P3)` produces 27 distinct fourth coordinates. The independent
integer-group-law replay is `verify_y_fiber_degree.py`; it checks all nine
input points, both targets and hashes of the sorted outputs.

For either **fixed** target, a generic triple of `y` values has at most
`3^3=27` fourth values. The exhibited unramified specialization attains
27, so its generic fourth-coordinate fiber has degree exactly 27. This
rules out a lower intrinsic fourth-coordinate degree from simply replacing
ordinary `x` by ordinary `y` on these two targets. It does not compare
completed solving degrees or costs, and it does not rule out restricted
selectors, other maps, or special targets.

## Exact equations on a finite selector

Let `S` be a public set of `B` distinct **liftable** abscissae and let
`H(X)=product_(s in S)(X-s)`. For a fixed public target `R`, write
`F_R=S5(X1,X2,X3,X4,x(R))`. Because `H` is split and squarefree, the
selector coordinate ring is

`A = F_p[X1,X2,X3,X4]/(H(X1),H(X2),H(X3),H(X4))
   ~= product_(s in S^4) F_p`.

Suppose a proposed single equation `C_R` accepts **exactly** the same
ordered tuples in `S^4` as `F_R`. In `A`, there is a unit `u` with
`C_R=u*F_R`: at each tuple where `F_R` is nonzero take the ratio
`C_R/F_R`, and at each common zero choose any nonzero unit value.
Conversely, a unit multiple has the same zero set. Consequently
`(C_R,H(X1),...,H(X4))` and `(F_R,H(X1),...,H(X4))` are the **same ideal**
in the polynomial ring. They have the same quotient algebra and reduced
Groebner basis for a fixed monomial order. This is a finite-product-ring
deduction, not a new summation-polynomial construction.

The same ideal conclusion holds for any finite system
`C_1=...=C_t=0` in these **four original abscissa variables** with the
same common zero set on `S^4`. At each factor of `A`, its generated ideal
is zero if all `C_j` vanish and is the whole field otherwise. It therefore
equals the ideal generated there by `F_R`, factor by factor. This extends
the finite-selector obstruction to multi-equation presentations without
auxiliary variables; it does not say their generators cost the same to
process.

A lower-degree representative or multi-equation presentation could still
change the work of a particular F4 run; equality of ideals does not make
generator choice free. The candidate must be built from public selector and target
data without first enumerating its accepted tuples, preserve the same
distinct-index and rational-point semantics, and finish an equal-input
cost comparison. The statement applies to the **actual** liftable base;
the B677 algebraic selector's larger off-curve superset needs its own
lifting and rejection account. Auxiliary-variable systems and
non-squarefree selector schemes are outside this ideal-equivalence claim.

### B677 split-selector support before target equations

The frozen B677 split circuit uses 1,024 Dickson abscissae, of which 509
lift to rational curve points and 515 do not, plus 168 liftable Y7
abscissae. Before imposing the public target equations, its 2+2
factor split has `C(1024,2)*C(168,2) = 7,347,529,728` unordered
root choices. Exactly `C(509,2)*C(168,2) = 1,813,624,008` of those
choices use four liftable leaves, or 24.683% of the raw choices.
The remaining 75.317% contain at least one off-curve Dickson root.

This is a **support count**, not an F4 branch count or a measured solver
slowdown. The four-leaf anti-Frobenius argument in
`research/prime31_four_summand_20261122/scaling-b677/MATHEMATICS.md`
excludes such roots from genuine relations when the target is nonidentity
and outside the selector and all four abscissae are distinct. Its
repository note remains unsigned; the saved split-circuit executions are
resource-limited before root extraction. The count identifies algebraic
work that the compact selector may have to eliminate, without asserting
that any off-curve choice survives the target equations.

### Meaning of the local split-circuit resource limits

Seven source-bound 2+2 receipts (positive and public-query cases) were
committed in `5096df8283bd1a6991d1119ab23b58f2d535d98c`; all report
`ResourceLimit` before root extraction. The independent receipt verifier
now checks all 14 manifest hashes against that exact commit and reports
current-head drift. At the 2026-11-24 mainline merge, this comprised the
verifier amendment itself, an equivalent remainder-test rewrite in
`f4_fp.rs`, and formatting/dead-code annotations in `ic_boundary.rs`.
The subsequent reviewed source-pin update adds `f4-source/build.rs` to
the current-drift list. That build also pins the changed `prime_fast.rs`,
which is outside the receipt's 14-file manifest; all 33 isolated library
tests pass on this post-merge closure.
The saved timings belong to the recorded source commit; they have not been
rerun at the merged PR head. The planted-assignment unit test passes one
known positive gate assignment, which is a construction check rather than
completed extraction.

In `src/cryptanalysis/f4_fp.rs`, `symbolic_step` calls the resource guard
*before* adding a prospective symbolic row or monomial. Its
`matrix_entries` field is a weighted workspace estimate: it includes
`8*rows*cols + 4*rows + 4*cols` and extra trace-storage terms when
recording. It is not a count of allocated matrix entries. The reported
`max_rows` and `max_cols` update after a matrix is built, so they can
remain small when a later prospective workspace is refused. For example,
the local positive cap-64-million receipt rejects a prospective estimate
of 64,009,887 while reporting completed maxima of 228 rows and 451
columns and partial solving degree three. It does **not** document a
64-million-entry allocated matrix, peak RSS, completed Gröbner basis,
or a root result. A separate batch-one arm reaches a symbolic-monomial
admission of 8,202 against 8,192; changing the matrix budget alone does
not explain that stop. All these observations remain censored stage data.

## Curve-specific symmetry screen

The direct factorization of `x^3 + ax + b` over `F_p` is irreducible on
both repository curves and on the explicit degree-three isogeny codomain
of B677:

| Curve | `a` | `b` | `j mod p` | Rational nonidentity 2-torsion |
| --- | ---: | ---: | ---: | --- |
| B677/RR5 | 847514846 | 2087444724 | 2145262698 | absent |
| B677 degree-three codomain | 323891735 | 1985248359 | 336047232 | absent |
| Registered graph curve | -3 | 21 | 1390704497 | absent |

All three discriminants are nonzero, and none of the listed `j` values is
zero or 1728. The rational 2-torsion symmetry used in the cited
symmetrized construction therefore does not apply directly to these
models. The explicit codomain is the curve in
`research/prime31_ic_20261115/native/PRIME31_3ISOGENY_CM_TRANSFER_20261118.md`.
For a transported non-2-torsion target, its four-summand locus in the
codomain's ordinary abscissae has the same degree-eight fourth-coordinate
fiber by the geometric argument above. Thus the isogeny alone does not
lower this intrinsic degree; it could still change the cost of a different
presentation or a selector-specific computation. This says nothing about
extension-field torsion or other maps.

Reproduction: with Python FLINT, factor
`nmod_poly([b,a,0,1], 2147483647)` for each row; compute
`j = 1728*4*a^3/(4*a^3+27*b^2) mod p`.

### Isogeny pullback is the codomain locus

Let `psi:E -> E'` be the recorded separable degree-three isogeny and set
`phi=x_E' o psi`, a degree-six morphism on `E`. For a fixed target `R`,
the four-coordinate sum locus for `phi` is **exactly** the ordinary
abscissa sum locus on `E'` with target `psi(R)`, over the algebraic closure.
One inclusion follows by applying `psi` to a source relation. Conversely,
choose source lifts of any four codomain points summing to `psi(R)`.
Their source sum differs from `R` by a kernel point `K`; subtracting `K`
from one lift preserves its `phi` coordinate and makes the source sum
equal `R`. By uniqueness of the irreducible polynomial for a fixed
morphism, the defining relation polynomial is the codomain specialized
`S5` up to scalar. Its generic fourth-coordinate degree is eight for
the non-2-torsion targets here, despite `deg(phi)=6`.

The isogeny restricts to a bijection between the two rational groups of
the same prime order, so the rational-point relation sets correspond as
well. This composition supplies no distinct lower-degree summation
polynomial. A changed codomain selector or polynomial presentation may
still change measured work and must carry its map and factor-base costs.

## Single split-graph secant capacity at B677 scale

The repository's
`research/prime_factor_base_degree_20261031/SECANT_GRAPH.md` proves a
presentation-specific homogeneous top-ideal degree `d+1` for a two-summand
secant system built from one degree-`d` ordinate graph `y=h(x)`, when
`h(x)^2-f(x)` is completely split and squarefree. That construction has
exactly `2d` distinct base abscissae; its same-base dense presentation
has top-ideal degree `2d+1`. These are **homogeneous presentation degrees**,
not completed F4 solving degrees.

To supply at least the frozen B677 base size from one such graph requires
`2d >= 677`, hence `d >= 339` and, if such a completely split graph
exists, secant top-ideal degree at least 340.
The minimum full split has 678 abscissae, so it is not the exact B677
selector without an additional restriction and a new degree proof. The
reported `d=2` reduction from five to three therefore does not transfer
to this 677-column workload as a low-degree single-graph system. This is
a degree-and-support screen, not a lower bound on iterative solver work
or on unions of graphs and Boolean-offset variants; those require their
own exact support, source-cost and rank comparison.

### Boolean-offset quadratic graphs: conditional capacity screen

The existing Boolean-offset construction in
`research/prime_factor_base_degree_20261031/pencil_search_20261101/`
uses `h_u(x)=a*x^2+b*x+c+t0+sum(w_i*u_i)` with `k` bits per
two-summand slot. Its Lean result proves exact relation semantics and
canonical homogeneous first-vanishing degree `2k+3`, assuming the two
quadratic leading coefficients are nonzero and the field has odd
characteristic. This theorem applies to presentations over M31, but its
reported fixture and same-base dense comparison are over F17. It does
not establish completed affine solving degree or relation cost over M31.

For each Boolean label, `h_u(x)^2-f(x)` is a nonzero quartic, hence has
at most four distinct abscissa roots over M31. A union of `2^k` labelled
quadratic graphs therefore has at most `4*2^k` distinct abscissae,
regardless of duplicate roots between labels. Reaching 677 abscissae
requires `k>=8`: seven bits permit at most 512, while eight permit at
most 1,024. At `k=8` the proved two-slot canonical degree is 19,
with 16 Boolean variables and three core variables. The bound is only
capacity; it supplies no 677-column M31 construction, rational-root
yield, or same-base comparison. The F17 eight-column fixture cannot be
scaled by replacing field coefficients in its proof.

There is also an exact average-yield identity on the B677 curve. Fix
any quadratic `h0` and let `N(t)` count rational roots of
`(h0(x)+t)^2=f(x)` for `t` in M31. Each affine curve point `(x,y)`
corresponds to exactly one counted pair `(t,x)`, namely
`t=y-h0(x)`. Thus `sum_t N(t)=#E(F_p)-1=2,147,446,830`, and the mean
over all `p=2,147,483,647` offsets is approximately `0.999982856`
roots. For a selected eight-bit family of 256 distinct offsets to
cover 677 distinct abscissae, its summed root count must be at least
677, or at least `677/256=2.64453125` roots per selected offset.
Because each `N(t)<=4`, at least 83 of those 256 offsets must have
three or four roots: otherwise their combined maximum is
`2*256+2*82=676`. Duplicate abscissae across offsets only raise the
required summed count. This is a necessary construction-yield gate;
it does not bound how many such high-root offsets exist or whether an
eight-weight subset-sum family can select them.

An independent SageMath 10.10.rc0 replay of
`EllipticCurve(GF(2147483647),[847514846,2087444724]).cardinality()`
returned `2147446831`, confirming the group-order input to this count.

The three-root case is exceptional. A squarefree quartic over a field
with three rational roots has a fourth rational root, so `N(t)=3`
requires a repeated root of `q_t(x)=(h0(x)+t)^2-f(x)`. At such a root,
`q_t=q_t'=0`; eliminating `t` gives
`4*f(x)*h0'(x)^2-f'(x)^2=0`. For nonzero quadratic leading coefficient
this is a degree-five polynomial in `x` over M31. Each of its roots
with `h0'(x)!=0` determines one `t`. On B677,
`f'(x)=3*x^2+847514846` has no M31 root: the Legendre symbol of
`-847514846/3 = 1149150816 (mod p)` is `-1`. A repeated root
therefore cannot occur at `h0'(x)=0`. Thus at most five offsets in the
entire field can have a repeated root, and at most five can have
`N(t)=3`. If an eight-bit selector had at most 79 four-root offsets,
even giving five others three roots and every remaining offset two
would yield only
`4*79+3*5+2*(256-79-5)=675` root labels. Covering 677 distinct
abscissae therefore requires **at least 80 completely split,
squarefree quartic fibers** among its selected offsets. This is
a necessary condition; repeated abscissae across offsets raise it.

### Global split-fiber count for every quadratic graph

The offset function `tau=y-h0(x)` is a degree-four map from the B677
elliptic curve to the projective line: its only pole is `4*O`, since
`h0` has nonzero quadratic leading coefficient. Its geometric
monodromy is `S4`. To see indecomposability, a hypothetical `2*2`
factorization has an intermediate curve of genus zero or one by
Riemann--Hurwitz. A degree-two map to a genus-one intermediate is
unramified and cannot pull back a pole to the single point `O`.
For a genus-zero intermediate, its relevant fiber must be `2*O`;
after a Mobius change of coordinate, its degree-two function lies in
`L(2*O)=span(1,x)`, so any composition lies in `F_p(x)`. But `tau`
contains `y`. Thus the cover is indecomposable and its geometric
monodromy is primitive. Inertia at the unique pole is a four-cycle;
among primitive subgroups of `S4`, only `S4` contains a four-cycle.
The arithmetic monodromy is consequently also `S4`.

Let `Y` be the smooth projective Galois closure. Riemann--Hurwitz for
`tau` gives total ramification eight, with three at infinity and five
at finite branch values; all ramification is tame since `p>4`. For
each finite branch permutation `sigma`, write `index(sigma)` for four
minus its number of cycles. The Galois-cover contribution
`24*(1-1/ord(sigma))` is at most
`12*index(sigma)`. The finite contributions total at most 60;
infinity contributes 18. Riemann--Hurwitz for the 24-sheeted Galois
cover gives `genus(Y)<=16`. There are at most five finite branch values.

As a concrete branch check, for `h0=3*x^2+5*x` over M31, SymPy computed
`disc_x((h0+t)^2-f(x))` of degree five in `t` with leading coefficient
`6912` and `gcd(discriminant, derivative_t)=1` modulo `p`. This sample
has five distinct finite branch values; the general bound does not
assume that every quadratic graph does.

Write `N4` for the number of **finite, squarefree, four-root** offsets
over M31. Each such offset contributes 24 rational points to `Y`;
other unramified offsets contribute zero. The at most six branch
values, including infinity, contribute between zero and 144 points
in total. The Hasse--Weil curve bound therefore gives

`abs(N4-(p+1)/24) <= (4/3)*sqrt(p)+6`,

or conservatively `89,416,692 <= N4 <= 89,540,279` for M31. This is
a global count of about `p/24` full-split offsets for **every**
nondegenerate quadratic `h0`; it is a deduction from monodromy and
point counting, not an enumeration of two billion offsets. The
function-field Chebotarev framework is also described by
[Andrade, Bary-Soroker and Rudnick, Appendix A](https://www.math.tau.ac.il/~rudnick/papers/andrade%20bary%20soroker%20rudnick.pdf).
The interval does not bound how many of those offsets can lie in one
256-element subset-sum family. The 80-full-split condition above
remains the local construction gate.

There is a rigorous baseline for **uniform random coefficients**, with
`h0` fixed. Choose `t0,w_1,...,w_8` independently and uniformly from
M31 and set `T_u=t0+sum(w_i*u_i)` for each of the 256 bit vectors `u`.
For distinct `u,v`, the coefficient rows `(1,u)` and `(1,v)` are
linearly independent over M31, so `T_u,T_v` are independent uniform
offsets. If `X` counts labels whose quartic has four distinct rational
roots and `delta=N4/p`, pairwise independence gives
`E[X]=256*delta` and `Var(X)=256*delta*(1-delta)`. The global interval
above yields `E[X]<=10.674034` and `Var(X)<=10.229589`. Cantelli's
one-sided variance inequality then bounds

`Pr[X>=80] <= 10.229589/(10.229589+(80-10.674034)^2) < 0.002124`.

Covering 677 distinct abscissae requires `X>=80`, even if some labels
collide. This bound applies to the specified random-coefficient draw.
Adaptive selection, deterministic designs, construction work and
completed relation-solving yield remain open.

At **ten bits**, support capacity becomes a provable existence result
when `t0,w_1,...,w_10` are independent uniform M31 elements. Let
`m=1024`. Let `S` be the sum of the root counts over its labelled
offsets, and `U` the number of distinct abscissae in their union. With
`mu=(#E(F_p)-1)/p`, pairwise independence and `0<=N(t)<=4` give
`E[S]=m*mu=1023.982444...` and
`Var(S)<=m*(4*mu-mu^2)=3071.964889...`.
Because the B677 group has odd order, each liftable abscissa has two
different ordinate values and therefore two offsets for a fixed `h0`.
For two independent uniform offset labels, the expected number of
shared abscissae is `2*(#E(F_p)-1)/p^2`. Counting all label pairs
bounds the expected duplicate loss by
`E[S-U]<=C(m,2)*2*(#E(F_p)-1)/p^2<0.000488`.

Cantelli's lower-tail bound and Markov's inequality now give

`Pr[U<677] <= Var(S)/(Var(S)+(E[S]-676)^2)+E[S-U] < 0.0252291`.

Thus a uniformly drawn ten-bit family has **more than 97.47%**
probability of at least 677 distinct rational abscissae, and such
families exist. The same two-slot top-degree formula gives degree 23
with 23 variables before projection or 21 variables after projecting
one bit per slot. Their top-quotient lengths are 6,291,456 and
3,670,016, respectively. The resulting base generally has more than
677 abscissae; selecting exactly B677 needs an additional exact
restriction with a new degree and cost account. This support theorem
does not supply four-summand extraction or a completed solver result.

The full affine monomial envelopes through the respective canonical
first-vanishing degrees expose the variable tax:

| Bits per slot | Canonical degree | All-bit variables / envelope | One-bit-projected variables / envelope |
| ---: | ---: | ---: | ---: |
| 8 | 19 | 19 / `C(38,19)=35,345,263,800` | 17 / `C(36,19)=8,597,496,600` |
| 10 | 23 | 23 / `C(46,23)=8,233,430,727,600` | 21 / `C(44,23)=2,012,616,400,080` |

These are derived counts of *possible* monomials of degree at most
the displayed value. They are not observed symbolic rows, F4 matrix
widths, completed affine solving degrees or runtime estimates. The
ten-bit support result therefore leaves the principal solving-cost
gate unresolved despite its ample rational support.

### Quadratic-offset fourth-coordinate degree on B677

For `h0(x)=3*x^2+5*x`, the quartics `(h0(x)+t)^2-f(x)` at
`t=20,33,85` each have four distinct rational roots. Their abscissae
are recorded in `verify_y_fiber_degree.py`, which checks all twelve
curve points and all `4^3=64` triples. For each fixed public target
`R=G,Q`, the 64 values of
`tau(R-P1-P2-P3)`, where `tau=y-h0(x)`, are distinct. An independent
SageMath elliptic-curve replay agreed with the verifier's sorted-value
SHA-256 digests for both targets.

Each generic input offset has four points in its fiber, so 64 is an
upper bound on the fourth-coordinate degree. The squarefree input
fibers and distinct fourth values at this specialization show that
the bound is attained over the function field of the three input
offsets for each of these fixed targets. Thus this quadratic graph's
four-summand locus has **fourth-coordinate degree 64** for `G` and
`Q`. The same witness makes the discriminant nonzero on a Zariski-open
set of quadratic coefficients and input offsets. This concerns the
intrinsic coordinate fiber, not a selector-restricted presentation,
completed solving degree or relation cost.

### A cubic-offset alternative and its support gate

The linear graph `y=a*x+b+t` gives a map
`tau_3=y-a*x-b:E -> P1` with one pole, `3*O`; its finite fibers are
roots of the cubic `(a*x+b+t)^2-f(x)`. On the B677 curve its geometric
monodromy is `S3`. A transitive cubic cover is cyclic or `S3`; the
cyclic case would give a deck automorphism of order three fixing `O`,
which requires `j(E)=0`. The B677 Weierstrass coefficient
`a_E=847514846` is nonzero, so `j(E)` is nonzero. The arithmetic
monodromy is also `S3`.

The map has total ramification six. Infinity contributes two, leaving
index sum four at finite branch values. Its six-sheeted Galois closure
has genus at most three: infinity contributes four to
Riemann--Hurwitz, and each finite contribution is at most three times
its index. If `N3` counts finite squarefree offsets with three rational
roots, Hasse--Weil and at most five branch values give

`abs(N3-(p+1)/6) <= sqrt(p)+5`.

As a concrete check, SymPy found that the cubic-fiber discriminant for
`a=3,b=5` has degree four in `t` and is squarefree modulo M31.
For this same fixed graph, the fibers at `t=3,5,28` each have three
distinct B677 rational points. All 27 triples produce 27 distinct
fourth values of `tau_3(R-P1-P2-P3)` for each public target `R=G,Q`.
`verify_y_fiber_degree.py` replays every point equation and group sum,
then checks hashes of the sorted fourth-value sets. The generic
fourth-coordinate fiber degree for this graph and either fixed target
is therefore **27**, by the 27-value upper bound and one unramified
specialization attaining it. Moreover, over the parameter space of
linear slopes and three input offsets, the three input fibers form a
generically finite 27-sheeted cover. On the locus with finite fourth
points, the discriminant of their 27 coordinate values vanishes where
two values collide. The displayed
specialization has no equalities, so for each of the two fixed targets
the fourth-coordinate degree is also 27 for a Zariski-open set of
linear slopes and input offsets. The constant intercept merely shifts
the offset parameter. This is a coordinate-degree statement; special
slopes, restricted selectors, completed solving degree and relation
cost remain open.

The global three-root density is therefore approximately `1/6` for
every fixed linear graph. A 256-label offset family covering 677
distinct abscissae needs at least **165 three-root fibers**: even
granting two roots to every other label gives only `512+X` roots when
`X` labels have three. With independent uniform `t0,w_1,...,w_8`,
the labelled offsets are pairwise independent. The upper density
bound and Cantelli give `Pr[X>=165]<0.002371`. This screens that
random family; it does not bound adaptive designs.

At ten bits, the point-count identity gives expected labelled root
total `1024*(#E(F_p)-1)/p`. Replacing the quartic bound `N(t)<=4`
above by `N(t)<=3` yields `Var(S)<=2047.983`; the same duplicate-root
charge and one-sided variance bound give `Pr[U>=677]>0.98288` for
uniform random coefficients. This proves support existence for a
larger linear-graph family. It does not identify the exact B677
selector, lower a completed summation-polynomial solving degree, or
price useful relations. A cubic coordinate has three generic point
preimages, so a lower membership degree alone is not a four-summand
relation-cost argument.

There is also a direct **canonical top-ideal obstruction** for using
these linear graphs in the existing two-summand secant presentation.
Put `h_u(x)=a*x+b+t0+sum(w_i*u_i)` in the five generators recorded in
`research/prime_factor_base_degree_20261031/pencil_search_20261101/reports/REPORT.md`,
with each sign branch fixed. Their highest homogeneous forms generate

`(m^2, x*z-r*m^2, m*x, m*z, u_i^2, v_i^2)`

and hence the same ideal as
`(m^2, x*z, m*x, m*z, u_i^2, v_i^2)`. Setting
`m=z=u_i=v_i=0` maps its quotient onto `F_p[x]`. Therefore `x^d`
survives for **every** `d`: this canonical highest-form ideal has no
finite first-vanishing homogeneous degree. The quadratic-graph law
`2k+3` relies on the nonzero `x^2` and `z^2` terms and cannot be
carried over by replacing the graph with a linear one. This is an
exact statement about the displayed generators' leading-form ideal;
elimination consequences, a different grading or an affine Groebner
run can behave differently and need their own completed cost test.

### Redundant curve equations give a finite linear-graph top ideal

The preceding obstruction concerns the original generator list. For a
finite target `R=(r,s)`, write `f(X)=X^3+A_E*X+B_E` and
`L(X)=m*(X-r)-s`. Let `g1=m^2-x-z-r` and
`g2=x*z-r^2-r*m^2-A_E-2*m*s` be its chord equations, and let
`q_x=sigma_x*h_u(x)-L(x)` and `q_z=sigma_z*h_v(z)-L(z)` be the two signed
ordinate residuals. The chord identity gives the **polynomial
identities**

`h_u(x)^2-f(x)=q_x*(sigma_x*h_u(x)+L(x))+(x-r)*(x*g1+g2)`,

`h_v(z)^2-f(z)=q_z*(sigma_z*h_v(z)+L(z))+(z-r)*(z*g1+g2)`.

Thus appending these two curve equations leaves the full affine ideal
unchanged, including its multiplicities and finite signed witnesses.
The displayed certificates have total degree at most four after their
leading terms cancel; they do not assert degree-three Macaulay
derivability. The original inverse-pair and infinity branches still
need the existing chart treatment.

For `k` Boolean bits in each linear-offset slot, the augmented
generators' canonical highest-form ideal is

`(m^2, x*z, m*x, m*z, x^3, z^3, b_1^2,...,b_(2k)^2)`.

Its core standard monomials are `1,m,x,z,x^2,z^2`, so its Hilbert
series is `(1+3t+2t^2)*(1+t)^(2k)`, its top-quotient length is
`6*2^(2k)`, and its first-vanishing degree is `2k+3`. At `k=8`
these are 393,216 and 19; at `k=10` they are 6,291,456 and 23.
The latter equal the all-bit quadratic-offset presentation's canonical
invariants despite the linear map's different support and
fourth-coordinate degree. The unaugmented linear presentation retains
the infinite canonical degree stated above.

`verify_linear_secant_top.py` checks both ideal-membership identities,
mutual highest-form ideal membership, a surviving top-degree monomial,
and the core Hilbert sequence over M31 for all four sign branches of
one fixed public B677 target. This independently replays the
[PR discussion's construction](https://github.com/isogeny-labs/crypto/pull/5097#issuecomment-7147311122).
Neither the augmented presentation nor its degree gives a completed
affine solving degree, an exact B677 linear selector, useful relation
rank or a cold same-target cost comparison.

### Fourth-moment capacity refinement for uniform offsets

The preceding pairwise-independence bounds are conservative. For either
map `tau=y-h0(x)` of degree `d=4` (quadratic graph) or `d=3` (linear
graph), let `N(t)` count rational points in its finite fiber and set
`mu=(#E(F_p)-1)/p` and `f(t)=N(t)-mu`. The `S_d` monodromy proved above
makes the ordered-distinct-pair fiber cover geometrically connected.
Its degree is `d*(d-1)`. Acting on ordered pairs, the branch-permutation
indices are `5,6,8,9` for degree-four cycle types `(2),(2,2),(3),(4)`,
and `3,4` for degree-three types `(2),(3)`. Tame Riemann--Hurwitz gives
genus at most six and three respectively. Hasse--Weil, with at most
five or four finite branch values charged separately, bounds
`v=E[f(t)^2]` by `V`, where

`B=p+1+2*g_max*ceil(sqrt(p))+b_max*d*(d-1)` and
`V=((#E-1+B)*p-(#E-1)^2)/p^2`.

For uniform independent coefficients `t0,w_1,...,w_k`, any three
distinct Boolean-label offsets are independent. Among four distinct
labels, the only dependencies are affine parallelograms; their exact
ordered count is `D=3*(6^k-2*4^k+2^k)`. The centered Fourier
coefficients of `N` satisfy `|fhat(s)|<=(d+1)/sqrt(p)` for `s!=0`:
the corresponding Artin--Schreier sheaf on `E` has one pole of order
`d<p`, compact-support cohomology dimension `d+1`, and the usual weight
bound. Parseval therefore bounds a dependent four-label expectation
by `(d+1)^2*V/p`. With `m=2^k`, the fourth centered moment of the
labelled root total `S` is at most

`M=m*(d-mu)^2*V+3*m*(m-1)*V^2+D*(d+1)^2*V/p`.

Markov's fourth-moment inequality and the earlier duplicate-root
charge then give these **derived upper/lower bounds for uniform draws**:

| Map degree | Eight-bit `Pr[U>=677]` upper | Nine-bit `Pr[U>=677]` upper | Ten-bit `Pr[U>=677]` lower |
| ---: | ---: | ---: | ---: |
| 4 | 0.000631064% | 0.106553% | 99.97149% |
| 3 | 0.000626826% | 0.106181% | 99.97154% |

For ten bits, the failure split uses `U<=676 => S<=688` or
`S-U>=13`. The calculation and pair-index enumeration are replayed
with exact rational arithmetic by `verify_offset_support_bounds.py`.
This independently checks and incorporates the derivation first
posted in the [PR discussion](https://github.com/isogeny-labs/crypto/pull/5097#issuecomment-7144814589).
These bounds do not constrain adaptive coefficient selection, determine
the exact B677 selector, or supply completed useful relations or solver
cost. They make a uniform eight- or nine-bit capacity search a weak
priority under this model; ten-bit support still carries its selector
and algebraic-variable costs.

Combining the quadratic-offset support with the target-centered degree-four `S4`
presentation has a separate rank gate. As proved in
`research/prime31_target_centering_20261123/DEGREE_RANK_GATE.md`, four-sum
rows on a fixed centered base have incidence matrix `M` with
`M*1=4*1`; the augmented target-log system retains the kernel vector
`(1,...,1,4)`. The 677-column support threshold, or a larger ten-bit
graph support, does not remove that gauge. Recomputing the centering
point from each public query changes the selector and requires an
exact transfer and cost account. A useful degree-four route needs a
charged rank breaker and completed same-target comparison.

Replay both probability bounds with exact rational arithmetic using
`python3 research/prime31_graph_ecdlp_20261123/verify_offset_support_bounds.py`.
Its exhaustive F17 two-bit check independently verifies the mean,
pairwise variance and duplicate-loss inequality on a finite example;
the M31 calculation uses the displayed point and split-count bounds.

This is a two-summand secant theorem. A four-summand B677 relation
would need a sound composition with target, sign, distinct-index and
root-extraction semantics. Before a lower-total-work claim, a concrete
public M31 selector must demonstrate its distinct liftable support and
charge the search for its coefficients, all labelled branches, completed
relation solving, failures and independent-row rank against the frozen
single-target rho control. For any proposed eight-bit selector, the next
falsifier is fewer than 677 distinct liftable abscissae, or a matched
completed solver run whose charged work is no lower than its same-base
control. Neither has been measured here.

### Projecting one bit per secant slot preserves the top degree

The earlier `PencilElimination.two_selectors_iff` theorem removes one
Boolean bit from each slot exactly: a residual `d+l*u=0` with `u` Boolean
becomes `d*(d+l)=0`. This reduces the variable count from `2k+3` to
`2k+1`. The projected equations have quartic residuals. Their canonical
highest-form ideal, with `n=2k-2` remaining Boolean bits, is

`I=(m^2, x*z, (x^2-alpha*m*x)^2, (z^2-beta*m*z)^2, b_1^2,...,b_n^2)`

for nonzero quadratic leading coefficients and either sign in each slot;
`alpha` and `beta` are their nonzero normalized reciprocals. This follows
by taking the leading forms of the *actual projected generators*:
the removed bit enters its original ordinate residual only linearly,
below its quadratic leading form. This derivation concerns the specified
generator presentation, not an arbitrary elimination ideal.

In the quotient by `I`, `m^2=x*z=0` and
`x^4=2*alpha*m*x^3`, `z^4=2*beta*m*z^3`. Hence every core monomial of
degree five vanishes. Boolean squares then make every homogeneous
monomial of degree `n+5` vanish. Conversely, `m*x^3*product(b_i)`
survives in degree `n+4`: after setting `z=0`, the two-variable relations
have Groebner basis `m^2, x^4-2*alpha*m*x^3` in lex order `x>m`, with
leading monomials `m^2, x^4`; the Boolean square relations leave the
squarefree bit product. The projected first-vanishing degree is
therefore **`n+5 = 2k+3`**, the same as the all-bit system, while using
two fewer variables.
This algebraic deduction has not been added to the Lean proof project.
An independent SymPy finite-field Groebner check over M31 found two
independent degree-`n+4` classes and none at `n+5` for `n=0,2`.
These sampled checks corroborate the general argument; they do not
establish its hypotheses for a concrete eight-bit selector.

The same top ideals give a more discriminating algebraic size measure.
The all-bit core has Hilbert series `1+3t+2t^2`; the projected core has
`1+3t+4t^2+4t^3+2t^4`. Independent Boolean squares multiply these by
`(1+t)^(2k)` and `(1+t)^(2k-2)`, respectively. At `k=8`, their total
top-quotient vector-space dimensions are `6*2^16=393,216` and
`14*2^14=229,376`. A SymPy Groebner standard-monomial enumeration over
M31 confirmed both core Hilbert sequences. These lengths concern the
homogeneous highest-form ideals; they are not F4 row counts, allocated
memory, solution counts or completed field-operation costs.
Replay with SymPy: `python3 research/prime31_graph_ecdlp_20261123/verify_pencil_projection_top.py`.
The script builds the actual two-bit-per-slot equations, checks the exact
Boolean projection identities, compares both highest-form ideals and
enumerates their standard monomials over M31. The `k=8` lengths follow
from the general independent-bit factor in the displayed Hilbert series.

For the conditional eight-bit M31 capacity case, projection leaves
14 bits and three core variables: 17 variables at canonical degree 19.
This is a presentation-level result. The quartic residuals, affine
solving degree, rational roots and charged relation work still need a
matched M31 execution before selecting this encoding over the all-bit
or dense systems.

### More than one projected Boolean bit

The one-bit calculation extends to the **literal product** that
eliminates `j_x` Boolean bits from the first quadratic secant slot and
`j_z` from the second, with `0<=j_x,j_z<=k`. Each projected product
vanishes exactly when at least one eliminated Boolean assignment makes
its original ordinate residual vanish. This preserves the existence of
a Boolean lift pointwise; repeated subset sums may give repeated factors
in the literal product. Deduplicating those factors would define a
different presentation and needs a separate top-degree calculation.

Put `n=2k-j_x-j_z`, `A=2^(j_x+1)` and `B=2^(j_z+1)`. For nonzero
quadratic leading coefficients, the canonical highest-form ideal of
this specified presentation is

`(m^2, x*z, x^A-2^(j_x)*alpha*m*x^(A-1),
  z^B-2^(j_z)*beta*m*z^(B-1), b_1^2,...,b_n^2)`,

where `alpha,beta` are the signed inverse leading coefficients. The
ordinate products have top forms `(x^2-alpha*m*x)^(2^(j_x))` and
`(z^2-beta*m*z)^(2^(j_z))`; reducing their binomial expansions modulo
`m^2` gives the displayed generators. In lex order `x>z>m`, the core
leading monomials are `m^2,x*z,x^A,z^B`; their S-pairs reduce to zero.
The core standard monomials are `m^e` times `1`, `x^i` for `1<=i<A`,
or `z^i` for `1<=i<B`, with `e` in `{0,1}`. The independent Boolean
squares therefore give Hilbert series

`(1+t)^(n+1)*(1+sum_(i=1)^(A-1)t^i+sum_(i=1)^(B-1)t^i)`,

top-quotient length `2*(A+B-1)*2^n`, and first-vanishing degree
`n+max(A,B)+1`. An independent SymPy M31 check of the core leading
monomials passed for `(j_x,j_z)=(0,0),(0,1),(1,1),(1,2),(2,2),
(2,3),(3,3)`; the formulas follow from the displayed algebra, not
from those finite checks alone. This generalizes the one-bit result
first proposed in the [PR discussion](https://github.com/isogeny-labs/crypto/pull/5097#issuecomment-7146126668).

| Bits per slot `k` | Projected per slot `j` | Variables | First-vanishing degree | Top-quotient length |
| ---: | ---: | ---: | ---: | ---: |
| 8 | 0 | 19 | 19 | 393,216 |
| 8 | 1 | 17 | 19 | 229,376 |
| 8 | 2 | 15 | 21 | 122,880 |
| 10 | 0 | 23 | 23 | 6,291,456 |
| 10 | 1 | 21 | 23 | 3,670,016 |
| 10 | 2 | 19 | 25 | 1,966,080 |

The formula also gives an exact minimum **within this literal-product
presentation family**. Write `m=max(j_x,j_z)` and `l=min(j_x,j_z)`.
Its first-vanishing degree is
`2k-m-l+2^(m+1)+1 >= 2k-2m+2^(m+1)+1`. For fixed `m`, equality
requires `l=m`. The symmetric expression is `2k+3` at `m=0,1`, and
its increment from `m` to `m+1` is `2^(m+1)-2`, positive thereafter.
Consequently only `(j_x,j_z)=(0,0)` and `(1,1)` attain the minimum
`2k+3` for `k>=1`. Projecting more bits cannot lower this canonical
first-vanishing degree; another generator presentation or a completed
affine computation is outside this conclusion.

At either capacity, two-bit projection reduces the homogeneous quotient
length by about 1.87 times relative to one-bit projection, while raising
the first-vanishing degree by two. These are presentation invariants, not
completed affine solving degrees, F4 sizes or relation-finding costs.
The eight-bit selector still lacks a 677-abscissa construction; the
ten-bit selector has a support-existence bound but lacks an exact B677
restriction and a completed same-target cost result.

## Existing presentations and the next falsifier

The balanced S4/S3 eight-leaf presentation and the S4-to-two-S3 auxiliary
chain are already documented in
`research/prime31_ic_20261115/native/BALANCED_S4_M8_INPUT.md` and
`research/prime31_ic_20261115/native/S4_S3_CHAIN_TARGET_DEPENDENT_GATE_20261116.md`.
The latter's exact Rust execution reached degree 10 for the original
single S4 and degree 6 for three chained pairings; all four were
oversize/undetermined. Its conditional eight-leaf arms reached degree 10
versus 5, again with no completed ideals. These are censored partial
degrees, not a completed solve-cost gain. Symmetric-coordinate roots also
have prior repository controls in
`research/prime31_ic_20261115/native/M8_SYMMETRIC_ROOT_RESULT_20261116.md`.
Auxiliary-variable decomposition has published precedent in characteristic
two: Karabina, [Point Decomposition Problem in Binary Elliptic Curves](https://arxiv.org/abs/1504.02347).

A qualifying new lead on this fixed M31 field must identify a genuinely
different exact locus or representation, prove full point and exceptional
branch semantics, then finish a same-input solver comparison. The measured
quantity is completed solving degree **and** total work through useful
independent relations, including failed calls, roots, sign lifting and
rank; the cold single-target cost is compared with the same-target rho
control. For fixed `p`, an asymptotic speed claim needs a separate family
and analysis rather than a fit on this one field.
