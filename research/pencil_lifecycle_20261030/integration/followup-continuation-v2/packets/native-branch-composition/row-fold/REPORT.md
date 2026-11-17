# Literal Kummer row-fold arithmetic

The new Lean module proves the actual four successive modular adds/multiplies equal the decoded field row action and the remainder of its widened dot product. It then derives raw nonzero-output equivalence from canonical matrix entries, canonical input, a prime modulus p<=2^32, and the sixteen recorded matrix-square remainder equations with a canonical nonzero square scalar.

## Source correspondence

snapshot/walk-source.rs.txt lines14–21 implement mul as x*y%p and add as (x+y)%p in u64, with operation tallies. Lines48–50 use std::array::from_fn over row i, then (0..4).fold from0 over column j. RowFold.linear uses List.finRange4.foldl in exactly this order, mat i j times x j, starting UInt64 zero. It does not transpose. UInt64 semantics wrap, whereas Rust overflow may panic depending on build settings; the proved bounds exclude arithmetic overflow on this path in either setting. Tally side effects are not modeled: the complete call requires starting tally counters with room for sixteen increments each and normal thread-local execution. Compiler/binary fidelity remains separate.

Each product is <2^64 and each canonical accumulator sum is <2^33. Each reduction is canonical. The unreduced sum of four products is <2^66, hence fits u128 but need not fit u64. linear_dot states the exact natural dot remainder equals native-fold output; dot_fits128 certifies that a widened four-product accumulator has adequate headroom. This is an arithmetic equivalence, not a mechanical proof of the Rust compiler or a modeled u128 instruction implementation.

square_from_remainders converts the saved row-by-column matrix-square residue equalities to RepairClosure.squareScalar. native_repair_nonzero combines this with the prior field nonvanishing theorem and exact canonical word/nonzero correspondence. Primality supplies the field; no odd-characteristic assumption is used by this arithmetic theorem. Odd characteristic and chart/action geometry remain separate prerequisites of the historical Kummer construction. Zero inputs are retained: the result is an iff, so zero maps to zero. No claim that an xadd branch succeeds or computes its geometric intended sum is added.

## Fixed evidence and reuse

The prior fixed133 manifest is retained as fixed133-binding.json, with digest 8f6c33e483a6ceffe8009004470afd7699c4ccd330327804d496e7ea95966772. Its historical walk-source.rs pin matches the copied literal input. Comparator, WordComparator and Repair sources are copied byte-for-byte from their prior packets. This task did not replay the105 historical matrix actions, enumerate points or run scientific inputs. The prior matrix audit supplies the instantiation evidence; this packet closes its conditional row-fold arithmetic interface. Exact runtime loading/custody and published version identity must still match the bound data.

The retained chart note explicitly distinguishes C2 degree-two coordinates from C1 degree-one coordinates; this arithmetic result does not identify the two charts. Prior literature and native input notes remain historical sources.

## Proof gates and controls

Twelve new main theorems and ten fixed controls are compiled with the pinned Lean toolchain, followed by a separate22-declaration axiom readback. Controls cover nontrivial row orientation, transpose mismatch, zero persistence, noncanonical raw-nonzero failure, singular matrix failure, endpoint product/add at p=2^32, wraparound failure at p=2^32+1, widened-dot headroom, and a nontrivial repeated action. These are fixed arithmetic propositions, not curve samples or performance measurements.

Attempts1/2 preserve indentation/typeclass and tactic/import-name failures. Attempt3 preserves unsupported vector-notation control syntax. Corrections use explicit coordinate functions; no prior failure is erased. The first shell invocation lacked executable permission; the retained script is invoked with bash. Final gate rebuilds all six modules in dependency order before accepting controls/readbacks. Cached mathlib/toolchain binaries remain trusted; no separate lean4checker binary audit is claimed.

Producer source-first plan predates proof. Root independent review follows publication; this packet is not that review. No scientific run, cloud action, repository edit, new cost ratio or campaign extension occurred. Full target scope1260 and expired campaign deadline remain unchanged.
