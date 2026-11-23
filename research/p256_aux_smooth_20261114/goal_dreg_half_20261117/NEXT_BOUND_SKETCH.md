# Unexecuted full-bound sketch for the next goal turn

This note preserves a derivation to check, not a full-system result. Do not
promote it or spend another old qualification slot. Every algebraic step,
source hypothesis and required materialization/validation remains an obligation.

## One relation polynomial on the exact finite input set

The complete-law source is section3/equation1/Algorithm1 of
https://joostrenes.nl/publications/complete.pdf . Its only exceptional pairs
have difference of order2. Exact retained inputs and all their sums are
P256-rational; the group has odd prime order. A direct native implementation
must still verify projective nonzero outputs and exact replay/exception cases.

For a finite target R=(x_R,y_R), a line of slope s through R cuts the curve,
after writing u=x-x_R, in R and the roots of

    u^2 + (3*x_R-s^2)*u + (3*x_R^2+a-2*s*y_R).

A **nonsquare** discriminant certifies that R is its only Fp intersection.
For target infinity, a vertical line x=c with nonsquare curve RHS does the
same. Because the exact membership ideal has only rational geometric points,
applying that linear form to a complete projective sum is an exact relation
test on the entire input point algebra, even over the algebraic closure.
Find/check an actual line certificate before relying on this step.

Construct f from the published group-law straight-line program and canonical
membership reduction, never from enumerated successful relation rows. Charge
all polynomial expansion/normal-form work; a hidden N^m truth table is not an
acceptable substitute for the requested workload or a runtime improvement.

## Possible dense lower bound via an almost complete intersection

For the existing depth5 base,21vertical pairs give42points. Try to certify
its dense projective ideal as a CI of degrees(3,14): reduce the degree21
abscissa polynomial modulo x^3=y^2+3x-B. The resulting degree14 section has
all42zeros and is nonzero at infinity; construct/check the exact coefficient
identity, height and point-count certificate. This would give quotient
regularity15, consistent with the existing native certificate.

For m independent copies sharing H, the dense input algebra A would be
Gorenstein, free over K[H], with Hilbert numerator h_A and regularity r=15m.
Let f have degree e and vanish exactly on the accepted tuple subset C.
An identity to verify from graded canonical-module duality is

    Hilb(Ann_A(f),t) = t^r*h_C(t^-1)/(1-t),
    Hilb(A/(f),t) =
      [h_A(t)-t^(e+r)*(h_A(t^-1)-h_C(t^-1))]/(1-t).

Let delta be the first degree with h_A-h_C nonzero. If delta<e, the proposed
calculation yields ideal regularity r+e-delta. If delta=e and the degree-e
accepted-point ideal has dimension at least2 inside A_e, the same calculation
should imply ideal regularity>=r. Empty accepted sets and unit ideals need
their own cases. Locate/read the duality source and check the shifts/signs
and H-torsion exact sequence before accepting any bound.

Two independent target cross-product forms may supply that dimension2
condition: fix m-1 inputs and vary the last42points; their sums are42distinct
curve points, so cannot all lie on one line. Their canonical degrees must
be no larger than e; certify this rather than assume no leading cancellation.
None of this lower-bound argument is yet a native/Lean certificate.

## Cheap feature lift to test, without changing the factor base

The current depth5 structured membership footprint has maximum degree4.
Add each recorded degree4 standard monomial as a new coordinate, retaining
all original coordinates and points. Every old point function then has a
degree<=3 representative: the former top-degree monomials are now linear.
Thus the new membership quotient regularity should be<=3, with the exact
number of added coordinates fixed by the existing footprint. Compute/check
the lifted point GB and include all new variables, equations and setup work.

On m blocks, reduce the group polynomial **locally by the same membership
normal-form rule on both arms**. The lifted tensor footprint has top degree
<=3m, so the proposed finite-point bound in FINITE_POINT_ROUTE.md would give
complete ideal regularity<=6m. If the dense lower bound>=15m above is fully
certified under this *same* group representation, it would exceed the2x
target for all m2–7. This is a conditional derivation, not an achieved bound.

A canonical normal form can have up to42^m coefficient positions. A small
degree does not make its construction or storage cheap. Preserve the original
depth8 requirement; do not declare completion from this depth5 sketch, from
comparing upper bounds, or from unmaterialized/unchecked large-instance inputs.

Next concrete work: preregister one deterministic feature lift and line-law
qualification; certify its point algebra and dense CI; formalize/check the
full-bound lemmas and degree accounting; only then choose a bounded complete
matched cell. Old four CoCoA cells and the public allowance remain closed.
