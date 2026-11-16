# Corrected consumer admission source review

Reviewed source 086b4cd590d2e871ca9e119c389470c9a9232f94 on 2026-10-29,
against the previous reviewed implementation 8e87ba53e62679c861a40816c0485dab54cf482b.
This is a source review, separate from execution and archived replay evidence.

Read the full new json.rs, the admission correction contract, and all native
diffs in large_prime.rs, budget.rs, filter.rs and affected tests. The classifier
uses no retained JSON tree, validates EOF, rejects duplicate/escaped duplicate
schema keys and binds the full bytes to the manifest hash. New-v2 raw limits
precede loading; raw string/depth bounds precede decoding. A unit-returning
DeserializeSeed traversal charges the structural model before allocating the
Value tree. The later strict duplicate-key parser remains in use. The transcript
and one auxiliary/batch payload at a time share the modeled remaining allowance.
Legacy v1 routing retains the original strict parser and is tested separately.

The matrix reservation now includes column vectors even for zero rows. Native
filter scratch charges include completed rows and temporary source/pivot copies
before their allocations. No additional actionable issue was found in this
bounded correction. Prior allocation-after-check review findings are addressed
in source; the former validation remains historical evidence only.

Limits remain explicit: the 512 MiB accounting is a conservative named-storage
model, not a process RSS proof; parser scratch, allocator overhead and other
legacy public/base/config parsing are separately bounded/excluded. This review
does not establish hardware costs, scientific corpus completeness, fall degree,
or coverage of a sealed native-censor fixture.
