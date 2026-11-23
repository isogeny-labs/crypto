# Normalization preserves polynomial support multiplicities

T-1186; 2026-10-31. Producer proof packet; independent source-fidelity review pending.

For any field, a nonzero scale a, arbitrary nonnegative exponents n,m,k and polynomials u,f, normalization preserves all power-divisibility predicates: T_n(u)^k divides T_m(f) if and only if u^k divides f. The proof supplies an explicit quotient in the forward direction and applies the previously checked inverse normalization for the converse. Power compatibility is T_n(u)^k=T_(nk)(u^k).

The normalized linear factor is T_1(X-r)=X-(r-b)/a. Consequently, exact finite polynomial root multiplicity is preserved at the transformed root. The predicate used is explicit: (X-r)^k divides f and (X-r)^(k+1) does not divide f. It does not silently assign a finite multiplicity to the zero polynomial. An additional theorem preserves all powers dividing the Mumford residual f-v^2, using the previously proved square compatibility of T_3(v) with T_6(v^2).

Eight main declarations and eight controls are exposed for separate axiom readback. Rational controls distinguish a double root from a simple root and an incorrect root. The fixture (X-5)^2 at a=2,b=3 has exact multiplicity two at the transformed root 1. The zero polynomial is divisible by every power but has no exact finite multiplicity under this predicate. Exponent zero is included; a=0 collapses the positive-weight transformation and is excluded from the equivalence theorems.

This is polynomial support and residual multiplicity, not yet the local valuation of a function on a curve. Connecting it to divisor multiplicities requires the local curve model, including ramified support. Extension at infinity and principal-divisor transport also remain in the geometric class construction. The affine signed-support correspondence is in the preceding normalization-inverse-support packet; word/storage validity is in stored-divisor-validity. Existing compiled upstream dependencies remain within the trust boundary.

Sources checked: prior inverse/support modules, canonical genus2_model_gate.rs with named extraction, and pinned Mathlib Algebra/Polynomial/AlgebraMap.lean (algEquivCMulXAddC and dvd_comp_C_mul_X_add_C_iff). Official documentation: https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/AlgebraMap.html. The implementation uses explicit quotient witnesses and the existing inverse, without new compiled dependencies or a novelty assertion.

All four final native checks passed with unchanged input hashes. Separate readback of all 16 declarations uses only propext, Classical.choice and Quot.sound. Outcomes are recorded in status.json. The first proof attempt failed to reassociate multiplication in the linear-factor identity; the corrected attempt uses an explicit reassociation. Failed attempts remain available. No scientific campaign, timing observation or cost ratio.
