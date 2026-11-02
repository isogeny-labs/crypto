# Original-bound Boolean replay and training export

The optional typed channel request binds bounded ordinary binary m3 Boolean production into the plan, worker and historical verifier. Plans with channels use v3; absent channels preserve the legacy representation. Requested unsupported cells remain explicit. SAT initial capture remains unsupported through this optional CLI channel.

Historical replay reconstructs the original equations and retained target, checks capture custody and sticky budget transitions, then independently verifies the polynomial identity. Export uses a private admitted projection to write `boolean-equation-features.jsonl` containing source equations and selector conventions, and `restricted/boolean-equation-records.jsonl` containing targets, proofs, assessments and provenance. Public features exclude solver outcomes and proof-dependent data. Final custody and post-encoding deadline checks precede release; unknown/expired/rejected identities never release positive public features. The original producer and historical replay caps are unchanged.

## Validation

The combined release dataset suite passed669 tests,0 failed,2 ignored on2026-11-02. Log SHA256 `4df9246a2c4609e92217ce7d52803b11794ab62405f1bbb6bee7d3afc1fc5875`; source and compile-time fixtures remained unchanged. Tests include genuine original16384 producer/historical replay and export projection for both twists/F4/F5, exhaustive Boolean arithmetic, requested-channel plan/config checks, final encoding deadline censoring and public/restricted projection controls.

The preceding replay/CLI run passed663 tests and failed one exhaustive test because its two test wire suffixes omitted the mandatory newline. The correction changes those test inputs only; the strict parser is unchanged. Failure log SHA256 `ad0fef0c562f3b150cf6cb04dcdb87dc98963123ce2633170e2fcfec483cd933` is preserved externally with S3 custody.

Actual CLI generate/verify/export acceptance and the full release library gate remain pending at this source checkpoint. Source review and library tests are not substitutes for those gates. The construction is the current ordinary N3 profile; full higher-arity/family/policy dataset acceptance remains separate. No exact first/last-fall-degree claim follows from a polynomial identity alone.

## Design sources

The implementation follows the canonical BQI1 grammar and independent arithmetic checker in `src/ml_dataset/boolean_identity.rs`, original-source and budget contracts in the ordinary binary m3 adapter, and the existing structural export allowlists. The independent export correction review and exact validation receipt are in `research/ml_dataset_20261028/boolean-replay-export-20261102/`.
