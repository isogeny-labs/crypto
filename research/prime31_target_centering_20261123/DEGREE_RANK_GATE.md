# Target-centered Prime31 degree reduction: exact but rank-inert by itself

Date: 2026-11-23. Status: symbolic derivation and novelty screen; no
completed solver comparison is claimed. This note concerns either of the
ordinary prime-order elliptic curves over `p=2^31-1` named in
`research/prime31_graph_ecdlp_20261123/SUMMATION_DEGREE_GATE.md`.

## An exact degree-four presentation

Let `R` be a fixed public group point and `U=[4^(-1) mod n]R`, so `4U=R`
in either odd-prime-order group. Translate each candidate summand as
`Pi=U+Zi`. Then, exactly as a point equation,

`P1+P2+P3+P4=R  <=>  Z1+Z2+Z3+Z4=O`.

Therefore the abscissae `zi=x(Zi)` satisfy the ordinary four-variable
Semaev `S4(z1,z2,z3,z4)=0`, of degree four in each variable. The apparent
factor-of-two reduction relative to fixed-target `S5` in unshifted
abscissae is valid because the coordinate map changed with `R`. It is
not a new summation polynomial: translated degree-two maps are covered by
Faugere et al., [Symmetrized Summation Polynomials](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf),
Proposition 5. Over the algebraic closure, the four-summand `S4` condition
still requires rational point lifting and exceptional-branch checks before
it can certify finite-field relations.

For a *fixed* rational translation `U`, write `D=R-4U`. The generic fourth
abscissa is `x(D-Z1-Z2-Z3)` for each of eight sign choices of the first
three `Zi`. Opposite sign choices yield the same fourth abscissa only if
`2D=O`. On these two rational prime-order groups, that means `D=O`.
Thus the degree-four branch pairing applies to `R=4U`; varying the public
query while retaining this translated factor base restores the generic
degree-eight condition. Other nongeneric inputs are outside this statement.

## Why fixed-target rows do not determine its scalar

Suppose a finite list of fixed translated points is `Fi=U+Zi`, and a
collection of four-summand rows is used as `sum_j F_ij=R`. Let `M` be the
row-by-point incidence matrix, `f` the unknown discrete logs of the `Fi`,
and `r` the unknown log of `R`. Every row has four entries, hence

`M f - 1 r = 0` and `M 1 = 4 1`.

Consequently `(1,...,1,4)` is in the kernel of `[M | -1]`: these rows
alone leave `r` undetermined. In the translated variables the same fact is
`M z=0`, with `f=z+(r/4)1`. This is a linear-algebra limitation of using
one fixed four-summand target, not a claim that coordinates or additional
relation families contain no information. A known-log anchor, variable
query coefficients, or other independent rows must break the gauge; their
construction and total cost must be included in any proposed gain.

If `U` is recomputed from each public query to keep `D=O`, the translated
point list also changes with that query. Any proposal to share a factor
base across those lists must give an exact transfer map and charge its
work. Simply counting the degree-four equation against the fixed-base
degree-eight equation omits that transfer and the rank defect above.

## Selector growth when the original base is retained

Suppose the original selector is a set of `B` liftable abscissae and
implicitly admits both signs. Let `A` be its `2B` affine curve points,
so `A=-A`, and assume `U` is not in `A`. In the centered coordinates the
exact allowed abscissae form `x(A-U)`. Two distinct original points `P,Q`
produce the same centered abscissa precisely when `P+Q=2U`. If `c` is the
number of such unordered pairs in `A`, the centered selector has exactly
`2B-c` distinct abscissae. In particular, without these special pairs its
degree as a square-free univariate selector is `2B`, not `B`.

On either prime-order M31 group, a proper nonempty `A` cannot be invariant
under `P -> 2U-P` when `U != O`: combined with `A=-A`, that would make `A`
invariant under translation by the nonzero point `2U`, and hence equal to
the whole prime-order group. Thus `c<B` for a proper base: even maximal
pairing cannot preserve the original `B`-value selector size. This is a
cardinality bound, not a prediction of Groebner cost. A selector that
chooses one signed point per old abscissa has a different count and must
retain the sign constraint explicitly.

## Falsifier for a useful variant

Before treating target centering as an M31 relation-finding advance,
exhibit one fixed or exactly transferable factor base and completed rows
whose combined coefficient matrix determines the target scalar after
independent replay. Compare *completed* solving degree, useful rank,
failed calls, selector size, transfer, and cold single-target cost with the unshifted
presentation and the same-target generic control. A reduced polynomial
degree alone does not pass this gate. A fixed-field result also cannot
establish an asymptotic exponent without a family and cost proof.
