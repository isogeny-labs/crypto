# H116 pointwise rational reconstruction protocol

Protocol `H116-POINTWISE-RATIONAL-RECONSTRUCTION-001` is frozen before either
new implementation is written or executed.  It attempts to turn the already
checked point formula `[t-1]P-pi(P)` into an explicit rational x-coordinate
candidate on each of the same eight extension cells. [pcc:unverified]

## Predicted model

Let `N=#E(F_p)`.  The tested model predicts numerator degree `N`, denominator
degree `N-1`, and finite poles exactly at the x-values of nonidentity base
points, with the multiplicity of each pole equal to the size of its affine
x-fiber.  The resulting comparison of two degree-model candidates has
cross-product degree at most `2N-1`.

The preceding capacity census fixes every distinct x-sample.  After removing
the predicted poles, every frozen cell has strictly more than `2N-1` regular
samples.  The smallest margin is one; no cell may be replaced if that tight
case fails.

## Two reconstructions

Both implementations enumerate the pointwise images and build the monic pole
denominator.  They independently interpolate a degree-at-most-`N` numerator
from the first `N+1` and the last `N+1` regular x-values in canonical field
index order.  Acceptance requires the two full numerators to agree, all
coefficients to lie in the embedded base field, every regular point to match,
and no pole to cancel.  Recovered coefficients must also be identical across
the degree-two/three/four cells belonging to the same base curve.

## Controls and boundary

Adding one to the numerator constant must reject every regular evaluation.
Adding one to the denominator constant must destroy every observed pole.  A
sample restricted to exactly `2N-1` is recorded as a boundary failure because
the Lean theorem requires strict cardinality.

This is a finite-panel reconstruction and a test of the predicted degree/pole
model.  The degree formula itself is not yet proved in Lean, and finite-field
stability does not by itself identify a scheme morphism over the algebraic
closure or its completed local series.  No ECDLP runtime claim is made.
[pcc:unverified]
