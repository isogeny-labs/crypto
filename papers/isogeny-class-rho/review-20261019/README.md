# Review receipts

`stats.rs` is an exact copy of companion PR #2174 source
`a277c5f06469bdb61b3af4a7e6c7df5cde7019d0:src/bin/isoclass_rho/stats.rs`.
It is preserved for the negative counterexample, not a corrected algorithm
or a second implementation to use in production. `main.rs` calls its native
KS implementation on identical tied samples and asserts the required D=0.
The historical source yields D=1, so the recorded exit 101 is the expected
defect reproduction, not a passing correctness test. `result.log` preserves
that original local reproduction. This new receipt directory is versioned;
it does not replace any frozen historical run output.

Reproduce after the repository's normal release dependencies are built:

```sh
rustc --edition=2021 main.rs -L dependency=/path/to/release/deps \
  --extern rand=/path/to/release/deps/librand-HASH.rlib \
  --extern serde_json=/path/to/release/deps/libserde_json-HASH.rlib \
  -o /tmp/isoclass-historical-ks-counterexample
/tmp/isoclass-historical-ks-counterexample
```

Select the existing release artifacts for the root Cargo.lock (`rand` with
`small_rng`; `serde_json`). Review used Rust 1.98.0. The corrected source
and positive correctness controls belong in the companion source PR.
The historical producer/row lineage must still be established before
correcting the study's statistics; a source counterexample alone does not
compute replacement historical p-values.

`lean-build.log` records successful elaboration with Lean 4.34.1 and pinned
Mathlib (2900 jobs). `leanchecker.log` records builtin kernel replay of all
three modules. `sources.sha256` binds the project sources, toolchain,
manifest and native counterexample sources. There is no `sorryAx` in the
successful final axiom audit. See the adjacent proof map for exact scope and
open concrete bridges.

`library-validation.json` and `library-release.log` record the successful
required whole-library release suite on the exact tested source tree. This
local validation does not close the empirical/proof bridges or replace
final-head GitHub CI.
