# Registry binding for the historical aimed-gather validation, 2026-11-05 (b)

Successor of [`../registry-binding-20261105/`](../registry-binding-20261105/README.md).
It is written because PR #2826 registers the stride-renew study curves on top of the
registry #3484 merged: `docs/curves/registry.json` goes from 22,268 to 22,283 rows,
SHA-256 `90ce323c116bbf8728b9c4e64a9b6d1f947c9288e54cacd791088640612da750`.
The registry was built by `ic-semantic-metadata` run 42102540205 on merge
68422f7cca (artifact digest `sha256:1ce6ec92…`).

`verify_registry_binding.rs` is the 2026-11-01 checker with two text changes:
`checked_on` reads 2026-11-05, and the receipt names
`registry-binding-20261105` as its predecessor.

Everything else is unchanged:
- The original report (SHA-256 `505327cf…`) and registry (`53fe929b…`, 20,809 rows,
  commit `ddaba2819f`) are pinned.
- The target `icv1-f2m53-tm56619371-dac20a85` has exactly one complete record in both
  registries, and the two records are equal.
- All 211 other raw pins match.
- Only the registry binding changes; every scientific field is unchanged.

Output on 2026-11-05: `PASS: unique complete target record equal; 211 other
raw pins verified; exactly one registry binding revalidated; all scientific
fields unchanged`. Built with `serde_json 1.0.151` and `sha2 0.10.9`, the
versions in the repository's `Cargo.lock`. No benchmark, scalar replay or
timing measurement was run; no number, plot or PDF changes.
