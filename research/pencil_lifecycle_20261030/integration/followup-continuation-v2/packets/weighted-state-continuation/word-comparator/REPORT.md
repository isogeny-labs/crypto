# Literal narrow-word Kummer comparator bridge

Eight main declarations and ten kernel controls passed. Separate readback for all 18 declarations uses only standard propext/Classical.choice/Quot.sound axioms. The producer-checked result binds literal narrow UInt64 products and the nested16 Boolean comparisons to the decoded field comparator for canonical coordinates and prime0<p<=2^32. Raw vector nonzero is equivalent to decoded nonzero under canonicality. Composing these with the existing field comparator theorem gives equivalence to a nonzero projective scale when both raw vectors are nonzero.

## Source correction and arithmetic

The source at snapshot/walk-source.rs.txt:14-17 uses x*y%p in UInt64, with a counter increment. It does NOT use the graph Ring's widened u128 multiply. No wide operation was substituted in the model. Lines51-53 implement nested all over0..4 for16 cross-product equalities. The model uses Fin4 coordinates and List.finRange4 nested all, preserving the Boolean comparison set. Function-valued coordinate encoding abstracts storage; compiler, Rust iterator implementation and counter semantics are not verified.

The stronger safe bound p<=2^32 is proved directly: canonical a,b give a,b<=2^32-1 and hence a*b<=18446744065119617025<2^64. Product reduction decodes exactly. Canonical residue casts are injective, so equality of reduced UInt64 words is equivalent to equality in ZMod p. Those two facts compose pointwise into sameB_iff. A separate zero-cast equivalence yields nonzeroB_iff.

No caller guard establishes p<=2^32 merely from this theorem: source/caller range validation remains explicit. Composite modulus2^32 is a legal arithmetic boundary control but not an instance of the prime-field theorem.

## Controls and failures

Controls cover scaled vectors, distinct axes, zero matching a nonzero vector, last-coordinate nonzero, all-zero vector, endpoint multiplication, prime4294967291 near the boundary, an overflow false positive at modulus4294967311 with raw cross-products wrapping to zero while the mathematical cross-product is225, and noncanonical coordinate17 modulo17 giving raw nonzero but decoded zero. The out-of-bound UInt64 examples describe wrap semantics; overflow-checking Rust builds may panic instead.

Main proofs passed initially; the final composition theorem also passed. First control attempt lacked an explicit Fact instance for the UInt64-decoded modulus17. Second required unfolding the nonzero predicate for kernel decidability. The premature readback after that failed control has its import failure retained. The third control attempt and second readback passed. No native_decide was used.

## Boundaries and custody

This closes the conditional word/Boolean comparator obligation at the stated coordinate-value abstraction. It does not prove the Kummer projection's sign fibers, xadd closure, chart formulas, curve group correctness, compiler equivalence, counter behavior or caller canonicality. Existing Comparator.lean was freshly compiled; transitive retained Product/Mathlib objects remain a cache trust boundary. Independent review is pending. Exact source snapshots, module-level pre/post bindings, commands, stdout/stderr and receipts are preserved. No scientific execution, curve evaluation or performance measurement occurred. Full1260-target scope and null cost ratio remain.

Final additional bridge `nonzeroB_iff_ne_zero` proves the Boolean any test equivalent to inequality with the all-zero coordinate vector, without modulus assumptions. This matches the native array-zero check at the coordinate-value abstraction. Its proof uses deprecated push_neg (warning retained), not a trusted native evaluator. The final main/control/readback rebuilds passed.
