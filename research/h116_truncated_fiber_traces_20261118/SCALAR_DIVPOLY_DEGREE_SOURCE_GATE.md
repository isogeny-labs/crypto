# H116 scalar division-polynomial source gate

The frozen protocol predates both implementations and the result. The exact
executed sources are:

```text
225da661ef80694e2652f94dd62993c73935ac059a147b22fb4b71c8af38de1e  h116_scalar_divpoly_degree.rs
4e802fb2bd6e6889869f5b899b729f99d0cc23778b4b4684e6e34704e5c2e6bb  h116_scalar_divpoly_degree_verify.py
```

The producer was compiled with the pinned Rust 1.100.0 toolchain using
`rustc --edition=2021 -O -D warnings`. Its self-test passed before the
result was emitted. The verifier is a separate Python implementation: it uses
independent extension-field/group arithmetic, independently implements the
division-polynomial recurrence, exhaustively replays every point, and compares
all correctness fields. It does not parse or execute the Rust implementation.
