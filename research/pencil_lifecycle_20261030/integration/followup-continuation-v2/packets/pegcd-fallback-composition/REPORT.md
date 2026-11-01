# Extended Euclidean fallback composition

Producer proof packet, 2026-11-01. Independent source-fidelity review pending.

Result. The six-register extended-Euclidean transition preserves both Bezout identities and the common-divisor invariant. An actual nonmonic word-division step supplies its quotient and remainder. Two completed polynomial traces, normalized by nonzero scalars, imply both native fallback divisibilities: D squared divides Ua times Ub, and D divides the stored numerator expression.

Checks. Six freshly compiled local modules contain 11 main declarations and four fixed producer controls. Fifteen exact named axiom readbacks use only the standard allowed axioms. The false theorem 0 = 1 fails compilation, and three native checker rejection controls pass. Per-module source/dependency hashes are checked before and after. Imported Mathlib and other local helper caches are reused.

Endpoint boundary. A nonzero first input implies a nonzero final gcd. A zero second input is covered by the initial-state invariant and stopping condition. Normalization scales the gcd and both Bezout coefficients by the same nonzero scalar. The native last().unwrap() requires a nonempty normalized gcd representation; the polynomial nonzero theorem is one component of that remaining storage correspondence.

Assumptions. The word-division bridge uses a prime modulus with 1 < p < 2^31, canonical coefficient lists and a canonical nonzero last divisor coefficient. The two-trace theorem assumes terminating polynomial traces, nonzero normalization scalars and Ua dividing f minus Va squared. It derives divisibility without assuming quotient witnesses for either fallback division.

Scope. The polynomial trace theorem does not yet establish termination or complete six-register word-list correspondence for the native loop. The final normalization scalar is not yet connected here to the native generated inverse. The complete geometric validity of the fallback output remains a further composition task. No cohort or timing run was performed; cost ratio is null.

Controls and retained failures. Fixed controls check the quotient subtraction sign, zero second input, rejection of an unscaled Bezout result, and exclusion of a zero endpoint for X. Earlier failures include missing namespace and syntax, an unavailable unit-cancellation lemma, and an unavailable Ruby editing command that caused one unchanged failed compile. All root continuation attempt receipts are retained; the original agent draft remains preserved separately.

Source and prior work. Native source: src/cryptanalysis/three_leads.rs, pegcd and fallback sections, at 246d5ece6519dd0e645a1de95473ede081cf9aa4. The sibling next-interface-derivations packet contains the independently reviewed written construction and literature context; the word-find-composition packet carries the native source snapshot. No novelty or performance claim is made.

Next review. Check register order and subtraction signs against native code, trace hypotheses, zero-input handling, normalization and both numerator divisibilities. Independently rebuild the named modules and helpers. The assigned reviewer agents remain unavailable after usage-limit errors; this is producer evidence only.
