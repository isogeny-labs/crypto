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

Initial pre-selection SHA-256 bindings:

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

The optimized exhaustive invocation was terminated after exactly 2m56.339s of
wall time and 2m56.186s of user CPU. Its captured stdout remained zero bytes,
with the empty-file SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
It therefore disclosed no selected curve or partial row.

Before another invocation, selector v2 applies the Hasse bound first: it
enumerates the possible group orders in the public Hasse interval, applies the
frozen primality, distinctness, torsion-degree and extension-size conditions,
and point-counts models only if at least one order remains. Models are still
visited in `(a,b)` order and qualifying orders are still sorted before global
selection. This is an implementation optimization of the same selection
function, not a cohort-rule change.

Selector v2 strict builds and self-tests passed. Its pre-selection SHA-256
bindings are:

```text
e54c745d3ead1184818fe792ca346d06f32d2d0c6b83ef2f6d63543117dae7f4  h116_select.rs
dded0b7bdb374b3d1b7c668604713c0c2e05f1b879e9e304c8540e8e9c89c0f3  /tmp/h116_select.v2.debug
c513a0d72a8eef3f294e0e10f9bf42ddaf83598485c5fb42e4dfb0c7576e2d3c  /tmp/h116_select.v2.opt
```

This v2 source and binary binding is committed and pushed before its first
default invocation.

Selector v2 completed in 0.003s user CPU and halted before printing because the
frozen rule found only two of five required curves. Its captured stdout was the
zero-byte file with SHA-256
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
This is a structural feasibility failure; no fiber, moment, scalar or timing
data was computed.

`FEASIBILITY_PLAN.md` now freezes a four-rung maximum-extension-size ladder and
requires the smallest rung yielding five curves. Selector v3 adds only the
`--feasibility` reporting path for that ladder. Strict builds and self-tests
passed. Pre-feasibility SHA-256 bindings are:

```text
9f3ef019c820826bf883ee7f7112d508998243c5ce655290603a1ef52636fa1c  h116_select.rs
cf85d3aa832aed78aa7a446dcee443095e144a87cb29ab08aba4bf059ce8dd23  /tmp/h116_select.v3.debug
8f6fc2ff6dd5ab732c04e7f82aa5c108e5e8bd6991cf32b4501ce830d9b5c06c  /tmp/h116_select.v3.opt
```

The plan, v3 source and binary binding are committed and pushed before the
first `--feasibility` invocation.

The selector source was committed and pushed before `/tmp/h116_select` was
invoked without `--selftest`. The resulting `COHORT.json`, raw stdout hash,
compiler version and source hash will be bound in a later receipt before any
candidate/reference moment run.
