# N16 action-certificate protocol (frozen before computation)

Base `7840c55b6c0`. Inputs are the 21 frozen odd-prime theta models and
their five recorded rational branch-point action matrices. No new curves,
target cohort, EKS job, or Diem generator.

For every model, use native exact arithmetic modulo its recorded prime to
check the following statements on the full four-dimensional coordinate
space, not merely on sampled points: every action matrix is nonzero and
invertible; each square is a nonzero scalar identity; every pair commutes
projectively; the product of all five is scalar identity; the fixed general
conversion matrix is invertible; and all theta inverse constants are
nonzero. Retain every scalar and any failure as a raw row.

Derive the generic Mumford projection's sign invariance and its inverse
candidate equations symbolically. Attempt a polynomial-identity certificate
that each recorded action sends the generic projection of a valid divisor
to the projection of its 2-torsion translation. The attempt must state the
Mumford ideal, denominators cleared, equality tested, and exact proof
boundary. If a full certificate cannot be constructed, do not infer it from
matrix relations or finite samples.

Check the exceptional degree-zero and five rational degree-one torsion
classes against native Jacobian addition and projected action, and sample
nonbranch degree-one classes by the fixed rule `x=0..min(p-1,255)` with both
available square roots of `f(x)`. Record missing projections, action
failures, and all tested class IDs. These are finite checks only. Provide an
independent validator for raw counts and digests, plus a neutral report,
graph, and PDF. Keep rho and end-to-end ratios null.
