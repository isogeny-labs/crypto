# Eleven newly audited mathematical statements

Both original attempts are terminal:overlap-lean51-01 compile0/audit0 and
specialize-lean51-01 compile0/audit0. The axiom checker reports5 and6 theorems,
respectively,zero failing and zero trusted/native_decide. All individually
named mathematical declarations report ok. Pinned Lean4.34.1 and
mathlibd13f23b remain unchanged. The first attempt's filesystem teardown was
reconciled before the next mathematical invocation.

| Module/declaration | Exact proposition boundary |
|---|---|
| Overlap51.first_square_identity | h0²+z*q0=E in a commutative ring with explicit2=0 |
| Overlap51.second_square_identity | h1²+z*q1=E under the same hypothesis |
| Overlap51.overlap_norm_identity | h0*h1+z(z+1)=E under the same hypothesis |
| Overlap51.localized_base_inverse | z*q=h² and h*s=1 imply z*(q*s²)=1 |
| Overlap51.inverse_maps_of_compatible_embeddings | inverse ambient maps and injective compatible embeddings imply inverse induced functions |
| Specialize51.mem_of_unit_mul_mem | cancel a certified denominator unit from ideal membership |
| Specialize51.ideals_eq_of_unit_scaled_inclusions | unitd and scaled inclusions d*I subset J,d²*J subset I imply I=J |
| Specialize51.seven_nsmul_zero_of_eight |8P=P implies7P=0 in an additive group |
| Specialize51.line_intersection_identity | exact characteristic2 line/Tate-cubic intersection polynomial |
| Specialize51.quartic_difference_identity | exact psi3(x)+psi3(t)=(x+t)Q under2=0,without inverseA |
| Specialize51.cross_kernel_denominator_inverse | the actual quartic formula and supplied Bezout/point identities give the explicit inverse of x+t when Delta!=0 |

The last inverse theorem assumes the supplied value g=psi7(x) is zero and
psi3(x)U+gV=Delta^16. The native LINE/COLLISION certificates supply those
actual point/Bezout identities;their arrays are not encoded in Lean.
Likewise the ideal theorem assumes the scaled inclusions obtained from the
native full polynomial matrices. The group theorem does not encode the
native point coordinates or prove the elliptic group-law formulas.

The native chart data supply actual coordinate/module products. Complete RR
identification,source primality where retained,geometric open cover,scheme
normalization,proper polarization and selected modular-factor action are not
new unconditional Lean conclusions. GLUING51.md and CM.md state the written
geometric premises. External TauCeti source is a lead only;none of its newer
toolchain declarations is counted as locally compiled or audited.
