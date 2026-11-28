# Preflight build receipt

- Task source: `cac5c78a9b` on the follow-up branch.
- Compile-repair base: PR #4650 revision
  `d0d4dcaed96cfba2b971abef656decacb428474c`, plus the disclosed local shims
  retained from the predecessor studies.
- Disposable verification head: `12f07e125a`.
- Toolchain: `rustc 1.94.1 (e408947bf 2026-03-25)`.
- Measurement binary SHA-256:
  `5972800a79a50b27d725d961cbc60ecc512a9c3c1923a73c2b38b5aa9c3da9d4`.

`cargo test --release --example curve25519_quotient_layout` passed 5/5
tests. The native self-test checked exact inverse-permutation identity for
every quotient layout, 1,088 unordered distinct-root triples per arm, four
planted subgroup targets per order and arm, and the 24-positive/12-negative
tiny exhaustive panel.

One fresh random target was run for all five arms concurrently as smoke. Those
rows are retained separately and are not qualification rows. The frozen runner
repeats the complete panel sequentially in target-major D/T/F/E/I order.
