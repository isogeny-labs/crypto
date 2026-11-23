# Fallback norm identity and stored remainder invariant

Producer proof packet, 2026-11-01. Independent source-fidelity review pending.

Result. Both input congruences imply that Ua*Ub divides f*D squared minus N squared. Combining this with the proved stored-quotient identities and cancelling nonzero D squared yields U dividing f-W squared. The actual modeled stored remainder V of W by U is canonical, normalized, strictly shorter than U, and satisfies U dividing f-V squared. The endpoint constructs both EEA runs, both exact quotients and the remainder from the input premises.

Algebra. Write D=x*Ua+y*Ub+z*(Va+Vb) and N=x*Ua*Vb+y*Ub*Va+z*(Va*Vb+f), with x=c1*e1, y=c1*e2, z=c2. A ring identity expands f*D squared minus N squared into terms containing both input residuals and Ua*Ub. Replacing the residuals by their divisibility witnesses makes the product factor explicit. Cancellation requires a domain and nonzero D; both are established for the polynomial setting used by the word endpoint.

Checks. Five local modules freshly compiled; seven main declarations, six named controls and thirteen exact axiom readbacks. The false proposition 0 = 1 is rejected and three native checker rejection controls pass. Each module's bound source and dependency files pass before/after hash checks. Imported helper and Mathlib caches are reused.

Assumptions. Prime modulus 1 < p < 2^31; canonical input coefficient lists; normalized nonzero Ua and Ub; and both input divisibilities Ua | (f-Va squared), Ub | (f-Vb squared). No smoothness premise is used for this congruence. The cancellation lemma is stated over a commutative domain, so it applies to the polynomial ring rather than only to field elements.

Fixed controls. Over F17, remainder of X squared plus 1 by X plus 1 is 2; a constant divisor 1 produces empty remainder; (3+6X) modulo 3 also produces empty remainder; an empty divisor is rejected. Integer boundary controls record that the first divisibility alone need not imply the second, and equality after multiplication by zero cannot justify cancellation. These are fixed producer controls, not scientific cohort measurements.

Proof boundary. The proved endpoint is the congruence and stored-list shape entering reduce. The subsequent monic normalization, degree-reduction loop, final fixed-array construction and divisor-class interpretation remain separate work. Physical allocation/stack bounds and compiled-Rust fidelity also remain outside this packet. No new cohort, timing or cost ratio is reported.

Development record. The cancellation theorem was first compiled for a field, then generalized and freshly checked for a commutative domain before use with polynomials. The remainder length statement was strengthened from an empty-or-shorter alternative to a strict bound using nonzero U. These successful intermediate versions are retained. There were no unexpected compiler failures in this construction; the deliberately false theorem is recorded separately.

Source and prior work. Native src/cryptanalysis/three_leads.rs at e0f9e58e4a76dc2a9b471135dd5d5ef1184ee098, fallback numerator, exact divisions, prem and entry to reduce. Sibling fallback-word-divisions supplies the stored quotient identities; pegcd-six-word-endpoint supplies the EEA word proofs; next-interface-derivations records the reviewed written construction and literature context. No novelty or performance claim is made.

Next review. Independently check the norm expansion, use of both residual premises, domain cancellation, coefficient wiring and remainder congruence. Then bind the degree assumptions needed by the normalization/reduction loop to the actual curve inputs. Assigned reviewer agents remain unavailable after usage-limit errors; this packet remains producer-checked only.
