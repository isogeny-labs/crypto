# Current affected Rust validation

This research changes no Rust library source, dependency, benchmark or test.
The branch's Rust source base is
`aaab886ec25b8cd5a54c97ca75c916b5f7dbb0ef`.

The earlier [uncapped full-library run and fixture replays](../../prime_tensor_variants_20261030/receipts/ROOT-VALIDATION.md)
were at source base `7923f05fcb96bdf653545b09cba2a5d05cd3c744`.
Six Rust paths changed upstream between those bases, listed in
`root-changed-since-prior-run.txt`. Therefore the old outcome is not
presented as a fresh full-suite result at the current source.

The current affected areas were rebuilt and tested with unchanged default
release settings and locked dependencies:

```sh
CARGO_HOME=/workspace/audit/toolchains/cargo \
RUSTUP_HOME=/workspace/audit/toolchains/rustup \
CARGO_TARGET_DIR=/workspace/audit/prime-root-cargo-target \
/workspace/audit/toolchains/cargo/bin/cargo +stable \
  test --release --lib --locked cryptanalysis::ecbench::

CARGO_HOME=/workspace/audit/toolchains/cargo \
RUSTUP_HOME=/workspace/audit/toolchains/rustup \
CARGO_TARGET_DIR=/workspace/audit/prime-root-cargo-target \
/workspace/audit/toolchains/cargo/bin/cargo +stable \
  test --release --lib --locked cryptanalysis::aps_single_relation::
```

| Actual command | Result |
| --- | --- |
| First ECBench attempt | Exit101: two tracked `include_str!` fixtures were off disk |
| Same ECBench command after materialization | Exit0:395 passed,0 failed,1 default ignored,5171 filtered out |
| APS single-relation filter, same rebuilt binary | Exit0:34 passed,0 failed,0 ignored,5533 filtered out |

The two restored files are
`research/aps_measurement_20261029/spec-controls.json` and
`spec-orbit-controls.json`. Their worktree Git blob hashes exactly match
HEAD: `c11e5f1515b0080aae37539e8321cc82be573f3c` and
`4f832f6190d8347f42892e304fec986453e59f66`. No fixture or source was
rewritten to make a test pass. The original compilation failure is retained.

The two passing filters are disjoint:429 passes and one existing ignored
case. This is targeted coverage of the changed areas, not a fresh single
green uncapped full-library run. The prior full-run exit101 and its26
subsequently passing fixture replays remain historical evidence as linked
above, not silently replaced by these results.

Complete current successful logs are timestamp-free gzip files beside this
receipt. Original raw-log and rebuilt-binary SHA256 values are in
`root-validation-bindings.sha256`. Compiler durations printed by Cargo are
unisolated development metadata and support no performance comparison.
