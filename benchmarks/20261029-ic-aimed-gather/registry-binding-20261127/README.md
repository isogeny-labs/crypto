# Current-registry custody binding - 2026-11-27

Successor of `registry-binding-20261123`. The 2026-11-24 registry regeneration (`204b6fe234`, `f1bae2657e`, `9bd0884ddd`) grew `docs/curves/registry.json` from 22,417 to 33,873 rows (SHA-256 `2e927513f51802827ee918068a71462b86ef07601e8f236b987508b2eb2195f0`), which broke the gate's raw-data pin for this row.

The checker is the 2026-11-23 checker with the checked date and predecessor text updated, plus one narrowed comparison: the regeneration rewrote the target curve's provenance `sources` lists (record and representation level) but no other field. The target record (`icv1-f2m53-tm56619371-dac20a85`) is therefore compared exactly with only those `sources` lists removed, and the content-addressed `curve_uid` is asserted equal. The custody receipt records `complete_target_record_equal: false`, `target_record_equal_except_provenance_sources: true` and `curve_uid_equal: true`.

Positive replay passes 211 other raw pins and rebinds exactly one (the registry). `check_numeric_fidelity.py` confirms every inherited scientific JSON value equals the original report under exact decimal parsing; `test_numeric_fidelity.py` passes 7/7. This is input custody only: no benchmark, scalar replay or timing measurement was executed. `previous-manifest.json` retains the exact earlier `VALIDATION.json`. All original reports and earlier binding directories remain unchanged. [pcc:unverified]

Reproduce with the original registry bytes from Git:

    git show ddaba2819f4ee96139c4145fbed905db1c1db46e:docs/curves/registry.json > ORIGINAL_REGISTRY
    cargo run --release --locked --manifest-path benchmarks/20261029-ic-aimed-gather/registry-binding-20261127/Cargo.toml -- ORIGINAL_REGISTRY docs/curves/registry.json benchmarks/20261029-ic-aimed-gather/validator-report.json benchmarks/20261029-ic-aimed-gather benchmarks/20261029-ic-aimed-gather/registry-binding-20261127/validator-report.json benchmarks/20261029-ic-aimed-gather/registry-binding-20261127/source-equivalence.json
