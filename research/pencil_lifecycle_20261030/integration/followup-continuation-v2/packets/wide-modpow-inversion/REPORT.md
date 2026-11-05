# Widened modular powers and generated inverse certificates

T-1154, 2026-10-31. Producer formal/source-model packet. Independent source-fidelity review pending.

For arbitrary UInt64 operands, their product fits in 128 bits. The checked BitVec128 model of source widening, multiplication, remainder and narrowing equals exact natural modular multiplication for positive modulus; the remainder fits back into UInt64. This primitive does not need the earlier p<2^31 restriction.

The binary-loop proof models the source parity test, one-bit right shift and skipped final squaring. Its invariant is decoded accumulator times decoded base raised to the remaining exponent. A fuel theorem assumes e<2^fuel; 64 steps cover every UInt64 exponent. For 1<p and canonical base, the modeled modpow returns a canonical word decoding to the expected power, including exponent zero.

For prime p and nonzero canonical input a, exponent p-2 generates a canonical inverse word. The proof uses the standard finite-group-cardinality/Fermat argument through already imported lemmas. Under the prior p<2^31 multiplication-verifier bound, the generated word satisfies InverseWitness. The final theorem instantiates the earlier coefficient-update proof with this generated inverse, removing its supplied-inverse premise.

Fourteen widened-loop declarations, six inversion/composition declarations and eighteen fixed controls are checked, followed by 38 named axiom readbacks. Controls include p17 inverse2->9, identity/negative-one inverses, exponents 0/1 and2^64-1, zero base, maximum operand products and the composed coefficient-slot result. Widened2^63*2 modulo5 is 1 while narrow UInt64 multiplication gives 0. For composite p15,a2, the p-2 generator returns 2 and fails the inverse certificate. Zero modulus, modulus one and zero inversion input remain outside the respective theorem domains.

The literal source excerpts bind Field::bi, the Field::inv dispatch and modpow at three_leads.rs195-200,307-332,588-600. The source path is odd-characteristic n=1 arithmetic without diagnostic tables. BitVec128 is the explicit logical model for Rust u128 operations; this is not a compiler or full-array execution proof. Independent source-fidelity review, array-loop composition and geometric class transport remain separate obligations.

The first widened proof attempt needed the explicit UInt64.ofNat conversion lemma. The first Fermat attempt needed an implicit argument instantiated. Corrected attempts and controls passed; nonfatal tactic-deprecation warnings are retained. Two exploratory cache planners failed on import parsing; no cache was copied, downloaded or built. The proof instead reused existing group-cardinality imports. Cache-planning error summaries are retained separately from native Lean receipts. Every Lean check records native exit and unchanged input bindings; readback admits only standard Lean axioms.

Primary sources: Lean Bitvectors logical semantics (https://lean-lang.org/doc/reference/latest/Basic-Types/Bitvectors/); Mathlib FiniteField.pow_card_sub_one_eq_one and ZMod Fermat documentation (https://leanprover-community.github.io/mathlib4_docs/Mathlib/FieldTheory/Finite/Basic.html), checked against pinned source. Established identities are reused; no novelty claim.

No scientific campaign, curve enumeration, timing or cost ratio. Frozen cohort and campaign limits unchanged. The diagram represents proof dependencies and scope; independent binary rechecking is not claimed.
