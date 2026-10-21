# Twist routes 1–3: experimental program

[Experiment designs](EXPERIMENTS.md) and [machine-readable matrix](experiments.json) cover E00–E17. [Dependency diagram](PROGRAM.svg). The three existing backlog cards carry route-specific protocols; experiments remain unperformed unless a linked receipt says otherwise.

The small [Rust preflight](tools/) checks five polynomial identities with the repository's certificate implementation and exact coefficient expansion. It is not a point-decomposition solver, a subgroup-transfer checker, an independent-host replay or a performance comparison.

Reproduction and actual run receipts are recorded in the result report after the source/protocol freeze. No remote job or scheduled campaign is started by this program.

## Completed preflight

[E00 result](reports/RESULTS.md): five identities passed exact-coefficient and evaluated-certificate checks; five false variants were rejected; 22 supporting tests passed. [Receipt](reports/run-20261021T144610Z/RECEIPT.json). This leaves E01–E17 unperformed.

Reproduce with Rust 1.90.0:

```sh
cargo test --release --locked --manifest-path research/twist_routes_1_3_20261021/tools/Cargo.toml
cargo build --release --locked --manifest-path research/twist_routes_1_3_20261021/tools/Cargo.toml
research/twist_routes_1_3_20261021/tools/target/release/twist-route-preflight check research/twist_routes_1_3_20261021/reports/run-20261021T144610Z/certificates
bash research/twist_routes_1_3_20261021/build-reports.sh
```

The small crate includes existing repository certificate/hash modules by path. Materialize src/cryptanalysis/identity_certificate.rs, src/hash/sha256.rs and docs/identity-certificates/schoenhage-10mult.json in a sparse checkout before building; these imports are not vendored copies. The full repository library suite was not run in this focused review checkout.
