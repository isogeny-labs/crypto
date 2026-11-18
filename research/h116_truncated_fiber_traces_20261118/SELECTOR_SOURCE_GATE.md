# H116 selector source gate

This gate covers structural cohort selection only. The selector has no code for
extension-field torsion, fibers, moments, scalar labels, predicates, timing or
the candidate algorithm.

Pre-selection checks:

```text
RUSTUP_HOME=/workspace/.h116-rustup \
  /workspace/.h116-cargo/bin/rustc --edition=2021 -D warnings \
  h116_select.rs -o /tmp/h116_select
/tmp/h116_select --selftest
h116 selector selftest: pass
```

Compiler identity:

```text
rustc 1.100.0 (0ef1bc386 2026-11-09)
host: x86_64-unknown-linux-gnu
LLVM version: 23.1.5
```

Pre-selection SHA-256 bindings:

```text
1528e504a9dd0a0d688041c4efcde8209617d8fab6710be99bd718532768afcf  h116_select.rs
ba08a3abca253a0c152a1828e3eb500cf9581bf60a0b408d0620517cf7e9ce6f  /tmp/h116_select
```

The selector source must be committed and pushed before `/tmp/h116_select`
is invoked without `--selftest`. The resulting `COHORT.json`, raw stdout hash,
compiler version and source hash will be bound in a later receipt before any
candidate/reference moment run.
