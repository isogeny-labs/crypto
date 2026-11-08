# Registry binding for the historical aimed-gather validation, 2026-11-07

Successor of [`../registry-binding-20261105/`](../registry-binding-20261105/README.md).
It was written because PR #3651 registers the ten prime medium-tier rho curves
(`research/prime_medium_tier_rho_20261107/curves.json`) in
`docs/curves/registry.json`. The registry grows from 22,268 to 22,278 rows,
SHA-256 `33a16da0548b7b0c3482b3ba7198500a0285271794f63e9c46e10f9a02186e4f`.

`verify_registry_binding.rs` is the 2026-11-05 checker with two text changes.
`checked_on` now reads 2026-11-07, and the receipt names
`registry-binding-20261105` as its predecessor.

Everything else is unchanged. The original report (SHA-256 `505327cf…`) and
registry (`53fe929b…`, 20,809 rows, commit `ddaba2819f`) stay pinned. The
target `icv1-f2m53-tm56619371-dac20a85` has exactly one complete record in
both registries, and the two records are equal. All 211 other raw pins match.
Only the registry binding changes, and every scientific field is unchanged.

The output on 2026-11-07 was `PASS: unique complete target record equal; 211
other raw pins verified; exactly one registry binding revalidated; all
scientific fields unchanged`. The checker was built with `serde_json 1.0.151`
and `sha2 0.10.9`. It was run as follows, with the old registry taken from
`git show ddaba2819f:docs/curves/registry.json`:

    verify_registry_binding OLD_REGISTRY docs/curves/registry.json \
        benchmarks/20261029-ic-aimed-gather/validator-report.json \
        benchmarks/20261029-ic-aimed-gather \
        registry-binding-20261107/validator-report.json \
        registry-binding-20261107/source-equivalence.json

No benchmark, scalar replay or timing measurement was run. No number, plot or
PDF changes.
