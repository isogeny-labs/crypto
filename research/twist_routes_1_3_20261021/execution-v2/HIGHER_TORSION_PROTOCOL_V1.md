# E20 actual higher-prime torsion, version 1

Freeze date: 2026-10-23. Commit the native constructor-input generator,
independent generic-field checker, supporting checks and this protocol before
execution. Preserve every degree exclusion, exhausted search, invalid basis,
pairing disagreement, parser error, timeout and partial output.

From the original ten ordinary toy sources, inspect ell=5,7,11,13,17. Retain
only necessary-module positives with characteristic and source subgroup
prime to ell. Choose at most the first two source ids in lexical order per
prime, as specified in the proposal. Record a zero-positive prime explicitly.
Use the exact order of the companion Frobenius matrix as the extension
degree; retain degrees above 12 as unsupported, rather than replacing the
selected source. Public NIST curves are excluded from this actual larger-prime
construction pilot. A field presentation is a certificate coordinate system,
not a newly registered extension-curve slug or a cross-method DLP workload.

PARI/GP constructs the declared field with ffinit and a primitive polynomial
generator, computes its group order from the exact source trace recurrence,
and projects random point trials to the ell-primary part. Strip ell powers
until a nonzero order-ell point remains; find a second basis point with
nontrivial Weil pairing. Cap this at 128 public trials, seeded by
202610230020+p*ell. Export the full monic modulus, actual point coordinates,
quadratic-twist root, pairing, cofactor, ell-valuation and extension order.
An exhausted/imbalanced primary search remains unresolved. Use one thread,
4-GiB address space and 1,800 seconds per constructor.

Independently replay each output in native Rust's generic polynomial-basis
field of degree at most 12. Prove irreducibility with the full Rabin test,
verify every point/coordinate, exact basis orders, all ell^2 distinct torsion
points, the actual Frobenius matrix and its characteristic polynomial, and
the twist coordinate isomorphism and opposite Frobenius action. Recompute
the extension order/cofactor independently with arbitrary-size integers.
Compute the Weil pairing with a separate native Miller implementation and
require exact equality with PARI's orientation and a primitive ell-th root.

Enumerate every determinant-minus-one intertwiner in the actual bases.
Check each map's pairing direction with the independent Miller evaluator and
every point in its ell^2-point graph under actual coordinate Frobenius.
Retain concrete graph coordinates. Reject a dependent basis, the identity's
wrong pairing direction and the wrong twist Frobenius sign. Give the checker
the same one-thread/4-GiB/1,800-second per-case envelope. Use the native CAS
error/hash gate on the constructor session as well; a final marker is
insufficient.

These are actual torsion/pairing/Galois certificates beyond the earlier
matrix-only filter. Higher-degree polarized quotients, genus-two existence,
explicit maps and subgroup recovery at those gluing primes remain separate
obligations. No speed, novelty, asymptotic, public NIST DLP or full E20
completion claim is admitted from this bounded pilot.
