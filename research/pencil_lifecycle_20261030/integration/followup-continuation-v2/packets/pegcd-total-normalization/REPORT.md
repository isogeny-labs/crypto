# Finite remainder trace and normalization

Producer proof packet, 2026-11-01. Independent source-fidelity review pending.

Result. A finite trace is constructed from actual nonmonic word-division returns and strict decreases of the remainder list length. The stored-output version retains a canonical, normalized gcd list and its polynomial decoding. For a nonzero first input, this list is nonempty and its last coefficient is nonzero. The generated inverse is connected to the common scalar normalizing the gcd and both Bezout coefficients.

Checks. Five fresh local modules contain five main declarations and three fixed producer controls. Eight exact named axiom readbacks pass the standard allowlist. The false proposition 0 = 1 is rejected, and three native checker rejection controls pass. Per-module source and dependency bindings are checked before and after compilation. Imported helper and Mathlib caches are reused.

Assumptions. Prime modulus 1 < p < 2^31; canonical input coefficient lists; normalized second input for trace existence. The stored-output theorem additionally requires normalized first input. The normalization endpoint additionally requires a nonzero decoded first input. These assumptions are explicit in the archived statements.

Proof boundary. The termination construction advances actual word remainder lists, while the four auxiliary Bezout registers remain polynomials. It supplies a completed polynomial trace and nonzero generated normalization scalar. Full word-list correspondence for those auxiliary registers and final scaling, physical allocation and stack bounds, and complete geometric fallback output validity remain separate interfaces.

Controls. The fixed cases reject a list ending in zero as normalized, show that an empty first input does not satisfy the nonzero-input premise, and construct the exact zero-second-input stored trace. No scientific cohort or timing data was collected; cost ratio is null.

Failures retained. The first trace draft omitted its prime-field instance parameter. A generic module name Stored then shadowed an existing dependency, creating an import cycle after compilation; three endpoint checks and a minimal import check timed out at 120 seconds. Uniquely prefixed module names removed the collision and all final modules compiled. One control initially lacked a decidability instance for the opaque Normal predicate and was repaired by unfolding that predicate. Original sources and all attempt receipts are preserved.

Source context. Native src/cryptanalysis/three_leads.rs at f7e629479237293c0e4deed6515d22adec9b5ba7, pegcd register loop and normalization. The sibling pegcd-fallback-composition packet establishes the prior invariant and two fallback divisibilities; next-interface-derivations contains the reviewed written construction and literature context. No novelty or performance claim is made.

Next review. Independently check strict length decrease, list normalization, empty-input cases, module resolution and native register order. Connect the four auxiliary polynomial registers and scaled outputs to stored word lists, then compose the geometric output proof. Reviewer agents remain unavailable after usage-limit errors; this packet is producer-checked only.
