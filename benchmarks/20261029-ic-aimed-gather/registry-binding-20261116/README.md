# Current-registry custody binding - 2026-11-16

Restore the exact modern registry from commit730e036d50^1:22,300 entries, SHA256c0e1027ac8023409e7ce072428b4a978cd4d81cbcf7101dc7e728b7abb8f0134. The temporary rollback in PR4394 removed entries used by current workloads. Preserve the original reports and source data; this prospective binding changes only the registry pin and custody metadata.

The original20,809-entry registry fromddaba2819f4ee96139c4145fbed905db1c1db46e has one complete target record, equal to the modern registry's one target record. Exact-integer JSON parsing gives target digest82041b1482d2da798d48a78a618aed7797b08755bcb525b5c6f136f304dfe637. An independent Decimal parse confirms every inherited scientific value equals the original report after removing the new custody receipt and restoring its original registry pin.

The Rust checker's control flow is the November7 checker, with checked date and predecessor text updated. Cargo.lock pins dependencies. Enable arbitrary_precision/float_roundtrip for serde_json so that serialization preserves all scientific numeric values: default parsing shifted some values by1 ULP in the first rejected successor. The rejected default-precision control remains a private diagnostic, never an accepted report.

Positive replay passes211 other raw pins:210 actual byte hashes locally and1 stored isolation-file manifest hash. That last raw file is not locally available; full CI hydrates and hashes the evidence. This is input custody; no new benchmark, scalar replay, timing or scientific validation result. previous-manifest.json retains the exact earlier live manifest. All original reports and earlier binding directories remain unchanged. [pcc:unverified]

Reproduce with the original registry bytes from Git and:

    cargo run --release --locked --manifest-path benchmarks/20261029-ic-aimed-gather/registry-binding-20261116/Cargo.toml -- ORIGINAL_REGISTRY docs/curves/registry.json benchmarks/20261029-ic-aimed-gather/validator-report.json benchmarks/20261029-ic-aimed-gather benchmarks/20261029-ic-aimed-gather/registry-binding-20261116/validator-report.json benchmarks/20261029-ic-aimed-gather/registry-binding-20261116/source-equivalence.json
