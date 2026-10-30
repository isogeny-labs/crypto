# T933 initial independent read-only review

2026-10-30 /root/four_lead_state. No execution, tests, edits, Git or cloud actions. Reviewed diagnostics.rs SHA9f7fef129d5a3e6b9dd8850ae2f70a6874a1685822a9952c70cdcf6d8a6b17c3; runner.rs5279ec895fd1f5c8a42d14c2280ce21bfc2516bb8fe99b94e11e88ad4d23bcf2; record.rse56fc0bfdb288c6b5e8ac85d5ce8af015dd05d8008b2bb956250140307c09721. Working-tree snapshot, not final corrected version.

Actual delivered scope: dormant bounded diagnostic ingestion and attachment plumbing, reserved Context input explicitly rejected before workload construction, plus an active shared timeout/interruption acceptance helper. Runner constructs diagnostics=None. This is not an enabled progress producer, admitted instrument or completed timeout integration.

Typed field/arithmetic/symbolic keys match isolated producer5415467 and its Counts/ArithmeticCounts/meter::Snapshot. PhaseTotals and nullable allocation fields match Region. Integer counters remain integer typed; u128 destination widens producer u64 counters without narrowing. Snapshot unknown fields and duplicate JSON keys are rejected. Reads cap65536bytes per entry and257 directory entries; contiguous JSON sequences are checked; symlink leaf and final files are rejected, with Linux O_NOFOLLOW. Trusted stable parent/no live child are explicit preconditions, not an adversarial filesystem guarantee.

Concrete findings requiring correction or narrowed claims:

1. monotonic checks only top-level field/arithmetic/symbolic. Phase cumulative fields, elapsed values and prior_io can regress or become internally impossible without rejection. Marking such rows structurally_valid_prefix exceeds strict producer-history validation.
2. Boundary checks permit null walk for walk_generation/sieve/processing, arbitrary completed_candidates, backwards walk indices, repeated setup boundaries and rows after solver_returning. Validate the actual producer state transitions and fixed milestone set, allowing a genuinely interrupted prefix rather than inventing completion.
3. A lone temporary name can have an unrelated sequence; producer permits only current incomplete sequence or same-sequence published/temp pair after unlink failure. Bind that rule without interpreting temporary content as accepted data.
4. ingest validates session_id/seq/algorithm_seed decimal string/binary_sha256 against actual Record field names and hashes canonicalized workload/method objects. Record has no protocol/source/attempt fields; those remain supplied Context assertions and require trusted external admission. It also does not verify Record.schema/record_id seal itself. Caller currently supplies freshly sealed bytes, so document trusted-precondition scope rather than claim standalone record authenticity.
5. synthetic_reaped_stop_excludes_child_without_solver builds report=None and calls the helper, not assemble. It demonstrates stop filtering but does not exercise a late successful mathematical report/cost through actual assembly. Add a synthetic assembly-level fixture or retain this limitation. Actual assemble does call accepted_after_stop, so wiring exists statically.

The actual runner attachment is emitted after record sealing and sync; it cannot import accepted result/cost and includes diagnostic ingestion wall separately. Future production accounting still needs attachment serialization/publication and full outer interval, identity construction, private leaf creation/custody, schema/checkpoint integration and execution admission. No scientific status or cost claim follows from these helpers.

Findings were sent directly to root and implementer pencil_diagnostics_design. No validation success is inferred from build progress.
