# Development validation record

Source baseline: `fc9af16fa1ce0654ee9df3ce63101c487ed215fb`.
Publication base: `b7f830121417997d8863189f435bcb66f6466ca2`; all tested PQC,
hash and RNG files have the same baseline bytes at both revisions. The
candidate source hashes were checked again after the refresh.
Candidate production change: only `src/pqc/ml_kem.rs`, as bound by
[`sources.sha256`](sources.sha256). Hash paths are relative to the repository
root. The focused harness and its lockfile are bound there too.

Native compiler: [`compiler.txt`](compiler.txt), Rust 1.99.1. Native host:
Linux x86-64, Intel Xeon Platinum 8370C. These are development correctness and
compilation checks, not a timed experiment or leakage assessment. Ignore Cargo
build/test duration messages when assessing performance; there is no A/A,
interleaved benchmark, isolated hardware epoch or external-reference run here.

## Commands and outcomes

```sh
# Existing source, then the encoder change (native x86-64):
cargo test --release --locked --manifest-path verification/pqc-portable/Cargo.toml
# Before: baseline-focused.log, 86 tests pass. After: candidate-focused.log,
# 88 tests pass (including the two new regressions); no test ignored or filtered.

# Run on both original and candidate source, once per listed target:
for target in aarch64-unknown-linux-gnu aarch64-apple-darwin \
  x86_64-pc-windows-gnu riscv64gc-unknown-linux-gnu i686-unknown-linux-gnu; do
  cargo check --release --tests --locked \
    --manifest-path verification/pqc-portable/Cargo.toml --target "$target"
done
# All ten checks pass: baseline-TARGET.log and candidate-TARGET.log.

cargo fmt --manifest-path verification/pqc-portable/Cargo.toml -- --check
# Against the committed change, check authored files while preserving raw data:
git diff --check HEAD^ HEAD -- . \
  ':(exclude)research/pqc_cross_platform_20261030/evidence/*.log' \
  ':(exclude)research/pqc_cross_platform_20261030/evidence/*.diff'
sha256sum --check verification/pqc-assay/sources.sha256
# All pass. The last command checks identity, not theorem-prover execution.

cargo test --release --lib --locked
# library-locked.log: refuses the existing inconsistent root lockfile.
cargo test --release --lib
# library-candidate.log: normalizes the local lockfile, then fails on the
# unclosed delimiter in src/cryptanalysis/ecbench/methods.rs:8824 (exit 101).
# library-baseline-replay.log: exact original encoder restored, same failure.
# generated-root-lock.diff: local-only normalization, subsequently discarded.
# library-refreshed-base.log and library-refreshed-baseline.log: candidate and
# original encoder on the publication base both fail (101), now first naming
# src/cryptanalysis/binary_semaev_chain_sat.rs:1703. Local root lock changes
# were again discarded, and the candidate encoder was restored byte for byte.
```

`harness-first-build.log` preserves the initial module-path build failure;
restoring the actual production module hierarchy fixes it. `format-first.log`
records the missing rustfmt component, which was installed; `format.log` is
the empty successful formatting output. The first baseline harness invocation
generated the focused lockfile without `--locked`; subsequent baseline and
candidate checks use its unchanged generated resolution.

No examples or Python suites were changed. No proof-bound source was changed.
No compiled artifact is substituted for the actual production modules; the
focused package compiles them directly by their repository paths.

The unrestricted staged whitespace check reports trailing blank lines in the
two verbatim Cargo test logs and a space-only context line in the generated
lock diff. Those are original tool output, not edited source. They are retained
byte for byte; the authored-file check explicitly excludes only these raw log
and diff formats. No correctness test or CI gate has been removed or changed.
