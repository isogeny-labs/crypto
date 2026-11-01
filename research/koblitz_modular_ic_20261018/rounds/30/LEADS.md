# R30 directions and exact claim boundaries

These are study-local directions, with novelty unverified and no official
hypothesis promotion. The available atlas entry point in the sibling checkout
is absent, so a complete program-wide novelty check is outstanding.

## 1. Uniform integral recovery at larger cyclic degree

R22's two-map coefficient pattern extends as a formal family
m=(2+w)+7r+(6+2w)r^3,7I over Z[w],w^2+w+2=0.
The first native R30 call checks all frozen odd n through1021 (augmentation
rank1020), with an integral identity on every augmentation basis vector.
Even n have a genuine mod7 evaluation obstruction for this same formal family.
The proof source seeks a universal identity, not extrapolation from the list.

Mechanism: modulo7, w-3 is square-zero and m reduces to
5(1+r^3)+(w-3)(1+2r^3). An alternating cyclic sum inverts 1+r^3 for odd n;
the nilpotent correction lifts this to integral recovery with the7I map.
This is the elementary unit-lifting mechanism of Stacks10.32.4, not a new
unit-lifting theorem. Larger geometric maps with these coefficients are not
constructed. R21's denominator5 obstruction for its different regular-cycle
maps is unaffected.

## 2. Change the exponent to address the even-cycle obstruction

Prediction, NOT EXECUTED: replace r^3 by r^s, choosing s so that
n/gcd(n,s) is odd (for example s=3*2^v2(n)). Then r^s has odd order, the
same alternating-sum argument should yield integral recovery for the changed
formal map family even when n is even. Minimal test: preregister all previous
even controls with the changed exponent, retain the unchanged-exponent
obstruction as negative control, and certify identities on the full integral
augmentation bases. Failure means a derivation/implementation error in the
formal statement, not evidence about geometric maps. Geometry must provide
correspondences with the new exponent before this is a transfer construction.

## 3. Scalar monodromy in kernel algebras, before full extension coordinates

Actual degree12 calculation: reconstruct compatible middle X0(49) points,
identify both cyclic11 kernels by quotient invariants, and check both complete
49-level transported triples. Frobenius in each degree-five x-kernel algebra
is compared with five iterates of doubling. This gives the generator action
in F11*/{+/-1} without adjoining every torsion point or y coordinate.

Positive controls include actual division-factor products, kernel stability,
commuting7/11 squares, complete transported triples and corrupted Frobenius
rejection. The degree436 input is retained as the larger native test under
the same4GiB/1200s bounds. It is not a change to n in the CM transfer family.

ENUM30 subsequently exhausts every common-neighbor/parameter root for the
retained12/436 pairs. Remaining acceptance checks: relation to the original
ordinary moduli/collector source;
all signed multiplicities and exceptional fibers. A nontrivial scalar orbit
can annihilate a complete deck-norm fiber in the augmentation module, but this
must be bound to the actual collector divisor before claiming a zero column.

## 4. Canonical differentials before the capped map elimination

Proposal, NOT EXECUTED: the R27 proper-cover bridge gives
K_C=Diff(C/E), since E has genus1 with nowhere-zero differential du.
The displayed tame different is6 at each of10 ramified rational cusp points
and6 at the degree10 ramified Z7 point, totaldegree120=2*61-2.
Thus a complete L(Diff)*du gives61 holomorphic differentials. Build its Cartier
operator with source/action/descent matrices, then use those linear and
semilinear constraints to reduce the F32 map system before elimination.

This is not a statement that a Cartier eigenvector integrates to an elliptic
map. Differential constraints only screen candidates; actual function-field
identities, F2 descent and the intended subgroup/kernel binding remain the
acceptance test. Verify the canonical-divisor bridge and complete RR dimension
before using any computed matrix. Keep the prior455equation solve/4GiB failure.

References: R22 RECOVERY.md; R27 CM.md (different/chart coverage); R29
ACCEPTANCE.md; ordinary13_lift.rs; Stacks0AMF; Sutherland ClassicalModPolys;
Kilford–McMurdy arXiv1110.6801, section2. No IC cost or relation-rank verdict.


## Followup: preserve F32 component classes rather than the F2 orbit sum

DESCENT30 checks all six actual kernel colors in a coupled Frobenius/deck
permutation model. Over F32 the F2 orbit splits into five components; deck
acts regularly on them. An individual component difference is nonzero as a
formal integer vector, although the full orbit difference is0. Thus the
original reverse evaluator must distinguish an individual colored component
from the full F2 orbit sum. Neither vector statement decides the Picard class.
This is a concrete next calculation with actual component divisors, not a
claim of a nonzero transfer or new relation rank.
