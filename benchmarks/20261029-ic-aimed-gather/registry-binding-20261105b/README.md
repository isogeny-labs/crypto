# Registry binding for the historical aimed-gather validation, 2026-11-05 (b)

Successor of [`../registry-binding-20261101/`](../registry-binding-20261101/README.md),
written because PR #2826 registers the stride-renew study curves in
`docs/curves/registry.json` (20,848 → 22,277 rows, SHA-256
`f6e63f486cbb824f8867f0fdf0d7de228ec9780a6b0384215bc673b37b8f6a7d`).
The suffix `b` keeps it apart from the binding PR #3484 writes on the same
day for its own registry; whichever of the two merges second rebinds again
against the merged registry.

`verify_registry_binding.rs` is the 2026-11-01 checker with two text changes:
`checked_on` reads 2026-11-05 and the receipt names
`registry-binding-20261101` as its predecessor.

Everything else is the same: the original report (SHA-256 `505327cf…`) and
registry (`53fe929b…`, 20,809 rows, commit `ddaba2819f`) are pinned; the
target `icv1-f2m53-tm56619371-dac20a85` has exactly one complete record in
both registries and the records are equal; all 211 other raw pins match (one,
`aa2_20261029/runs/isolation.jsonl`, through the artifact-store manifest);
only the registry binding changes, and every scientific field is unchanged.

Output on 2026-11-05: `PASS: unique complete target record equal; 211 other
raw pins verified; exactly one registry binding revalidated; all scientific
fields unchanged`. Built with `serde_json 1.0.151` and `sha2 0.10.9` (the
versions in the repository's `Cargo.lock`). No benchmark, scalar replay or
timing measurement was run; no number, plot or PDF changes.
