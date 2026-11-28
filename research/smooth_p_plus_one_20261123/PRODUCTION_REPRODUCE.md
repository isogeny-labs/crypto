# Reproduce the bounded production adapter

Use an isolated checkout of source
`8be878c8739ed23e1c8c9c53684613642d751238` for the frozen full-base panels.
Materialize the whole repository, or all source paths listed in
`evidence/production_v1/production_source_manifest.json`; the closure
intentionally refuses missing inventory files and stale source pins.

From the repository root, with Rust 1.99.1 available:

```sh
export CARGO_TARGET_DIR=/tmp/rr5-production-build
cargo +1.99.1 build --locked --release \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --features production-controls

# Run each of these in a separate shell/process. The last positional path is
# unused by the production cofactor mode; no msolve installation is required.
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 127 4 unused cofactor)
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 8191 4 unused cofactor)
(ulimit -v 2097152; RAYON_NUM_THREADS=1 timeout --kill-after=5s 60 \
  "$CARGO_TARGET_DIR/release/dickson-four-summand" 8191 5 unused cofactor)
```

This produces JSONL with native diagnostics and explicit incomplete status.
An exit-zero panel is not a complete-query result. The frozen observations
contain thirteen resource refusals, retained in the report. [pcc:unverified]

The later source adds only the planted integration unit test to `production.rs`.
Run the final feature-enabled tests on that later revision with:

```sh
RAYON_NUM_THREADS=1 cargo +1.99.1 test --locked --release \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --features production-controls -- --test-threads=1
```

The original cofactor library and public example remain separate test targets
under `research/prime31_four_summand_20261122/native/Cargo.toml` (`--lib` and
`--example prime31_rr5_quartic`). They use the current reviewed source closure,
not a rewrite of its algorithms. The parent root library suite is distinct
and has not completed in this workspace.

The final raw-file manifest is `evidence/PRODUCTION_SHA256SUMS`. It complements,
and does not replace, the before-call source/executable identity. The
prose keyword linter flags a negated performance term in the frozen protocol's
limitations paragraph. The protocol is preserved at its original bytes;
that sentence limits the conclusion. The result report and PR prose have
separate lint receipts.

No EKS admission or external stream delivery is implied by these commands.
