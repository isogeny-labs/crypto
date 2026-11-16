# Original auxiliary Boolean descent replay

This component consumes an authenticated `provenance::CheckedMap` and reuses the production `boolean_equations::expand_owned` algorithm. It retains that authentic source and every expansion prefix on all exits. Only a complete local expansion, exact captured raw lexicographic generator equality, and exact captured normalized degree-then-lex generator equality set `Audit::original_descent_verified()` true. Generator order and zero generators are preserved. Domain clauses use the actual sparse expander: positive literal unsatisfaction is 1+x, negative is x, and DIMACS wire w maps to variable w-1.

Success establishes the authenticated original pattern's auxiliary Boolean gate equations. It does not establish a signed PDP relation, a negative result, or all-pattern exhaustion. Authentic CheckedMap construction, capsule/map binding, and latest-snapshot selection remain central caller responsibilities. Producer charge counters and normalization cursors are not independently certified.

## API and integration

`verify(map: CheckedMap, root: &wire::Value, limits: Limits, deadline: Instant, owner: &mut impl wire::Admission) -> Audit` consumes the central frame's map after all snapshot binding checks. `Limits` contains expansion limits plus total work/bytes. `Audit::source()` supplies the same retained map for subsequent revalidation/projection; `Audit::expansion()` exposes retained expansion diagnostics. Public result, charge and compared_generators retain local progress.

Both production expansion quotes and comparison/index allocations pass through the same caller admission and absolute deadline. All temporary order vectors are retained in the audit. Local ceilings do not create owner credit. A caller reservation accepted before its postcheck fails remains charged; if a caller itself records a failed reservation, its cumulative ledger remains authoritative. No refunds or retries are supplied.

Missing captured expansion, valid producer resource-stop statuses, replay caps and owner/deadline failures are inconclusive. Malformed fields, invalid-wire reports, conflicting success metadata, and equality mismatches are invalid. Producer Err(Unknown(...)) requires a known non-Done stage. Ok(()) requires Done, null owner_failure, and empty partial/scratch. A later search failure does not erase complete mathematical descent.

## Validation

23 focused Rust tests pass: 16 new descent controls and 7 existing actual sparse-expansion controls. The new controls exercise AND/XOR/unit/impossible/domain generators, an explicit zero generator, lex-to-degreelex differences, raw/normalized mutations, omitted/extra/reordered/zero-dropped generators, incomplete/corrupt metadata, missing expansion, cap/refusal/deadline retention, accepted reservation followed by postcheck refusal, and total allocation cap before reservation.

The harness uses the actual production checked-equation wrapper and sparse kernel. It omits unrelated child modules and their integration tests, and supplies a structural CheckedMap fixture; it does not establish original map authentication. Production code accepts only the actual CheckedMap type at integration. Source hashes and exact donor transformations are preserved in SOURCE_BINDINGS.json and prepare_tests.py. No full Cargo build, scientific generation, cloud job, repository mutation, or published revision change occurred.

The first focused compilation failed because removing an unrelated affine child left a dangling documentation comment. compile.initial.log preserves that failure. The test preparation script was corrected; component source semantics were unaffected. Initial 18-, 20-, and 22-pass logs remain alongside final 23-pass log. Replay: `rustc +1.98 --edition=2021 --test tests.rs -o tests && ./tests`.
