# E14 explicit isogeny and representation execution, version 1

Use all ten prime-order fixtures. Test degrees 2,3,7. Degree two requires a
rational cubic root. For odd degrees, test every nonzero Frobenius eigenvalue
modulo ell. No eigenline rejects only that degree/family. If applicable, find
an exact order-ell point in the declared Fp6 field by seeded public sampling,
cofactor multiplication and Frobenius projection. Limit sampling to 128 trials.
The seed is 20261021+7919*p; preserve bounded failures.

Construct the full Vélu kernel and its rational functions. Require every
coefficient to descend to Fp; independently recount the destination group.
Evaluate every rational source point, check the destination equation,
injectivity, generator order, and the homomorphism with generator addition.
Retain a complete finite lookup inverse as a toy control, with its full table
size. This inverse is exponential setup and is not a scalable transport claim.

When the two distinct eigenlines are available over Fp6, construct the
normalized dual from the complementary torsion image. Determine and verify its
model isomorphism, check the degree multiplication on every rational point and
recover eight scalar controls using ell^-1 modulo r. If only one eigenline is
available, keep the algebraic-dual gap explicit; the finite lookup does not
discharge it. Check three additional isomorphic representations (u=1,2,3) and
their exact inverse coordinate maps.

Register all emitted model identities in this PR before report citation. These
are correctness and representation experiments, not an IC speed comparison.
No wall time, rho factor or known-scalar improvement is promoted. Use one L0
process, 4 GiB and 30 minutes; retain errors and fresh output directories.

```sh
research/twist_routes_1_3_20261021/runner/target/release/twist-route-experiments isogeny research/twist_routes_1_3_20261021/execution-v1/fixtures.json FRESH_ISOGENY_DIRECTORY
```
