# RR5 and root development commands

Working directory: `/workspace/scratch/5e8abb017b93/crypto-build`.
The direct installed Cargo and rustc were used with the existing registry cache:

```sh
CARGO_HOME=/workspace/scratch/cdbe94c88979/toolchain/cargo
RUSTC=/workspace/scratch/cdbe94c88979/toolchain/rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc
CARGO_TARGET_DIR=/workspace/scratch/5e8abb017b93/rr5-target
CARGO=/workspace/scratch/cdbe94c88979/toolchain/rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/cargo
```

The variables were supplied by `env` to each invocation (the exact path values
above were passed as arguments; no implicit managed-wrapper behavior was used).
Each command's stdout and stderr were redirected together to its named `.log`;
the shell wrote its terminal status to the matching `.exit`.

1. `timeout 240 "$CARGO" test --offline --locked --release --manifest-path research/prime31_four_summand_20261122/native/Cargo.toml --lib -j1 -- --nocapture`
   - `native-tests-initial.log`, `.exit`: exit 0, 42 library tests.
   - Contains detailed existing and new public-input receipts. The new empty
     query fixture was corrected to 100G before native test compilation: G is
     representable by the disclosed signed base and must not be called empty.
2. `timeout 240 "$CARGO" test --offline --locked --release --manifest-path research/prime31_four_summand_20261122/native/Cargo.toml --all-targets -j1 -- --test-threads=1`
   - `native-all-targets.log`, `.exit`: exit 0, 42 library and 8 example tests;
     the existing three binary targets also compile as test targets.
   - Serial test execution on the final RR5 source. It catches downstream
     compatibility risks from the new public enum status and module exports.

Compiler/test elapsed-time messages are preserved raw; they are not matched
performance measurements. The initial library invocation used the harness's
default test parallelism; the final compatibility invocation was serial.
All ten new field-dependent controls use the original ordinary M31 curve.
The older unchanged regression suite retains its historical fixtures.

## Root library gate

The full-root command is separate from the standalone component gates:

```sh
"$CARGO" test --offline --locked --release --lib -j1
```

- `../../evidence/root-library-initial.log` and `.exit` record the default-wrapper
  attempt refusing a sparse path dependency before compilation (exit 101).
- `../../evidence/root-library-dependency-restored.log` records the direct
  toolchain invocation under `timeout 600`, target directory
  `/workspace/scratch/5e8abb017b93/root-target`, after sparse dependency hydration.
  Its execution handle disappeared with no terminal exit; the exact capture
  limitation is in `../../evidence/root-library-capture-status.txt`.
- `../../evidence/root-library-retry.log` is a separately named cached retry
  under `timeout -k 10s 240s`, with the same root target directory. Its `.exit`
  is authoritative if present; absence of the exit does not mean success.

These are local development builds under AGENTS section 9a. No scientific
panel, new collection, paid job or benchmark run was launched.

The cached root release-test retry returned 124 while compiling dependencies.
A diagnostic `cargo check --offline --locked --release --lib --tests -j1`
returned 101 before compilation because two unrelated sparse binary targets
were absent (`../../evidence/root-compiler-diagnostic.*`). The narrower
`cargo check --offline --locked --release --lib -j1`, under the same 240-second
cap, reached production source and printed E0425 for powers, gcds, previous,
r and g in unchanged `src/cryptanalysis/prime_root_trace.rs`, then returned
124 at the cap (`../../evidence/root-library-check.*`). The log also records
an absent tracked ecbench panel; that input was hydrated. The source file's
byte equality to main is separately recorded. No root test pass is claimed.
