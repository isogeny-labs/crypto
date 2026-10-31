# Complexity literature retrieval

Native Rust retrieval over the complete, dated Hexagon `cs.CC` snapshot in
[`research/hexagon_cscc_20261030`](../../research/hexagon_cscc_20261030/README.md).
This tool returns source claims, scope, evidence status and implementation
touchpoints. It runs no downloaded code and never treats retrieval as proof.

```sh
cargo test --locked --manifest-path tools/complexity-catalog/Cargo.toml
cargo run --locked --manifest-path tools/complexity-catalog/Cargo.toml -- --check
cargo run --locked --manifest-path tools/complexity-catalog/Cargo.toml -- factoring
cargo run --locked --manifest-path tools/complexity-catalog/Cargo.toml -- certificate degree
```

With no query it returns all papers. Query words are matched case-insensitively
with AND semantics. The checked inventory contains 22 versioned works,
including cross-listings and the current v2 entries, as retrieved on
2026-10-30. Future additions require a new explicit source review; no watcher
or schedule is installed.

Tests preserve two finite counterexample controls from the reviewed papers:
the product-zero/no-common-zero three-generator construction and overlapping
affine-subspace deletions. Those tests check their stated finite properties;
they do not independently certify the papers' universal lower bounds.
