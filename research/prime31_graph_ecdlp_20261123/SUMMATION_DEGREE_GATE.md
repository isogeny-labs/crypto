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
