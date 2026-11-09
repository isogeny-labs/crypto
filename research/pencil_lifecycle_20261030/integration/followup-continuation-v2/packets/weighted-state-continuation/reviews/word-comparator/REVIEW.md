# Independent native-word projective comparator review

Reviewer /root/monic_review. 2026-11-01. Prospective source-first plan before WordComparator proof-body access. Native walk mul is literal narrow UInt64 x*y%p, with a counted multiplication; same enumerates sixteen cross-product comparisons. No widened multiplier is substituted.

## Findings

product_bound correctly admits p<=2^32, not merely strict inequality: canonical operands are at most2^32-1 and their product is strictly below2^64. mul_decode removes UInt64 wrapping only with that bound. mul_canonical correctly proves remainder canonical even without product no-overflow; it does not itself claim a correct mathematical product. canonical_eq reflects word equality from field equality using canonical residues. sameB_iff converts both List.finRange all predicates to universal Fin4 quantification, reflects canonical modular-product equality, then applies the exact narrow multiplication decoding twice per pair. Thus this packet supplies the all-sixteen Boolean/field predicate bridge missing in the earlier comparator review.

nonzeroB_iff needs canonical coordinates and prime modulus and correctly reflects word nonzero into decoded field nonzero. nonzeroB_iff_ne_zero relates the Boolean scan to a genuinely nonzero raw vector without field assumptions. same_nonzero_iff_scale composes both bridges with the abstract comparator theorem, retaining canonicality, prime modulus, p<=2^32 and both nonzero inputs. Its scalar is nonzero in ZMod p; it does not pretend raw vectors are literally equal under an integer scale. No circular premise or missing direction found.

Native field-value fidelity is faithful_scoped to these literal formulas and sixteen-pair predicate. Actual Rust loop compilation, raw array/function representation, instrumentation counters and surrounding geometry remain separate. Zero vectors still match every vector in raw same; raw p-valued coordinates still invalidate nonzero decoding absent canonicality. The new theorem does not erase those domain constraints.

## Fresh checks

Comparator, WordComparator, WordControls and WordReadback rebuilt in a fresh isolated snapshot, with explicit pre/post local-input binding. Eight WordComparator declarations and ten producer controls passed eighteen named readbacks. Six reviewer controls also passed: multi-coordinate nonzero scaling, reversed scaling, a changed last slot, characteristic-two canonical word comparator, exact largest nonwrapping product at bound and first square that wraps above bound. Twenty-four named readbacks pass standard axiom allowlist. No reviewer failures. Producer failed attempts remain unchanged in their original directory.

The producer overflow false-positive control is materially informative: outside p<=2^32, distinct coordinate axes with entries2^32 can compare equal after wrapped products. Its natural-product modulo control separately demonstrates the missing nonzero cross-product. The endpoint p=2^32 fixture is an arithmetic bound control, not an instantiation of the prime-field theorem. The prime-modulus assumption belongs to the final scalar correspondence.

## Workflow and trust boundary

Separate reviewer, source-first plan, exact source inspection, fresh complete new-module replay, independent controls, named axiom audit and nonzero/pivot composition checks completed. No sibling review report read. Coordinator confirmed this review was dispatched against sealed private inputs before producer publication (T-1225); this sequencing deviation is retained. Local plan was not precommitted. Source-first access for these new modules is recorded; full prior-history blindness is not claimed. Imported helper binaries and cached Mathlib dependency unions remain the explicit trust boundary; no full clean dependency rebuild, independent kernel checker or multi-reviewer independence gate. Coordinator publication remains separate. No scientific runs, repository writes or official transitions.

Primary references: bound native walk source; new WordComparator and earlier Comparator theorem; standard bounded-word modular decoding and field projective pivot reasoning. No novelty claim or cost inference.
