# CoefficientData branch multiplicity specialization

[pcc:unverified] T-1732 adds one prospective source theorem, `BranchCoefficientMultiplicity.source_support_base_multiplicity_sum`. No Lean compiler, tests, axiom readback or experiment was invoked. Independent source review remains pending.

## Exact composition

The previous `support_base_multiplicity_sum` was generic in a polynomial norm identity. This wrapper instantiates its b with the first component of actual coefficientData, n with ActualNonbranchNorm.numerator, u1 and u2 with firstSupport and secondSupport, and U with ActualNonbranchNorm.residual. It supplies the identity using the existing `source_norm_identity`, whose inputs are b nonzero and the two divisor congruences. No norm identity is assumed in the new statement.

Separability and the retained branch equation fAt(a)=0 give derivativeAt(a) nonzero using `WeightedProjQuinticHead.derivativeAt_ne_zero_of_separable`. Thus the generic theorem applies on its actual branch DVR. Its conclusion retains all three support-polynomial nonzero conjuncts and the sum of base root multiplicities equal to one. Those nonzero facts are conclusions, not new hypotheses. The generic proof establishes finite stalk sum two before applying finite-order/base-multiplicity transport; the wrapper makes no new ENat.toNat conversion.

## Assumptions and boundary

The field, rational branch point, separability, nonzero b, both exact divisor congruences and support-root product equation are explicit. The root may belong to any of the three supports; no choice is presupposed. The theorem does not assert a result at a singular branch or infinity, or add a separable-double-cover interpretation in characteristic two. It retains the predecessor's restriction excluding zero polynomials through its proved conclusions; rootMultiplicity(a,0)=0 is not used to conceal infinite valuation. It adds no axioms, sorry, native_decide or assumed norm bridge. A print-axioms request is source text only.

## Source checks and prior work

Read the exact predecessor source and independent review, the coefficientData numerator/residual/source_norm_identity definitions, and WeightedProjQuinticHead's derivative theorem. `input-bindings.json` pins their bytes. The research-kb tools are absent from this runtime's exposed catalog, so exact primary-source fallback was used; no novelty claim is made. This is direct composition of existing derived identities, not a new general valuation result. The earlier report and diagram remain unchanged; the new diagram describes only this specialization.

## Next verification

Publish these exact inputs for independent source-fidelity review. Bind the full import closure and obtain authorized kernel elaboration and named axiom readback separately. The current packet is source-only and makes no performance or measured-cohort claim. The existing frozen cohort is unchanged.
