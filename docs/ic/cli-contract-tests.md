# IC CLI contract tests

The normal Cargo integration suites `ic_framework`, `ic_progress` and
`ic_tensor` remain enabled and use Cargo's freshly built `CARGO_BIN_EXE_ic`.

The dedicated IC workflow also exposes those same suites in the selected-bin
harness through the explicit `ic-cli-contracts` feature. This avoids building
unrelated binaries while keeping every existing assertion in the CI run:

```sh
cargo build --release --features ic-cli-contracts --bin ic
IC_TEST_BINARY="$PWD/target/release/ic" \
  cargo test --release --features ic-cli-contracts --bin ic
```

Use the same feature for both commands to reuse the library build. The feature
adds test modules only; it changes no production algorithm. Ordinary
`cargo test --bin ic` retains its normal unit tests without requiring an
external executable. Opting into the contracts requires the explicit freshly
built production executable; the helper fails clearly if neither Cargo's
integration path nor `IC_TEST_BINARY` was supplied. It never guesses a sibling
executable from an earlier build.
