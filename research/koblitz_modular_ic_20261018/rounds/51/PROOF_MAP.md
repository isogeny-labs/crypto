# Five kernel-checked overlap statements

Original attempt overlap-lean51-01 has compile0 and audit0. Audit5 mathematical
declarations,zero failing,zero trusted/native_decide. Pinned Lean4.34.1 and
mathlibd13f23b as recorded in the runner and protocol. The successful printed
audit preceded a filesystem teardown delay;the final exit receipt was
reconciled before the next mathematical invocation.

| Declaration | Exact proposition boundary |
|---|---|
| first_square_identity | h0²+z*q0=E in a commutative ring with explicit 2=0 |
| second_square_identity | h1²+z*q1=E under the same hypothesis |
| overlap_norm_identity | h0*h1+z(z+1)=E under the same hypothesis |
| localized_base_inverse | z*q=h² and h*s=1 imply z*(q*s²)=1 |
| inverse_maps_of_compatible_embeddings | injective compatible embeddings into a common set with inverse ambient maps imply inverse induced maps |

The final theorem is valid at the level of functions;to apply it as an
algebraic gluing statement,existence of the native ring maps,injective field
embeddings,the actual inverse/compatibility identities and open-cover geometry
must be supplied separately. CAS arrays,complete RR identification,primality,
normalization and proper polarized embeddings are not encoded in this Lean
module. The native gluing checker supplies finite coordinate/module products;
its acceptance is independently recorded rather than inferred from Lean.
