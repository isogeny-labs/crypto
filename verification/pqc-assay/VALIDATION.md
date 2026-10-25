# Initial port validation

Base revision: `5cb553810e5bbfbd8691ec614ffcaacefc625757`.
Date: 2026-10-25. Host: Linux x86_64, Debian 13.6.
This is correctness validation. No performance measurement or comparison was made.

| Requirement | Status | Evidence |
| --- | --- | --- |
| Compile actual production functions | Verified | complete unchanged module plus ABI exports; all five production source hashes pinned |
| Rust equals upstream Cryptol reductions | Verified | SAW 1.5.1 exited 0 for all four `llvm_verify` obligations, overflow checks enabled |
| Wrong specifications are rejected for the intended reason | Verified | four output-bit mutants, each with an LLVM solver counterexample |
| Wrong production function is rejected | Verified | compiled `caddq` subtraction mutant rejected with a solver counterexample |
| Shell and workflow syntax | Verified | ShellCheck 0.10.0 and actionlint 1.7.12 exited 0 |
| Full Rust library regression suite | Verified with fixture replay | first run: 4626 passed, one missing-fixture failure, 119 pre-existing ignored; failed test passed after materializing its tracked session |
| Regenerated lifts and Isabelle reduction theorems | Implemented, unverified locally | `verify-math.sh`; workflow's manual `math_reference` input |
| NTT, signing, samplers, reachability and side channels | Not attempted | outside this scalar-reduction port |

## Commands

```sh
export RUSTUP_HOME=/workspace/toolchains/rustup
export CARGO_HOME=/workspace/toolchains/cargo
export PATH=/workspace/toolchains/cargo/bin:$PATH
export ASSAY_SAW_HOME=/workspace/toolchains/saw-1.5.1
verification/pqc-assay/setup.sh
verification/pqc-assay/verify.sh
verification/pqc-assay/mutations.sh
CARGO_TARGET_DIR=/workspace/toolchains/crypto-target cargo test --release --lib
```

Rust: `rustc 1.90.0 (1159e78c4 2025-09-14)`, LLVM 20.1.8.
SAW: `1.5.1 (1f227d9 (release tag v1.5.1))`.
Upstream: `6b0096f4a2bda9cf07c7347094affc85d9eb24ff`.
Compact raw proof and mutation outputs are retained under `evidence/`.
CI uploads fresh logs per run; a new run does not replace this initial receipt.

Conductor was unavailable (`command not found`). `docs/CONDUCTOR.md` describes
that state as a warning when its binaries/control plane are absent. No
production source was changed. There were no applicable example or Python
suite edits. The port uses shell orchestration and native Rust, not Python.

## Failed development attempts

The initial version check assumed SAW wrote its version to stdout; it writes
stderr. The first harness omitted the real `keccak4` dependency, and the
first proof script used an unsupported string concatenation operator. Those
attempts failed before verification. The initial completion check expected
a print message suppressed by verbosity 3; the final runner requires the
prover's exit status and four actual solver counterexamples. These changes
did not alter production functions or narrow their stated contracts.

The first library regression command failed because `isogeny_algos/` was
outside the sparse profile. The existing sparse-checkout script materialized
it and the command was rerun. That full run completed with 4626 passed,
one failure and 119 ignored. The failure was the existing
`cryptanalysis::ecbench::bounds::tests::committed_version_1_bounds_re_derive_under_their_recorded_version`
test, whose tracked session `research/ecbench_bounds_challenges_20261006/sessions/prime-toy-reference-e1/`
was also excluded by the sparse profile. After materializing that directory,
the exact failing test passed on replay (one passed, zero failed). Thus all
4627 non-ignored tests passed across the full run and its targeted replay;
the initial full command itself exited 101. No test was changed, disabled or
newly ignored. The compact development attempt logs and the full-run failure
and summary, plus the replay output, are retained under `evidence/`. The full
verbose library log is over the repository's Git data threshold and stays in
the execution workspace; its digest and byte count are recorded alongside
the compact result. This receipt does not claim a durable raw-log upload.
