# Base polynomial root multiplicity at the actual simple branch

[pcc:unverified] Seven prospective Lean declarations complete the source-level base-polynomial transport left by 332575b279. The predecessor's independent review964e59f1cc was read. No compiler, tests, axiom readback or scientific execution was run. All new source remains uncompiled.

## Exact construction and primary APIs

Let R be the actual weighted-Proj stalk at the rational simple branch (a,0), with the existing DVR normalization vP(y)=1 and vP(x-a)=2. E is the actual branch polynomial evaluator from the predecessor. First derive branchEval(affineEval(g))=g(a) by ring-hom extensionality. Constants use the quotient's actual scalar map, and X uses branchEval_xbar.

The kernel of branchEval is the actual branchIdeal. Mathlib IsLocalization.AtPrime.isUnit_to_map_iff identifies units after localization with the complement of that prime ideal. Transport units both ways across branchRingEquiv. This proves IsUnit(rho(t)) iff branchEval(t)!=0, and the predecessor evaluator equality then proves IsUnit(E(g)) iff g(a)!=0. The unit criterion is derived; it is not an arbitrary premise.

For g!=0 set m=rootMultiplicity(a,g) and t=g /ₘ (X-C(a))^m. Mathlib pow_mul_divByMonic_rootMultiplicity_eq gives the exact identity (X-C(a))^m*t=g, and eval_divByMonic_pow_rootMultiplicity_ne_zero gives t(a)!=0. The latter genuinely requires g!=0. Map the factorization through E. The new residue criterion makes E(t) a unit, so its additive valuation is0. projectiveX_sub_scalar identifies E(X-C(a)) with projectiveXSubA; projectiveXSubA_order_two supplies its order2. AddValuation.map_mul and map_pow now derive vP(E(g))=m*2 in Nat-infinity. This is the desired transport as a conclusion, with no transport identity assumed.

The finite natural multiple of2 cannot be top. The Nat theorem explicitly records this before applying ENat.toNat_mul/toNat_natCast to conclude orderNat(E(g))=2m. The zero-polynomial countermodel is retained: rootMultiplicity(a,0)=0 by convention, while vP(E(0))=top. Dropping g!=0 would make the ENat equality false and a careless toNat conversion misleading.

## Norm support corollary

At a simple branch support root with b!=0 and the polynomial norm identity, the predecessor gives vP(Eu1)+vP(Eu2)+vP(EU)=2. If any support polynomial were zero, its germ would have top valuation and the sum could not be finite2. The new corollary therefore derives u1!=0,u2!=0,U!=0 rather than assuming them. Apply the polynomial transport to each, obtain 2m1+2m2+2m3=2 and conclude m1+m2+m3=1. This establishes the base normalization at source level. Combining with the predecessor's source_norm_identity specialization provides the actual coefficientData application; this packet does not add a second expanded coefficient wrapper.

The supported objects remain the actual rational branch stalk and nonzero base polynomials. The generic polynomial transport needs a simple root, not a presumed norm. The norm support corollary needs the actual polynomial identity and nonzero b. Characteristic2 is not excluded by this local algebra statement; no separable double-cover interpretation in characteristic2 is inferred.

## Nonvacuity and adverse cases (hand derivations)

For any m>=0, g=(X-a)^m has base multiplicity m and stalk order2m. A nonzero constant has both orders0. On the source fixture f=X⁵-X, a=0, u1=X²-X,u2=X²+1,U=-4(X+1), the base multiplicities are(1,0,0), stalk orders(2,0,0), selected numerator factor order1. For a nonsimple point, the DVR/simple-root hypothesis is unavailable; no arbitrary vP(x-a)=2 assumption is substituted. These are symbolic checks and prior hand fixtures, not executed test results.

## Verification boundary and next work

No new mathematical API gap remains in the stated source construction: exact factorization, unit criterion, parameter order and valuation APIs were located. Remaining uncertainty is Lean elaboration/import/instance/tactic acceptance and the full dependency closure, not an assumed local identity. Seven print-axioms requests are source text only. Bind the exact import closure and independently review then compile/audit under a distinct authorized protocol; this task authorizes none of that execution.

Infinity, nonrational closed points/residue degrees, a single global function field and finite support/global principal-divisor assembly remain separate constructions. Earlier frozen evidence and sources were not edited. No overall numerical or runtime claim is made.

Sources checked (KB): unavailable in this runtime; fallback exact primary paths and hashes in input-bindings.json. Reviewed APIs: Polynomial.Div lines544/670, Localization.AtPrime.Basic line156, Valuation.Basic AddValuation.map_pow line1192, ENat.Monoid toNat_mul line70, and actual Common22BranchStalk/WeightedProjFiniteBranchDVR. The normalization agrees with already-read Stacks00PD (https://stacks.math.columbia.edu/tag/00PD) and0EXQ (https://stacks.math.columbia.edu/tag/0EXQ). This packet introduces no novelty claim.
