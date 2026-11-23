# Verification record — development and proof checks only

Baseline checkout: `50a51d94b1266c0f8afc94735391c0fba31cd317` on the local
feature branch. No research panel, timing comparison or remote job ran.

Native toolchain: Rust/Cargo 1.94.1. Set `CARGO_HOME=/workspace/n83-cargo`,
`RUSTUP_HOME=/workspace/n83-rustup`; cargo is `/workspace/n83-cargo/bin/cargo`.

```sh
cargo test --release --locked --offline --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
cargo clippy --locked --offline --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml -- -D warnings
cargo fmt --check --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
cargo build --release --locked --offline --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
```

All passed. Eight release tests, zero failures. The 625 linear-generator-pair
fixture and the exact degree-three artificial fixtures validate the audit
engine; they are not new elliptic factor-base degree results. The fixture
constructor checks rational lift membership and one exact S3/group sum on
the standard prime-order E/F17 example.

Lean command:

```sh
/workspace/prime-degree-lean/bin/lean research/prime_factor_base_degree_20261031/proof/TopDegree.lean
```

Lean 4.19.0, commit `6caaee842e94`, exited zero. A first version had an unused
hypothesis warning; the unnecessary hypothesis was removed and the final file
checked without warning. Axiom audit:

```text
DensePairTop.all_above: propext, Classical.choice, Quot.sound
DensePairTop.survives_at_top: propext, Classical.choice, Quot.sound
DensePairTop.survives_below: propext, Quot.sound
DensePairTop.first_full_degree: propext, Classical.choice, Quot.sound
```

The complete polynomial-ring bridge is written, not formally checked.
`lean4checker --fresh` is unavailable on this installation; it was not run.
No custom axioms, `sorry`, `admit`, or native evaluation appear in the proof.

Report: Typst 0.15.0 compiled two pages. Initial compile failed on an unescaped
less-than in prose; moved the inequality into math mode and rebuilt. Both PDF
pages were rendered and visually inspected. The diagram contains no performance
measurement or claimed degree win. Existing numerical dashboards were not changed.

The checkpoint gate was attempted and failed with no upload route. GitHub's
injected token remains invalid. No source push or PR creation succeeded; full
root-library validation and exact-head CI remain pending before publication.

| Source/artifact | SHA-256 |
| --- | --- |
| native/main.rs | `13434c7b5f58614651f1f292afed7aff9cc371925c726c12707be16a54fc842b` |
| native/pilot.rs | `afbc3b7cb652c8ea192e5b9161f519738b72d1d07002f6352fff337aabd3ade5` |
| proof/TopDegree.lean | `9f6853c4c27b17f479ef5283b6f682b20858d8e523378449d2002f64bdf0a7fe` |
| REPORT.pdf | `b7bf7e2a2c01c25ab40e850dc49e7082c8b412ffaa8d1a3806e4166878f1c411` |
