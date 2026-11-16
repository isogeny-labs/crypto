# Custody identity restore correction: source review

Accepted at source-review scope. Patch58db2d04dc908e3a1c7458be9008847fc9ab87bf405b71d7d6b706075ae7e970; base custody18351ef855cad9ae11d1a88fd723cf54140592d51fe3cac6a82eec84ca875180; proposed91e4890880c853a0a95f10ead983b06390f5c4fe65c5c8408affd93691d0bdb5. All manifest hashes verified against the reconciled donor.

verify_custody_run_identity is an exact extraction of the previous publication bound_run+three raw-file hashes+two producer identity comparisons. Publish retains compare_source_run before it and native verify_run afterwards. Restore invokes it after authenticated file reconstruction but before native verification/success. Remote restore invokes it on the reconstructed three-file preflight before completion/catalog and remaining bulk chunks. check_manifest/replay_members continue binding the three top-level digests to members and actual file bytes. Actor-bound validate_budget_run remains separate and still executes; no active-budget or S3 transport rule was changed.

This closes the identified producer-only manifest substitution path without confusing original producer identity with today's reader/publisher. The helper does not prove mathematical replay; final verify_run remains necessary. No source/destination overwrite/retry behavior changed. No new blocker found in this narrow correction.

No build/native execution/remote calls/production edits. Genuine copied four-cell local roundtrip and independently rehashed producer-only mutations remain the required runtime acceptance. Include all three top-level digest-only controls as regression checks; retain original acceptance bytes unchanged. The previous target module/CLI/caller integration dependencies still apply.
