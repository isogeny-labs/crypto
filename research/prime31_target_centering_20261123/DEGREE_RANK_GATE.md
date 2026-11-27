# Target-centered Prime31 degree reduction: exact, with rank and label costs

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

## Exhaustion of rational degree-two coordinates

The branch count covers every degree-two morphism `phi:E -> P1` defined
over `F_p` on these curves. The separable quadratic function-field
extension has a unique deck involution `iota` over `F_p`. Proposition 5
above shows over the algebraic closure that this involution is
`iota(P)=T-P` for a point `T`. Since `T=iota(O)`, it is rational. Each
rational group has odd prime order, so `T=2U` for a unique rational `U`.
The functions `phi(P)` and `x(P-U)` have the same invariant subfield;
therefore `phi(P)=h(x(P-U))` for some `h in PGL2(F_p)`.

Write `D=R-4U`. Given three generic `phi` values, the two point
preimages of each can be written `U +/- Zi`. The eight possible fourth
values are `h(x(D-e1*Z1-e2*Z2-e3*Z3))`, with signs `ei=+/-1`.
Two values coincide if the signed sums are equal or sum to `2D`.
Generically the first condition forces identical signs; the second
pairs opposite signs exactly when `2D=O`. On these rational odd-order
groups that means `D=O`. Hence the generic fourth-coordinate fiber has
eight values for `R!=4U` and four for `R=4U`. Varying the public query
while retaining the same translated factor base restores the eight-value
case. This is a geometric fiber-degree classification for rational
degree-two maps, not a claim about Groebner solving degree, restricted
finite selectors, or higher-degree morphisms. Exceptional and ramified
fibers can have fewer values.

## What a Mobius change can and cannot lower

For a fixed translation `U`, replace each centered abscissa `z_i` by
`h(z_i)` for `h in PGL2(F_p)`. Homogenize each coordinate on its own
`P1` factor and transform the selector equations along with the
summation equation. The resulting exact relation-and-selector ideal is
the image of the original ideal under an automorphism of `(P1)^4`.
That automorphism preserves the multigrading: the defining
multidegrees, multigraded Hilbert function and graded syzygy degrees
are unchanged. In particular, a Mobius rewrite alone cannot lower
those intrinsic degree measures while retaining the same selector.
Affine chart choice, monomial order, sparsity and F4 resource use can
still change, so a claim about **measured** solving degree or work needs
a completed equal-input comparison. A target-dependent translation is
a different operation: it changes the selector and has the costs above.

## Rank limit for an unlabelled fixed base

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

This kernel argument treats the `Fi` logs as unknown. It does not apply
to an independently **labelled** base whose point logs are already known:
one verified row then determines the target log. That published
single-relation route is the [Amadori-Pintore-Sala method](https://pure.tue.nl/ws/portalfiles/portal/91158650/609.pdf), already tested
on the repository's Prime31 curve in
`research/prime31_ic_20261115/native/APS_ONE_RELATION_PRIME31_RUST_20261116.md`.
If instead the chosen `Zi` have known logs and `Fi=U+Zi`, the `Fi` labels
contain the unknown `log(U)=r/4`; a centered four-point row merely says
`sum log(Zi)=0` and still cancels `r`. To claim a benefit with independently
labelled `Fi`, one must construct them with the claimed centered selector
and charge that target-dependent selection, extraction and verification.
The degree-four identity by itself supplies none of those labels.

For the specific strategy of drawing independent uniform known scalars
`k mod n` and retaining `kG` only when `x(kG-U)` lies in a fixed centered
selector of `B` liftable abscissae, the acceptance probability is exactly
`2B/n` when the translated set contains no identity point. Translation
is a bijection and each selected abscissa has two rational points.
Collecting `L` **distinct** labelled points by this rejection strategy
therefore takes an expected
`n * sum_{j=0}^{L-1} 1/(2B-j)` scalar draws. At `L=B` this is
`n*(H_(2B)-H_B)`, of order `n`, before relation extraction. This bound
applies to uniform rejection sampling into a fixed selector; it does not
rule out a different, proved way to construct labels and compact
membership together. It shows why the independently labelled escape
route needs an explicit construction-cost account.

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

## A natural rank breaker returns to ordinary target descent

Keep a fixed centered support of `B` abscissae `zi=x(Zi)` and let
`U=[4^(-1)]R`. A three-point relation among the corresponding shifted
points has the exact form

`(U+Z1)+(U+Z2)+(U+Z3)=R  <=>  Z1+Z2+Z3=U`.

This is the standard three-summand target query, represented by
`S4(z1,z2,z3,x(U))=0` and followed by rational point/sign replay. Its
coefficient sum is three, so it breaks the additive shift symmetry of
the four-point rows. A known-log anchor is still needed to set the
absolute discrete-log scale; with such an anchor, the centered
four-point rows are target-independent precomputation and the
three-point row is ordinary target descent.

For a fixed `B`-abscissa support independent of uniformly sampled `U`,
the exact expected number of distinct-index signed three-point tuples
equal to `U` is `8*C(B,3)/n`, by linearity of expectation. At `B=677`
and the B677 group order this is `0.1918030258` per uniform query. This
is a raw-witness expectation, not a completed extraction probability or
useful-rank estimate. The centered four-point zero-sum rows are fixed
when this `Z` support is fixed, so their number must not be inferred from
a uniform-target expectation. These counts show why the target-bearing
row cannot be treated as free when comparing the degree-four equation
with the original four-summand target presentation.

## Falsifier for a useful variant

Before treating target centering as an M31 relation-finding advance,
exhibit one fixed or exactly transferable factor base and completed rows
whose combined coefficient matrix determines the target scalar after
independent replay. Compare *completed* solving degree, useful rank,
failed calls, selector size, transfer, and cold single-target cost with the unshifted
presentation and the same-target generic control. A reduced polynomial
degree alone does not pass this gate. A fixed-field result also cannot
establish an asymptotic exponent without a family and cost proof.
