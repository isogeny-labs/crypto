# Six word-list registers and scaled outputs

Producer proof packet, 2026-11-01. Independent source-fidelity review pending.

Result. All six extended-Euclidean registers are modeled as UInt64 coefficient lists. Stored-quotient division, list multiplication and subtraction implement each transition. A finite trace exists under the stated input conditions; all registers remain canonical and the trace decodes to the polynomial invariant. The final generated inverse scales three actual word lists, whose decodings satisfy the Bezout identity and common-divisor characterization.

Endpoint. For canonical normalized input lists a and b, prime modulus 1 < p < 2^31 and nonzero decoded a, the theorem constructs a finite Run from the literal initial registers, a state with empty second remainder, and successful finish returning canonical normalized d, s, t. The decoded identity is d = s*a + t*b; d divides both inputs; a polynomial divides both inputs exactly when it divides d. The proof does not assume termination or auxiliary polynomial-register values as inputs.

Checks. Seven local modules freshly compiled, ten main declarations, five named fixed controls and fifteen exact named axiom readbacks. The controls use five private helper lemmas for explicit intermediate values; their dependencies are included in the final theorem axiom readbacks. The false proposition 0 = 1 is rejected and three native checker rejection controls pass. Source and dependency hashes are checked per module. Imported helper and Mathlib caches are reused.

Fixed outputs. Over F17, the exact-division case a=X squared minus 1, b=X minus 1 returns d=X minus 1, s=0, t=1. The two-step case a=X squared plus 1, b=X plus 1 returns d=1, s=9, t=9+8X. The second case exercises a constant remainder 2 and generated inverse 9. A separate control checks the subtraction sign in the first auxiliary update.

Proof boundary. This is a source-shaped mathematical model with native word arithmetic and list operations, not a compiled-Rust equivalence proof. List lengths and recursion are mathematical values; physical allocation, machine-size capacity and stack bounds remain outside this packet. Monicity is not a separate endpoint theorem here. Complete geometric validity of the genus-2 fallback output remains a further composition task. No scientific cohort or timing run was performed; cost ratio is null.

Failures retained. One division proof initially supplied an equality after case analysis had already rewritten its goal. Two output controls initially could not reduce the well-founded list-addition definition through decide, including a kernel-reduction attempt. Explicit intermediate product, negation and list-addition lemmas resolved this without adding axioms. All failed attempt receipts are retained.

Source and prior work. Native src/cryptanalysis/three_leads.rs at 3ac4249a57c1784d10938a83951e653ea8ccb29d, polynomial primitives and pegcd loop/output. Prior packets pegcd-fallback-composition and pegcd-total-normalization establish the earlier algebraic and partial storage interfaces; next-interface-derivations contains the reviewed written construction and literature context. No novelty or performance claim is made.

Next review. Independently map register updates, quotient storage, subtraction order, leading coefficient selection and final scaling to the native source. Replay controls and audit theorem premises. Then compose the two full word endpoints with the fallback divisions and geometric output proof. Assigned reviewer agents remain unavailable after usage-limit errors; this packet remains producer-checked.
