# solve4 pivot and swap source review

The prospective declarations faithfully describe first-nonzero list search and permutation of complete five-entry augmented rows in the retained native solve4 excerpt. No source correction is required for that stated scope.

**First nonzero search.** search_some proves zero prefix in the supplied list and a nonzero returned row; search_none characterizes list exhaustion. native_search_bound proves row eligibility and nonzero entry. Ascending/no-duplicate coverage of candidates versus Rust col..4 is not proved.

**Whole-row swap.** swapIndex is an involution including equal indices. swapRows permutes all five entries including RHS. Rowwise predicate equivalence supplies decoded equation-set equivalence for arbitrary decode; no primality or overflow assumptions needed for permutation alone.

**Prefix transfer.** selected_zero_prefix requires all eligible rows to have zero entries before col. It transfers that premise only; it does not establish the loop invariant. selected_pivot places the supplied nonzero entry on the diagonal.

**Controls.** Concrete matrix has eligible column values 0,2,3 at col1; selects row2 and moves RHS6 with coefficient2; prior row0 unchanged. Zero matrix exhausts search. All-ones and RHS-omission negatives reject unjustified prefix and coefficient-only swapping. These are prospective uncompiled controls.

## Boundaries

- Readback is prospective; no Lean elaboration or axiom audit occurred in this review.
- Native source binding is the retained concatenated excerpt, including the complete solve4 function; it is not a binding to the full native module, compiled binary or compiler.
- Candidate-list representation refinement, physical array mutation, canonical decode/zero correspondence, loop invariant propagation, counters and full returned-vector correctness remain open integrations.
- Already-reviewed normalization is reused, not reproved.

Nine declarations and eight controls have 17 prospective named axiom readbacks. All 11 provided source bindings matched read bytes. No compiler, payload, measurement or new mathematical acceptance was performed.
