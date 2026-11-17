# Computed scale, trimming and divisor storage

T-1174, 2026-10-31. Producer proof packet; independent source-fidelity review pending.

The source-model small power now supplies the nonzero canonical scale and the equality required by the T_m normalization theorem. The complete word transform followed by terminal-zero trimming and zero padding represents T_m(f), without a separately supplied scale-equality premise.

The scale theorem retains prime 1<p<2^31, canonical nonzero alpha, canonical beta and coefficients, length<=6 and exponent m in 0..5. The list polynomial laws need only a commutative ring. The trim model tests getLast?=some 0 and removes dropLast; it preserves the represented polynomial for any fuel and finishes with no terminal zero whenever fuel is at least the original length. Padding appends zeros, preserves the polynomial, and reaches the requested capacity when the original length fits. Over-capacity inputs are not truncated and do not satisfy that capacity theorem.

A coefficient theorem connects the list polynomial to the preceding array-decoding representation. The composed theorem then identifies the trimmed/padded output with T_m(f). For source divisor storage, the intended capacities are 3 for u and 2 for v; source degree/domain validity remains to be composed with the earlier Mumford validity theorem. This is a logical model of scale, last/pop and zero-initialized prefix copying, not compiled-Rust equivalence or a geometric class transport proof.

Seventeen main declarations and fifteen controls passed, with 32 named standard-axiom readbacks. Controls include empty/all-zero lists, internal zeros, multiple trailing zeros, insufficient fuel, capacities 2/3, forbidden over-capacity truncation, and power exponents 0/2/3. Modulo 17, the transformed u coefficient example [1,2,1] at alpha 2,beta 3,scale alpha^2 yields [4,4,1]; the separate v example [5,0] at scale alpha^3 trims and pads to [7,0]. These examples test representations, not validity of a paired divisor on a named curve.

Scale, controls and readback passed first attempt. Trim attempt 1 required an explicit dropLast-length rewrite and the available empty-list lemma; trim2 passed. Bridge attempt 1 contained compact <=i notation parsed as an initial-segment token; bridge2 passed after spacing correction. All failures and nonfatal unused-simp warnings remain recorded; final named readback contains only standard axioms.

Sources: projective_law_gate.rs transform/map_divisor, prior smallPow and polynomial-transform proofs, pinned Mathlib Data/List/Basic.lean dropLast_append_getLast?, and official Lean take/drop documentation https://lean-lang.org/doc/api/Init/Data/List/TakeDrop.html. Standard polynomial/list identities are reused without novelty claims. Existing compiled dependencies remain in the trust boundary.

Inspected PDF, editable SVG, status JSON, native receipts and custody accompany this packet. No scientific campaign, timing or cost ratio; cohort and authorization limits unchanged.
