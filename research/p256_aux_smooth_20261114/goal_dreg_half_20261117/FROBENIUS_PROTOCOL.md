# Higher-m bound certificate: finite weighted Frobenius check (preregistered)

The exact planted m2 unit succeeded. The next distinct obligation is to
make the dense filtered-duality hypothesis in FULL_BOUND_PROOF.md directly
checkable from finite point data, before applying it to all m=2--7. This
is not another relation search, dense BM retry or old CAS/lift rerun.

Use the frozen dense-membership.json from ANN-M2:288 points, degree-compatible
standard basis, quotient r97 and exactly one degree97 basis monomial.
Construct E, solve E^T*lambda=e_top, and verify every column equation.
Check every lambda entry is nonzero; emit all288weights and their product
with an inverse. This gives an explicitly nondegenerate weighted evaluation
pairing on K^288, not a Gorenstein label inferred from Hilbert symmetry.

Verify all unordered standard-basis products with degree sum<97 have zero
weighted evaluation. Commutativity covers the ordered products as well.
Check the Hilbert numerator is symmetric and every complementary filtered
dimension sums to288. Nondegeneracy + orthogonality + dimensions then give
V_i^perp=V_(96-i). Emit exact counts, hashes and corrupt-weight/dimension
controls. No extra dense/chain BM, full relation enumeration or new target.

Write the tensorization and degree-bound application explicitly. The
product functional has nonzero weights, annihilates degree<97m, and its
Hilbert numerator is the mth power of the certified one-point numerator.
Thus the same duality holds for every m, without materializing288^m points.
The two independent target-vanishing degree-e forms are justified by fixing
m-1 inputs and translating288 distinct final inputs; a line cannot contain
that many distinct points on the nonsingular cubic. Check all concrete
target/circuit hashes for the twelve frozen cells, and retain unknown
relation counts and complete higher-m solving status honestly.

The candidate r=5m span and upper-bound lemma retain finite H-torsion and
the actual raw homogeneous group equation. Report upper/lower bounds as
such, not exact values or affine F4/runtime degrees. Preserve original
ICV1/FB1 IDs, unique graph lifting, all signs and charged7m extra variables.

New local native exact correctness allowance: one<=180s/2GiB invocation,
and only after success one fresh-directory byte replay<=180s/2GiB.
Freeze source/input hashes and publish before execution; no retry after
failure/timeout. No CoCoA, publicCAS, EKS, paid run or measured solver work.
Keep the failed feature-lift/older solver caps closed. Maximum data are one
288x288 evaluation matrix and its transpose solve; no exponential lists.

Prior art checked: CBP filtered canonical-module definitions from the
existing primary PDF, classical BM/point interpolation, Chardin sections1--2
for standard CM regularity/local cohomology. New finite checker is a
certificate implementation, not a novelty claim. KB remains unavailable;
use the linked primary sources and frozen repo proof/data as fallback.
