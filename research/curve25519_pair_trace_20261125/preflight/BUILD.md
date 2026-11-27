# Preflight build receipt

- Task source: `c567c15ecf151f86db4fc351f5bc1257285f9911`.
- Compile-repair base: PR #4650 revision
  `d0d4dcaed96cfba2b971abef656decacb428474c`, plus the previously disclosed
  local shims retained by the Curve25519 studies.
- Disposable verification head:
  `127b0f81313253ce0ccf8f80ab9f08bcd08f3345`.
- Toolchain: `rustc 1.94.1 (e408947bf 2026-03-25)`.
- Measurement binary SHA-256:
  `4f0d9d328e74c2d166ac30e12206dd6d3a8c58d372c36b8d227c154efbcc916a`.
- Pinned predecessor D-reproduction binary SHA-256:
  `5972800a79a50b27d725d961cbc60ecc512a9c3c1923a73c2b38b5aa9c3da9d4`.

`cargo test --release --example curve25519_pair_trace` passed 4/4 tests.
The initial test run correctly rejected the unquadratized cubic trace-doubling
term; Amendment 1 was filed before solver data, the local cross products were
retained, and the unchanged tests then passed.

The native self-test exhaustively checked the pair-trace bijection and trace
root set at every study order, all retained circuit assignments, 32 signed
main-curve `d=7` relations per arm, four planted targets per order and arm,
and the predecessor's 24-positive/12-negative tiny truth panel.
