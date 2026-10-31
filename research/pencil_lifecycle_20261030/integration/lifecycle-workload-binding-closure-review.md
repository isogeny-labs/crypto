# Additive source closure: workload binding and resource correction

Read-only review 2026-10-30. No source edits, builds, tests, Git operations or scientific execution.

The workload-binding finding in lifecycle-child-dispatch-review.md is closed in the proposed real-child source path at record.rs SHA e1d32fdb0b2e907a3d8da7e6e134c9900c1a3d91aab62a3e4a161380e118a6f9. validate_workload serializes the rebuilt Workload through serde_json::Value and hashes the bytes, matching runner prepare_lifecycle_child. The call is immediately after the existing expected_workload_id check and before method resolution or measurement/solver hooks. Its serialization-only fixture establishes equality and rejects target_seed mutation without constructing or solving a workload. The generic fake-callback helper intentionally remains metadata-only. The early disabled child gate remains, so this is prospective source closure, not exercised execution or complete activation.

Author self-check only: diagnostic_resources.rs SHA e25bdaa342ed7351eef92299ac576fdba5aeea946821be7bd164b6e5274e6c41 now calls external exit minus payload start closed_payload_wall_ns. Emitted total_active_worker_ns remains null, both before and after closure. Prefix and journal bytes are length-checked and duplicate-key checked before serde, with64KiB/8MiB caps. Authored assertions cover active-null, duplicate keys, oversized data and external binding/order failures. This is not an independent audit of the author's resource module or a passing software result.

## Proposed filtered software verification commands (not run here)

Run from repository root after the upstream compilation dependency is integrated. These are source-fixture checks, not cohort commands. The diagnostic_ substring includes diagnostic modules and explicitly named diagnostic test modules; it excludes unrelated full-library solver tests. The two exact pencil tests exercise fake sink dispatch/ownership and panic cleanup only.

```sh
cargo test --release --lib -j 2 diagnostic_ -- --test-threads=1
cargo test --release --lib -j 2 cryptanalysis::ecbench::three_leads::pencil::tests::scoped_prefix_fake_dispatch_and_shared_meter -- --exact --test-threads=1
cargo test --release --lib -j 2 cryptanalysis::ecbench::three_leads::pencil::tests::scoped_prefix_panic_clears_ownership -- --exact --test-threads=1
```

Retain the exact command and filtered counts; a zero-test result is not verification. No empirical timing, scientific admission, complete-cost or root build qualification follows from this read-only review.
