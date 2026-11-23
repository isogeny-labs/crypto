# K600 m6 source-bound Linux build producer

Status: **implemented and locally tested; Linux build not executed**. This
amendment adds `examples/n83_k600_m6_linux_build.rs` as the native child for
the budget controller's already-defined `charge build` phase. It does not
produce a K600 measurement, rank result, factor-base comparison, or total
index-calculus runtime.

## Contract

Run on a Linux aarch64 host from a clean checkout at committed build revision
`C`:

```text
n83_k600_m6_linux_build run COMMITTED_BUILD_PLAN_JSON NEW_OUTPUT_DIR
```

The JSON plan must be byte-for-byte tracked at `HEAD`, with schema
`n83.k600-m6-linux-build-plan/v1`, status `FROZEN_BUILD_PLAN`, target
`linux/arm64`, a fresh `image_ref`, and exactly three ordered steps:
`rust-binaries`, `solver-binary`, `runtime-image`. Each step specifies an
absolute executable path and SHA-256, string arguments, explicit environment
map, bounded timeout (1–3600 seconds), and a list of external input files
with absolute paths and SHA-256 values. The runner clears inherited
environment before each child, records command/log hashes and exit status,
rechecks the source, executable, and listed inputs, and writes immutable
step receipts. Timeout kills the child's process group and marks cleanup
unverified. A failed step stops the sequence and leaves a terminal
`failure.json`.

Success requires six output-local executable Linux ARM64 ELF files with
complete 64-byte ELF headers named `worker`,
`exporter`, `solver`, `arm-supervisor`, `preflight-supervisor`, and
`budget-controller`; their SHA-256 values are recorded in
`n83.k600-m6-linux-build/v1` `build.json`. The Docker image must be new,
have `linux/arm64` architecture, and carry
`org.opencontainers.image.revision=C`; its immutable image ID is recorded.
The four-phase precharge verifier already accepts this receipt only when its
source, image ID, exporter, solver, and supervisor identities match the arm
lock. Its enclosing `charge build` receipt accounts for the whole build
producer invocation through its child exit and receipt check.

The plan must enumerate all external executable inputs used by its commands.
The current code checks the declared files; it cannot prove that an arbitrary
build script did not read an undeclared external file. The chosen build plan
and host therefore require review before execution. ELF headers and Docker
metadata are admission checks, not proof that the six programs start or
produce correct results; the real zero-trial and constructed-arm controls
remain required. The `build.json` write occurs inside the outer charged
child interval. The outer controller's own receipt write and process exit
are still outside its measured interval, so a full-invocation cold wall
receipt remains open.

## Local validation and remaining work

On macOS arm64 with Rust 1.99.1, the release example tests pass 2/2.
The tests check ELF architecture rejection, placeholder expansion, and
external-input hash mismatch detection. A production release build and the
existing budget-controller tests pass 5/5; their exact commands use
`RUSTC="$(rustup which rustc --toolchain 1.99.1)" cargo
test --release --example <example>`, with
`n83_k600_m6_linux_build` and `n83_k600_m6_budget` respectively.
The production build uses `cargo build --release --example
n83_k600_m6_linux_build` under the same `RUSTC` selection.
Compressed [producer test](linux-build-producer-test.log.gz),
[producer build](linux-build-producer-build.log.gz), and
[budget test](linux-build-budget-test.log.gz) logs retain the raw outputs.
The repository-wide `cargo test --release --lib` gate, with the same
`RUSTC`, exited 101 after 607.72 seconds: 6,306 passed, 59 failed,
138 ignored. Its [full compressed log](linux-build-full-lib.log.gz)
shows missing tracked fixtures in this sparse checkout, including
`gpu/certicom` source and ecbench session records. The new commit has
no `src/` changes. This is a failed validation gate, and the PR cannot
merge on this local result. No
Linux image, solver binary, Docker container, preflight, or K600 arm was
executed by these tests.

Next, freeze an actual host-specific plan at revision `C`, including native
Rust and CryptoMiniSat build commands and pinned external inputs. Build on a
qualified Linux arm64 host inside the `charge build` interval; verify its
receipt and six binaries; commit the write-once preflight lock at child
revision `H`; then run staging, zero-trial preflight, constructed controls,
natural query and independent replay under the frozen gates. Preserve every
failure and all elapsed intervals. No canonical performance graph changes
here because this amendment adds code and no measured K600 values.
