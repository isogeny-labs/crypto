# Same-u zero-slope exception exclusion

## Result

Three lemmas compiled and passed separate axiom readback: equal_slope_classification, cross_scales and source_zero_slope_excluded. The native same_u_mixed_zero_slope_delta exception is excluded at field-expression scope from original validity and the preceding guards.

## Original premises

Arbitrary field; two weighted degree-two inputs satisfying their normalized Mumford divisibility conditions; four nonzero representation weights; literal crossed q and r equalities; earlier inverse guard false; earlier equal-ordinate guard false. No smoothness or characteristic restriction is needed.

## Derivation

The reviewed factor_identity says (v1-v2)(v1+v2) equals C((s1-s2)(s1+s2))*u. Equal slopes make the right side zero, so the polynomial integral-domain property gives v1=v2 or v1=-v2. Coefficients give the exact equal/inverse alternatives. Nonzero cross-weight scales transport literal guards into normalized coordinates and back. If literal sourceK were zero, one of the preceding guards would have held, a contradiction.

## Exact source scope

Retained projective_law_gate.rs lines903-933: crossQ/R/S/T retain native multiplication association, including z^3 and z^5. sourceK is the previously bound literal subtraction. This closes an ordered-guard obligation, not a new payload formula. PROOF_AUDIT.md already describes the degree argument; the added result is explicit Lean composition with the crossed source guards.

## Controls and failures

A full original-validity application over Q uses f=X^5, u=X^2, v1=X, v2=2X. Equal and inverse controls retain zero slope; an F2 case uses the same classification without an odd-characteristic premise. A failed-validity counterexample has matching slopes but neither earlier guard. Initial main tactic/rewrite failure and four control failures (heartbeat, missing polynomial evaluation API, and coefficient simplification) are retained. Final control uses coefficient zero instead of importing extra evaluation modules.

## Limits and review status

Independent source review pending. The proof relies on reviewed imported factor identity and weighted normalization definitions; standard axiom readback lists only propext, Classical.choice and Quot.sound. No UInt64 equality reflection, compiler/counter execution, divisor class-sum or complete degree2+2 dispatcher result is claimed. No scientific trial was run and cost ratio is null.
