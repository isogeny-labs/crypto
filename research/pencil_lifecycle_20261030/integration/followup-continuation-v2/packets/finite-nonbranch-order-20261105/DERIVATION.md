# Finite nonbranch multiplicity from the actual curve

[pcc:unverified] T-1613. Hand derivation and prospective Lean source, not kernel-checked. No compiler, axiom readback or experiment was run. Earlier kernel/native receipts keep their earlier scope.

## Construction

Let K be a field, F=X^5+a3 X^3+a2 X^2+a1 X+a0 separable, A=K[x,y]/(y²-F), and P=(x0,y0) an actual rational point with 2,b,y0 nonzero. Let n be the source numerator with n(x0)=b*y0. The conjugate P'=(x0,-y0) is also an actual point. If g_A=n(x)-b*y vanished as an element of A, evaluation at P' would give n(x0)+b*y0=2*b*y0=0, a contradiction. This proves nonvanishing as an affine function, despite g_A(P)=0. A zero value at the selected point is not a zero function.

The existing separable-affine-domain declaration supplies that A is a domain. Localization at the point prime is injective; the actual pointRingEquiv is an isomorphism. Their composition rho therefore transports the nonzero affine element to a nonzero element g of the actual projective stalk R. Existing h2 and hy give its DVR instance. Mathlib addVal_eq_top_iff now gives v(g) != top.

Reuse the existing source norm identity and concrete_norm_order, rather than re-proving either. Write v(g)=v(u1)+v(u2)+v(U), with every support image formed through the same germEval. Since the left side is finite, each right summand is finite: a top summand would make the sum top. Only now apply ENat.toNat_add twice. The resulting equality is an equality of natural-number orders, with explicit non-top statements for all four values. Conversely addVal_eq_top_iff shows each support germ is nonzero. The existing numerator_nonunit and addVal_eq_zero_iff also imply selected g has positive order; positivity is a written consequence here, not a separate prospective Lean declaration.

Five prospective declarations implement these joints: affine_selected_ne_zero, germ_selected_ne_zero, finite_sum_toNat, actual_norm_order_nat, source_actual_norm_order_nat. The last supplies its polynomial norm hypothesis from source_norm_identity and the actual coefficient/divisibility premises; it does not assume the wanted valuation equation or principal divisor. Exact elaboration and imports remain to be checked under a future bounded release.

## Why this route and its limits

The selected-point argument reuses already available evaluations. It needs no new degree proof. A complementary stronger argument uses the monic degree-two polynomial Y²-F(X): n(X)-bY has Y-degree one, so cannot belong to its principal ideal when b !=0. Transporting that proof through the existing nested polynomial equivalence would remove the rational nonbranch witness from affine nonvanishing. Alternatively F has odd degree5, whereas a nonzero polynomial square has even degree, so n²=b²F cannot hold. These are derivation alternatives, not implemented duplicate proofs.

Controls for independent review: b=0,n=0 makes g=0 and v(g)=top. Omitting y0 !=0 makes the conjugate argument inconclusive, even when g is actually nonzero. In characteristic2 the conjugate points coincide, so their evaluation difference vanishes. A nonzero element of a ring with zero divisors may vanish after localization; the actual domain hypothesis must not be dropped. Finally ENat.toNat(top)=0: applying toNat before excluding infinity can silently report zero multiplicity. No claim is made that all of these method obstructions are counterexamples to stronger geometric statements.

## Global divisor prerequisites

This supplies a finite order at the specified rational nonbranch stalk only. A global divisor requires a single nonzero rational function on the same integral projective curve, compatible embeddings into all codimension-one local fraction fields, branch-point and infinity orders, and finite support. Over non-algebraically-closed K, rational points alone do not cover all closed points; residue degrees or explicit base-change/descent are required. Repeated supports retain multiplicities. The same function must have the pole order6 when the cubic numerator coefficient is nonzero and5 in its lower-degree case. Finally identify the concrete principal divisor and genuine class quotient with the source output; no arbitrary principal-class premise substitutes for this construction.

## Sources and verification status

Local source hashes and primary references are in sources.json. Stacks00PD supplies the nonzero DVR factorization underlying finite valuation; Stacks02RJ defines global orders using nonzero rational functions and codimension-one stalks. Exact local APIs inspected: Common22SeparableAffineDomain.affine_ring_domain_of_separable; FaithfulSMul.algebraMap_injective as used in WeightedProjNonbranchDVR.stalkYSubY_ne_zero; actual pointRingEquiv; IsDiscreteValuationRing.addVal_eq_top_iff and addVal_eq_zero_iff; ENat.toNat_add. The new Lean file is prospective source, has no holes or new axioms, and has not been compiled. An independent source-fidelity review and named axiom readback remain required. Frozen cohort rows, cost records and prior failures are unchanged.
