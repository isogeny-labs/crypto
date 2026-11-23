# Qualification receipt — hypothesis search, 23 November 2026

All entries are producer observations and remain [pcc:unverified].
No F4 solve, generated target, natural relation search or performance panel
was executed. The standalone crate does not build the root library or reuse
its shared target directory.

## Native identity helper

Working directory: repository root. Toolchain and source bindings are in
`toolchain.txt` and `source.sha256`. The build uses the checked-in independent
manifest and lockfile, with `CARGO_TARGET_DIR=/tmp/ic-hypotheses-20261123/target`.

```
cargo test --release --offline --manifest-path research/extension_ic_graph_20261115/hypotheses-20261123/native/Cargo.toml
cargo clippy --release --offline --locked --manifest-path research/extension_ic_graph_20261115/hypotheses-20261123/native/Cargo.toml --all-targets -- -D warnings
cargo run --release --offline --locked --manifest-path research/extension_ic_graph_20261115/hypotheses-20261123/native/Cargo.toml -- issue
cargo run --release --offline --locked --manifest-path research/extension_ic_graph_20261115/hypotheses-20261123/native/Cargo.toml -- check research/extension_ic_graph_20261115/hypotheses-20261123/norm-quotient.identity.json
```

- Initial test compile: exit 101, incorrect caller constructor names and
  missing `hex` dependency. `native-tests-initial-failed.log` is retained.
- Corrected test build: exit 0, 11 tests, zero failures, zero ignored. The
  nine existing checker controls are reused; two new controls compare exact
  integer coefficients and reject the missing-cross-term identity.
- Scoped strict Clippy: exit 0.
- Issue: exit 0. Check: exit 0. Source and result certificate are separate
  files; original scientific trials and their outputs were untouched.
- `IDC1h3a70da000d31ad8a`: 32 sealed evaluations plus 16 extra producer-supplied
  evaluations. Exact integer expansion leaves three nonzero monomials and
  equal coefficients. Scope is the norm polynomial identity, with no
  dimension, parametrization, yield or speed conclusion.
- Standalone executable is LOCAL at
  `/tmp/ic-hypotheses-20261123/target/release/graph-ic-hypothesis-identity`,
  SHA-256 `ba6b791b80752214877cca1feef9663626cffa50781c68df395baa72f96cf015`.
  This is a post-build binding, not a frozen benchmark pre-registration.

## Report and publication checks

- Native Graphviz SVG and Typst 0.15.1 PDF, two A4 pages. Final PDF pages were
  rendered at 150 dpi and both inspected after PNG quantization. Equations,
  candidate labels, certificate ID, sources and scope are legible. Page-one
  and page-two previews and the diagram preview are each below 64 KiB.
- Full hypotheses and rendered source pass `pcc lint-prose`.
- Existing canonical scoreboards and measured figures are unchanged because
  no solver measurement or ranking was added.
- No fresh full-root build was requested for these additive standalone files;
  the existing root syntax and historical benchmark-record failures remain
  recorded separately. No claim that full-library CI passes is made.
- KB wrapper and Conductor are absent (127); atlas connected-read decoding
  failed. Prior notes plus the primary-paper/source review are the bounded
  fallback. Paper hashes below identify LOCAL downloads, not Git artifacts.

## Primary-paper custody

- Joux–Vitse 2012: `7ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38`.
- Larsson et al. CVPR 2017: `2e97d90d03f40e84763f4acc48c47638c13d5e57b8876846f5b4ef1f9ffea7b2`.
- Larsson et al. arXiv:1803.04360v1: `579716aae0658589e1f688bfd64986b5bc346788c9b911e121c0577bc5c1a999`.

Exact public links and the sections consulted are in `../README.md`.
