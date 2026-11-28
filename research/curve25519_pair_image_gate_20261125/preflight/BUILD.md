# Native certificate build receipt

- Task source revision: `651f7402cb`.
- Compile-repair base: PR #4650 revision
  `d0d4dcaed96cfba2b971abef656decacb428474c`, plus the previously disclosed
  local shims retained by the Curve25519 studies.
- Disposable verification head:
  `81e53a5546c7d73849f9983955b475892ca92a47`.
- Toolchain: `rustc 1.94.1 (e408947bf 2026-03-25)`.
- Release certificate binary SHA-256:
  `f8be115c9bcf73d9511cf12f590dd9d05b2bd5f348d76612e6b37fe343608f50`.

`cargo test --release --example curve25519_pair_image_degree` passed 2/2
tests. The first pre-data test run found that the coprime control expected the
zero polynomial instead of the correct gcd `1`; only that assertion was
corrected before executing the certificate. Production squarefreeness already
used degree-zero gcd as the passing condition.

Two release-binary executions were byte-identical. The certificate uses only
native Rust finite-field and polynomial arithmetic.
