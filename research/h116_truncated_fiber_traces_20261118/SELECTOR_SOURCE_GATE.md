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

The first unoptimized default invocation was terminated after 75 CPU seconds
without producing stdout. Because the wrapper did not retain its session
handle, any eventual stdout would also have been inaccessible. No cohort file
or partial row was emitted. Before retrying, the unchanged bound source was
compiled with `-C opt-level=3`; its self-test passed and its SHA-256 is:

```text
a4ef1507157f55ba8e8f1ee01089f87f08b66b13553fcce4ef1e8d0d9034748e  /tmp/h116_select.opt
```

This amended binary binding is committed and pushed before its first default
invocation. Optimization changes implementation speed, not the frozen selector
rule or ordering.

The selector source was committed and pushed before `/tmp/h116_select` was
invoked without `--selftest`. The resulting `COHORT.json`, raw stdout hash,
compiler version and source hash will be bound in a later receipt before any
candidate/reference moment run.
