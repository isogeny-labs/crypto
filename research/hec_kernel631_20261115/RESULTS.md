# Actual F631 Frobenius-primary 3-torsion kernels constructed

Source freeze e40de6a2b412, checkpoint20261116T012553Z, preceded one scientific execution. Exit0/stderr empty; all raw output and complete field/kernel serialization are retained. Input is the unique current-builder/arithmetic-compatible case from the separately frozen 96-case joint cohort: p631, f=[49,89,73,48,91,41,123,1], N237185924, ell59296481 and its recovered L. No old kernel, alpha or point table was loaded.

## Construction and exact checks

The candidate discovers the first irreducible degree-13 modulus t13+t+c, here c=10, and uses the actual cardinality63113. Exact irreducibility and32seeded field law/inverse/Frobenius controls pass. The field arithmetic is adapted from the F127 implementation; there is no independent implementation or all-input formal verification claim.

The extension Jacobian order is recomputed from actual L by Newton identities and independently by a companion-matrix determinant; reciprocal coefficient and ell divisibility checks pass. The 3-adic valuation is6. Removing the non-3 cofactor, reducing to order3 and applying freshly computed Bezout primary projectors gives two groups of27valid points. Success occurs at attempt index1/depth0. The inherited sampler does not log the precise reason for every unsuccessful internal branch; no claim of complete per-branch failure diagnostics is made.

Each group passes729pairwise closure checks, order3 and Frobenius stability. Their intersection consists only of zero. All729pairwise sums across the two groups are distinct valid order3points, matching the full geometric3-torsion size for a genus3Jacobian in characteristic not3. A corrupted monic leading coefficient is rejected. run/KERNELS.txt stores both complete groups and the field modulus.

The two irreducible coprime nonselfreciprocal primaries are T³+T²+2 and T³+2T+2. A Frobenius-invariant Weil pairing can pair one primary only with its reciprocal, so each3-dimensional primary group is isotropic and hence maximal. This is a standard readable symplectic argument, NOT an explicitly computed Weil-pairing table or a Lean geometry proof. No pairing values were computed by this gate.

## Scope and next construction

This is actual larger-field torsion/kernel data, beyond a polynomial compatibility flag. It is still not normalization, quotient geometry, a target curve, transfer images or a DLP solve. The implementation is fixed to this registered F631case; cardinality and intermediate bounds are not generalized to arbitrary primes. The source is selected by a stated compatibility rule, not a generic sample or security result.

An initial serialization build error (String += char) is retained in BUILD-PRODUCER.txt. Corrected successful source/binary/input hashes were committed before scientific execution. No scientific rerun or unexpected failure occurred. Kernel construction, controls and their verification must be charged in any future timing comparison. No timing, exponent, novelty or tenfold gain is claimed. Genus2/4 and the full objective remain open; prior failed cold-cost result unchanged.

Next: construct the source kernel functions, compatible normalization and quotient geometry from these fresh groups; require model/Jacobian identity and source-target verification before using destination IC. Do not reuse F127normalization, coordinates, images or logs as inputs.
