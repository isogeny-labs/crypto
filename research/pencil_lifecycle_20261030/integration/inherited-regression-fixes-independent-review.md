# Read-only review of inherited regression fixes

Scope: source inspection only. No repository changes, builds, tests, solver calls, or cloud operations. T965 restore.rs was not opened or changed. PR2986 boundary repair remains its owner dependency.

## Findings

1. Restore sat_decompose_chained_s3_with_domain admission from 5..=6 to 4..=6. The existing chained_s3_four_to_six_summands_lift_on_the_original_curve fixture explicitly tests m=4,5,6, and the report admission guard permits 4..=6. Keep this wrapper range distinct from the wider API.

2. Change shared sat_decompose_chained_s3_checked guard from 4..=6 to 2..=9 so the public any_m path can reach its advertised range. Preserve all domain, cap, field-equation, and full group-lift checks. Update stale comments mentioning legacy5..6. The current encoder independently permits2..15; do not claim9 is its physical mask bound and do not broaden this adapter to15 as part of this fix. This is a source-consistency conclusion, not new mathematical validation.

3. In BudgetedS3Backend::head, retain the successful HEAD result. Only for Some(meta) with meta.sha256=None, call the existing get_body with the same bucket/key/expected/scope and sink. Return Some(expected.sha256) only after get_body succeeds. get_body enforces registered length, body SHA, request/byte charging, a300-second body timeout, and a durable successful attempt ticket. Every error must propagate. None remains absence; existing digestSome remains unchanged; HEAD403 already performs its own charged GET and returns digestSome, so must not cause a second fallback GET. Store::push remains unchanged and continues requiring exact size and digestSome. Expected registration alone is not evidence of actual bytes.

4. resumed_publication_rejects_same_size_remote_corruption_without_overwrite currently expects the late round-trip error. DirBackend::head now hashes the actual file, and storage publication rejects an earlier existing size/digest mismatch. Change the assertion to the mismatch diagnostic (and preferably refusing overwrite); preserve the assertion that remote bytes remain ghijkl. This acknowledges earlier detection and does not weaken corruption rejection.

## Exact inspected source hashes

- `src/cryptanalysis/koblitz_index_calculus.rs`: `91101537b9f8614c96afbfcb72d3e8fd1a856c26f38b2358080f6a7ce072b462`
- `src/cryptanalysis/binary_semaev_chain_sat.rs`: `24e0d5e443f9754d2cccde44e1f45fcf3968831321f8e182136f0d99da4be352`
- `src/ml_dataset/s3_budget.rs`: `6ba7025d74967a1d9b83115906b86e09f6d373f221a9b277700819a1aeec59bc`
- `src/ml_dataset/storage.rs`: `981f6cb3f652281005facfd2c9920df4c35c4ef3a787a307b93b6e1da1d29c35`
- `src/store/mod.rs`: `935f66a0cad17934ad0f3ac6fb0b14dd8de05d981af9a68387adfdae53ad7ecb`
- `src/store/backend.rs`: `42317f4faf315d80ba78b0e73a242f9e6be56811ad3651550f5eaaf673593c75`
