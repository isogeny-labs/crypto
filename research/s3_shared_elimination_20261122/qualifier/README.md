# Source-component development qualifier

This small Cargo crate compiles the actual `exact_modular_matrix.rs` and
`shared_macaulay.rs` files, including their focused unit tests. It provides a
smaller development compilation after the full-library compiler was terminated
by SIGKILL. The signal alone does not establish memory exhaustion. No arithmetic
algorithm is rewritten or substituted.

The production dependencies have a small, explicit source closure:

| Source | Exact selected scope |
| --- | --- |
| `src/cryptanalysis/fp_backend.rs` | Imports through the complete `Fp32` implementation: the actual `PrimeField` trait, generic helpers, square-root setup, inversion, Barrett reduction, and CPU dispatch |
| `src/cryptanalysis/challenge_ladder.rs` | Actual `mulm`, `powm`, and `is_prime_u64` functions |
| `src/cryptanalysis/f4_fp.rs` | Actual `Poly` type alias and its documentation only |
| `src/cryptanalysis/exact_modular_matrix.rs` | Entire file, included by source path |
| `src/cryptanalysis/shared_macaulay.rs` | Entire file, included by source path |

`build.rs` verifies pinned whole-file SHA-256 hashes for the three selected
production sources, locates unique source boundaries, and copies the selected
bytes unchanged into Cargo's `OUT_DIR`. It verifies the written bytes against
the source range and records the exact offsets and both whole-source and
selected-range hashes. A production-source change fails the build until the
selection is reviewed and its pin deliberately updated. The two new modules
are included directly from their working-tree paths and their complete hashes
are recorded in the same manifest.

## Running the local correctness checks

From the repository root, after the build owner admits the local check:

```sh
cargo test --offline --release --jobs 1 \
  --manifest-path research/s3_shared_elimination_20261122/qualifier/Cargo.toml \
  --target-dir ../build --lib
```

The qualifier package uses `opt-level=0`, `codegen-units=256`, and no debug
information; registry dependencies keep the ordinary release optimization
level. This is a development qualification profile. Its elapsed time is not
performance evidence for the matrix kernels or the solver.

The manifest can be emitted without running an experiment:

```sh
cargo run --offline --release --jobs 1 \
  --manifest-path research/s3_shared_elimination_20261122/qualifier/Cargo.toml \
  --target-dir ../build --bin shared-s3-source-manifest
```

The first Cargo invocation creates this qualifier's own lockfile. Preserve that
lockfile and the emitted manifest with any test receipt; they bind the resolved
registry versions and the source bytes actually compiled. Use `--locked` for
subsequent replay of that frozen receipt.

## What a passing result establishes

The test names and their assertions establish the precise checked scope:
arithmetic equality against independent integer oracles, matrix admission and
chunk boundaries, specialization before elimination, shared/fresh row-space
parity, changing rank and support, cache-key normalization, and bounded
allocation refusal. These are the tests in the actual new production source
files, not rewritten versions of those tests.

This crate contains no F4 solver and no substitute for one. It does not compile
`examples/shared_s3_matrix.rs`, run the existing F4 visitor, verify the signed
IC witnesses, or run the complete root-library suite. Those remain separate
integration gates. The adjacent `../integration_qualifier/` exercises a declared
closure of the actual F4 and group source. The complete root suite still needs
a host on which that build completes. The unchanged integration fixture's tests are selected by
`cargo test --offline --release --example shared_s3_matrix` at the repository
root; passing the component qualifier does not imply that command passed.

## Manifest scope

The generated `shared-s3-source-component-manifest/v1` lists all locally compiled
algorithm bytes in this qualifier, its source-selection/build wrapper, and its
Cargo manifests. Registry package bodies are bound through the qualifier's
lockfile checksums rather than embedded. The manifest does not include the
compiler, linker, system libraries, CPU configuration, or arbitrary environment
flags, and it is not a hermetic build manifest or a PCC certificate. Record the
actual command, toolchain, outcome, and host constraints in the separate test
receipt. The manifest itself asserts no test pass or performance result.
