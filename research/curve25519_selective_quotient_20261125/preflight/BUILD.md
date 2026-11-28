# Preflight build receipt

- Task source: `042f14d030` on the follow-up branch.
- Compile-repair base: PR #4650 revision
  `d0d4dcaed96cfba2b971abef656decacb428474c`, plus the disclosed local shims
  retained from the predecessor study.
- Disposable verification head: `523b8b5d3a6ba32d784020fd126fff3530beff2e`.
- Toolchain: `rustc 1.94.1 (e408947bf 2026-03-25)`.
- Measurement binary SHA-256:
  `c83526ee09d99a012741c3a8520612d5b192e1823c2c070efd671fe9e18088d1`.

`cargo test --release --example curve25519_selective_quotient` passed 4/4
tests, including the exact frozen `d=19` cut sets. The exhaustive native
selftest produced `selftest.json`.

Before the qualification run, one random target was executed for D, A, and B
concurrently as smoke. D and A reproduced the predecessor target, count,
degree, matrices, steps, and verdict exactly. Those three smoke attempts are
retained separately and are not qualification rows; the frozen runner repeats
the complete panel sequentially in target-major order.
