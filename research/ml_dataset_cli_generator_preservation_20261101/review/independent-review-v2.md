# Independent review — corrected generating-set preservation

Reviewed exact corrected patch SHA256 `7a80c924d48f923f90c0d782ad41dfe1ccc9157ca77b3755d25bb9e595af5cff`. The superseded d26041c patch and v1 finding remain preserved.

The provenance blocker is closed in source. Replay captures native_basis_len before augmenting the generating set. The explicit native-plus-input contract preserves those raw native rows as an ordered prefix. Algebraic emission now takes only `basis[..native_basis_len]` and the matching `right_from_left[..native_basis_len]` certificate proofs. Thus existing source_basis_row indices and retained-native derivation origin still refer to actual raw native report rows. Appended incoming constraints remain in the full certified frame basis for branch selection and child specialization, but are not mislabeled as native algebraic targets.

Slicing is justified by the successful rowwise contract check, which requires the preserved list to equal the raw native list followed by missing input generators; legacy records leave basis unchanged. The two-way equality certificate's right_from_left proofs correspond to that ordered list. There is no new unchecked native algebraic inference.

The added regression uses all four retained inputs, verifies the preserved list is strictly longer than the raw report, and collects actual polynomial_consequence source_basis_row indices, requiring exactly0..raw_len. This directly guards the reported provenance defect. Earlier same-set specialization and mutation controls remain in place.

All other v1 observations remain applicable: explicit traced-traversal semantic change, original native report/counters preserved, independent two-way containment still required, legacy unknown behavior retained, existing caps unchanged, and clone/comparison/allocation overhead not represented as native instruction counts. No further blocking issue identified in this source review.

Recommendation: ready for isolated integration and focused compilation/tests under root ownership. This is source review only: no patch application, Rust build/test, native traversal or dataset replay was performed by this reviewer. Runtime completion and actual CLI coverage remain unverified.
