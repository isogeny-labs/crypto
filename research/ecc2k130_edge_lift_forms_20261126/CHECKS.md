# Reproduction and checks

Toolchain installed for this diagnostic: `rustc 1.99.1 (58cf8f9c3
2026-10-13)`, minimal official rustup profile plus rustfmt.

From `research/representation_leads_20261109/derive`:

```sh
cargo fmt -- --check
cargo test --release
cargo run --release -- edgecounts > ../../ecc2k130_edge_lift_forms_20261126/results/edgecounts.jsonl
sha256sum ../../ecc2k130_edge_lift_forms_20261126/results/edgecounts.jsonl
```

Observed test result: 11 passed, 0 failed. Raw-result SHA-256:

```text
9af7c84b4767330fbfc4c2c89c9567862b3e49f29fd0207bc12e27245f113cd7
```

Aggregate `results/summary.json` is produced with `jq -s` by grouping on
`[n,a,m,ell]`, selecting the three arms, and taking matched ratios. It records
72 cells, 216 rows, and `all_valid=true`.

Limit: these checks establish exact evaluation of planted toy systems and
presentation-size statistics only. They do not measure closure degree, F4
solving degree, relation yield, or ECC2K-130 cost.

The committed example `examples/identity_certificate_ecc2k130_edge_lift.rs`
issues and self-checks the substituted lift records. IDs and file hashes:

```text
IDC1h02b24a926e268f4d  identities/edge-ac.json  sha256 99ef4734c28fec00abe34f44246fe5107a260f4ddebe04a336f24b0096a02666
IDC1h943d3d2cc5f015d2  identities/edge-bc.json  sha256 8bf6539973992cd6e43fbede89027de3ae35e5fd8fb726c0808db0b4bb1de474
```

They were issued by the repository's exact `identity_certificate.rs` and
`sha256.rs` sources in a minimal scratch crate after the full root release build
was stopped during long optimisation. The committed root example is the normal
reproduction entry point; its focused CI compile/replay remains a PR check.
