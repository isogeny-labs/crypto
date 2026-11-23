# Formal proof boundaries

SignedDivisor37 checks two identities in an arbitrary commutative additive
group: the signed expression equals the five-pair expression, and a common
padding class cancels. These justify the expression cross-check and cancellation
of the common padding/base terms. They do not verify the concrete divisor data.

Torsion37 checks that a nonzero class killed by 11 has exact order 11, and that
a class killed by both 5 and 11 is zero. The first implication applies to the
extracted class 61E under the native scalar certificates and geometric bridge;
it does not apply to the original E, whose 11-multiple is nonzero.

ClassOrder37 checks the conditional classification from the actual scalar
certificates: order dividing 40931, with 11E, 61E and 3721E nonzero, leaves only
orders 671 or 40931. Separate implications handle the 671-multiple being zero
or nonzero. It also checks the scalar-to-eleven criterion at order 671 and
idempotence of the finite projector 122 on a class killed by 671.

The first torsion compile failed from overlapping instances. The first
source-order compile failed from kernel recursion depth and rewrite orientation.
Each full failed input/log remains beside its fresh corrected attempt. The
recursion setting does not add compiler trust; all accepted proofs still require
the pinned transitive axiom audit, with the same heap and wall limits.

Native arrays, the class-operation implementation, normalization, Riemann–Roch
and the intended CM-sign map are not formalized here. The exact workflow and
audit counts in results.json apply only to terminal accepted attempts.
