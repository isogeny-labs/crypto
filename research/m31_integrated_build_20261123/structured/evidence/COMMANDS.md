# Structured component development commands

All paths below are relative to the combined checkout unless absolute. These
are bounded local build and unit-test commands, not scientific benchmarks.
Historical failed attempts are retained without replacement.

## Initial tool checks

`cargo fmt --manifest-path research/m31_integrated_build_20261123/structured/Cargo.toml --check`
returned exit 1: `cargo-fmt` was not installed for the active stable toolchain.
No formatter pass is claimed.

`unit.initial.log` / `.exit` records the initial offline Cargo attempt, which
returned 101 because the managed wrapper's registry cache lacked `serde`.
`unit.cached.log` / `.exit` records the same error after supplying `CARGO_HOME`:
the managed wrapper unconditionally overwrote that variable, so it still used
the other cache. Neither attempt ran a test binary.

## Explicit installed compiler and hydrated registry

The direct installed `cargo` executable is used below because the managed
wrapper selects a different cache. No new compiler or package was downloaded.

```sh
CARGO_HOME=/workspace/scratch/cdbe94c88979/toolchain/cargo \
RUSTC=/workspace/scratch/cdbe94c88979/toolchain/rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc \
RAYON_NUM_THREADS=1 \
timeout 180 \
/workspace/scratch/cdbe94c88979/toolchain/rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/cargo \
test --offline --locked \
--manifest-path research/m31_integrated_build_20261123/structured/Cargo.toml \
--lib -- --test-threads=1
```

`unit.direct.log` / `.exit` retains the first direct-compiler attempt (the
initial invocation omitted `--locked` while creating the new lock file).
Dependency compilation reached the source-provenance build script and stopped
with exit 101 because its tracked `RUN66_Y7_EXTRA168.csv` input was sparse.
The parent session materialized the tracked dependency; no fixture bytes were
edited. `unit.hydrated.log` / `.exit` records the subsequent command above.

The CLI integration command uses the same environment and deadline, replacing
`--lib` with `--test cli`. Its output is `cli.hydrated.log` / `.exit`. These
tests launch the actual binary as a child process and check success, refusal,
partial-output exit codes, and signed witness replay.

Compiler messages include the test harness's own elapsed-time displays. They
are retained raw and are not used as performance measurements. The source
closure emits inherited dead-code/import warnings; a passing development
test does not satisfy the repository's independent signed validation gate.

## Auxiliary generic identity record and final target gate

`identity-certificate.command.txt` records the exact cached release invocation
of `m31-structured-identity-certificate`, under `timeout 180s` with the installed
Cargo/rustc and populated cache. Its `.log` and `.exit` record exit 0. The
certificate uses the fixed M61 format for generic formal-variable algebra;
it is not a curve fixture, an M31 test replacement, or a formal proof.

`release.final.command.txt` records the subsequent exact release all-targets
command, also under `timeout 180s`, with one build job and one test thread.
`release.final.log` and `.exit` record exit 0, 10 library tests and 3 CLI tests
passed, and successful compilation of both binary targets. The auxiliary
binary's actual issuance/refusal controls ran in the separate invocation above.
The earlier `release.all-targets` result remains retained; its precise enclosing
timeout was not recorded and is not inferred from this final command.

## Release compatibility gate

The same installed direct toolchain and cache ran the component command with
`--release --all-targets -j1 -- --test-threads=1`. Its enclosing timeout was not
recorded by that worker and is not inferred. The working target directory was
`structured/target`. The terminal
`release.all-targets.log` / `.exit` records exit 0 with 10 library tests and
3 CLI tests. The library and CLI implementation sources are bound by the
combined source manifest; source hashes for the independently executed debug
library binary are also recorded in the review handoff. [pcc:unverified]
