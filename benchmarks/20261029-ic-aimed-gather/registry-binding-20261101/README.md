# Registry binding for the historical aimed-gather validation, 2026-11-01

Successor of [`../registry-binding-20261030/`](../registry-binding-20261030/README.md),
written because PR #2223 adds four `prime:` curves to `docs/curves/registry.json`
(20,844 → 20,848 rows, SHA-256 `6dce2c6f17bb1b3b68d078658a0eadb18de5f761da749f2876b0a44b16c2dedf`).

`verify_registry_binding.rs` is the 2026-10-30 checker with two changes:

- `checked_on` reads 2026-11-01 and the receipt names its predecessor.
- A raw input that the artifact store moved out of the checkout is checked
  against the SHA-256 its directory's `.artifacts.json` manifest records,
  instead of its bytes. The receipt counts these separately
  (`other_raw_input_hashes_verified_via_store_manifest`: 1, the
  `aa2_20261029/runs/isolation.jsonl` file).

Everything else is the same: the original report (SHA-256 `505327cf…`) and
registry (`53fe929b…`, 20,809 rows, commit `ddaba2819f`) are pinned; the
target `icv1-f2m53-tm56619371-dac20a85` has exactly one complete record in
both registries and the records are equal; all 211 other raw pins match;
only the registry binding changes, and every scientific field is unchanged.

Output on 2026-11-01: `PASS: unique complete target record equal; 211 other
raw pins verified; exactly one registry binding revalidated; all scientific
fields unchanged`. Built with `serde_json 1.0.151` and `sha2 0.10.9` (the
versions in the repository's `Cargo.lock`). No benchmark, scalar replay or
timing measurement was run; no number, plot or PDF changes.
