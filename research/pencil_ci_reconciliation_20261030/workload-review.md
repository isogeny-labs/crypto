# T937: planted-only fixture reconciliation proposal

Read-only source inspection against commit71d6c3aca (PR2957 merge). The shorter71d6 was ambiguous with tree/blob objects; inspection used the unique commit prefix. Exact workload.rsSHA c763e579805c72f11da932c430eddfedc44592f2a5d79497547eb2eaea4157bd.

Unapplied patch: /tmp/t937-planted-only-fixture.patch
SHA2564eea40d959873154272a4439c3585a004c1b322ca44eed8f10cd0f88d07f0194.

One test-only hunk changes contradictory fixture expectations, not production semantics:
- All three instantiated variants must return None from the currently blocked hash_to_subgroup route.
- Torsion subgroup checks use the existing known synthetic point2G instead of a public hash target.
- Preserve deterministic repeat check using mul_generator_hex(2), x/y decoding, explicit on-curve assertion and r-annihilation for both torsion field types.
- Preserve serde roundtrip, constructor/metadata checks, generator equality,0G/rG identity checks,2G nonidentity check, and invalid p/m rejection.
- Translation retains its existing generator checks and unavailable public-law assertion; no new curve implementation or constructor is introduced.

This deliberately does NOT claim public-target functionality tested. The replaced expectation described a conflicting older policy; production's five-variant guard remains unchanged. The original test source is recoverable from the pinned commit. No special-case disabling, test ignore, weakened subgroup assertion or changed arithmetic is proposed.

Apply separately from the orphan four-line syntax repair; neither patch touches methods.rs or Cargo.lock (T944 territory). The unchanged orphan syntax defect still exists in the private fixture-only candidate until that separate repair is combined. No build, parser, test, solver or working-tree edit was performed for this task.

Suggested focused software test after root combines repairs: cargo test --release --lib extension_variants_have_distinct_constructors_and_subgroup_arithmetic -- --test-threads=1 . This is a recommendation, not a passed result.
