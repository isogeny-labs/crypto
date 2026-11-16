# Independent source review — generating-set preservation v1

Reviewed patch SHA256 `d26041c04e6c70be618d9c2efe6436f82f0b36366b58a0af3b60c577c3bc1805` and the actual proposed Rust files against existing producer, replay and algebraic-target code. No patch application, compilation, native traversal, dataset replay or fixture modification was performed.

## Blocking finding: native-target provenance

`prime_chain_prefix.rs` passes the entire augmented `basis` to `algebraic_targets::Node.basis` and all equality proofs to the algebraic emitter. Existing `algebraic_targets.rs:15` declares derivation origin `separately_searched_generator_identity_for_retained_native_basis_row`; records at lines174-179 identify `source_basis_row` and use that native origin. On all four retained examples, the actual raw native basis has three quadratic rows, while the augmented list adds the two incoming quartics. The resulting last two labels would identify nonexistent native basis rows3 and4. These are valid original-generator consequences but their retained-native provenance would be false.

Correct before integration: retain the raw native row count and emit only that leading slice plus the corresponding right-from-left certificate proofs, while the full preserved set still drives branch and child replay; alternatively introduce an explicit separate generating-set origin/index contract and controls. Test actual emitted target/index/origin bindings, including proving no appended quartic receives a native row identity. Producer was notified.

## Checked construction and consistency

The producer creates the list only for traced traversal: raw native rows in original order, then each nonempty incoming row not exactly present. It serializes the explicit list, scans it for branch selection, and specializes that same list for recursion. The untraced branch remains the old raw native basis. This deliberately changes traced traversal behavior, rather than claiming a passive instrumentation-only change. Raw F4Report fields remain untouched.

Independent replay parses and bounds the explicit list, reconstructs the native-plus-incoming contract, checks rowwise equality with term-order-independent coefficient maps, then invokes existing two-way ideal containment certification. Inclusion of the original generators proves one containment by construction; the opposite containment is still checked, never assumed. The certified full list is retained as frame.basis and feeds independent branch selection and child specialization. The existing inconsistency/timeout flags remain checked. Legacy streams keep their prior path and bounded unknown behavior.

The raw report mismatch test (replacing raw basis with empty) is meaningful for these fixtures: equivalent_rows is ordered by rows, so the reconstructed input-first expected list differs from preserved native-first order even though unordered ideals would agree.

## Quotas, costs and compilation scope

No certificate, row, event or stream cap is raised. The parser applies existing bounds to the additional list and the equality/algebraic channel uses existing cumulative ledgers. Producer cloning, comparisons and enlarged child specialization add real allocation/wall work without new preflight allocation quotes; README discloses that scope. Native field_ops remains native-only and must not be advertised as total traversal instruction count. Existing sink/supervisor bounds still censor or terminate larger events/work; this is not proof of a stronger memory bound.

No obvious Rust type/borrow mismatch was found by source inspection. All other VisitEvent::NativeResult matches in the repository use `..`, so adding the field does not require exhaustive-pattern edits. The include_str path resolves from src/ml_dataset to the newly supplied research fixture. These observations do not replace compilation and the focused tests remain unrun.

The regression draft covers saved legacy unknown roots, actual same-input traced root/first-child construction, retained quartics, independent equality, one admitted branch, missing/changed generators, raw-report mismatch and changed child specialization. It should additionally enforce native algebraic target provenance as above. Full fixed-case completion and CLI coverage remain future validation, not established by these root tests.

Verdict: hold this exact patch for the provenance correction; no other blocking issue identified in inspected source scope.
