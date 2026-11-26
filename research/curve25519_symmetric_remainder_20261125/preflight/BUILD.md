# Preflight build receipt

- Task source: `636423e969` on
  `codex/curve25519-exponent-ideas-20261117`.
- Verification base: compile-repair PR #4650 revision
  `d0d4dcaed96cfba2b971abef656decacb428474c`, with the same disclosed local
  duplicate/module shims used by the preceding projective study.
- Disposable verification head: `1c1fc4b7f59ff9521ea499c3b713cc05d738f2c1`.
- Toolchain: `rustc 1.94.1 (e408947bf 2026-03-25)`,
  `cargo 1.94.1 (29ea6fb6a 2026-03-24)`.
- Host: Linux 6.18.57, x86-64.
- Release binary SHA-256:
  `b3bf1294e3c8f3b86662b398e163c5c82ced01097294d85b6696f0edc17bad02`.

`cargo test --release --example curve25519_symmetric_remainder -- --nocapture`
passed 3/3 tests: quotient coefficient identity, all frozen orders ending at
remainder one, and the committed generator having order `r`.

`cargo run --release --example curve25519_symmetric_remainder -- selftest`
produced `selftest.json`. The compile-repair overlay changes no equation,
target, seed, cell, threshold, or F4 metric in this study.
