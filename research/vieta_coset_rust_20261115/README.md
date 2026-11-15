# Native Vieta / smooth-coset decomposition

The Rust library module `src/cryptanalysis/vieta_coset.rs` implements the
non-interpolating two-point extractor discussed in the user request. It is
exported from `crypto_lib::cryptanalysis::vieta_coset`. The small harness here
compiles **that exact source**, not a second solver. Producer checks and claims
remain [pcc:unverified] pending an independent signed verdict.

```sh
cargo test --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml
cargo run --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml \
  --bin vieta-coset-lab -- check p224 128 2 120
cargo run --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml \
  --bin vieta-coset-lab -- check word17 8 2 30 three > /tmp/vieta-check.jsonl
cargo run --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml \
  --bin vieta-coset-replay -- /tmp/vieta-check.jsonl
```

`check` emits no timings. `bench` has the same arguments and additionally needs
an admitted launcher setting `VIETA_ISOLATED_RUN=1`; that variable is an intent
guard, **not isolation certification**. Do not set it to bypass the protocol.
Every stage has operation/time limits; failures emit rows and a nonzero status.
`vieta-coset-analyze FILE...` summarizes retained rows with descriptive quartiles.
The separate replay binary recomputes two-term answers via BigUint scalar
subtraction. It does not certify three-term rows or confer independent standing.

Development coverage includes exhaustive rational targets on three small curves,
two coset representatives (not assumed distinct cosets), identity/two-torsion/diagonal cases, word/wide agreement, P-224 known
answers, and explicit invalid-input and root-budget rejection. Larger development
checks use m=4096 on a 62-bit prime and m=1024 on P-224; they establish output
agreement, not performance scaling. See [PROTOCOL.md](PROTOCOL.md) for exact
scope, cost boundaries, pending scale matrix and launch blockers.

The arithmetic backends here are a counted generic reference: u128-reduced u64
and native Rust BigUint. Dense polynomial operations use schoolbook arithmetic.
The library's optimized Montgomery/binary-GCD backends and an external polynomial
backend still need matched measurements before a fastest-method conclusion.
The m=2^d lift does not bound the elimination or last-fall degree by two.

No performance experiment has launched. EKS checkpoint credentials and live
budget/capacity access are missing, and the existing ecbench image builder does
not yet package this component binary. `protocol.json` remains a draft rather
than claiming a freeze without the required preflight. The original larger-scale
measurement requirement remains open.
