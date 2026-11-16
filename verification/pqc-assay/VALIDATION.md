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

## CI follow-up, 2026-10-25

The native proof, Rust compilation/lint, workflow-syntax and ecbench checks
passed on `bab2ff34690931cb260128d486f6d492d10b7763`.
The artifact job's sparse checkout omitted the new proof directory. Its
checker could not resolve the abbreviated object ID `59dae9f` for the new
`.gitattributes` file. A fresh blobless sparse clone reproduced that failure;
including `/verification/pqc-assay/` made the same checker pass. The workflow
now includes that input directory. `evidence/artifact-sparse-before.txt` and
`artifact-sparse-after.txt` preserve the two results. The gate and its storage
policy are unchanged.

Two initial-head gates report issues in unchanged research data:

- [curve names](https://github.com/aburan28/crypto/actions/runs/40150544527):
  unregistered curve slugs in existing research reports and dashboards;
- [validation](https://github.com/aburan28/crypto/actions/runs/40150544471):
  33 existing leaderboard/benchmark issues, starting with a missing
  `VALIDATION.json` for `research/ic_tool_program/rule/v7/analysis.json`.

Those gates remain merge requirements. No curve registration, frozen record
or validation admission was changed to make this port pass. Merge is
authorized, but remains pending successful applicable checks.

## Standalone suite import, 2026-10-25

The complete public upstream repository is a Git submodule at `upstream/`,
pinned to `6b0096f4a2bda9cf07c7347094affc85d9eb24ff`. Its sources, Makefile,
models, theories, target bytes and licenses are unmodified. The upstream
private `paper/` submodule remains uninitialized and is not a proof input.
The local native Rust proofs and their original receipts remain unchanged.

| Requirement | Status | Evidence |
| --- | --- | --- |
| Import the standalone public proof suite | Verified | `.gitmodules`, pinned Git link, clean upstream checkout |
| Check vendored target identity, claim lint and generated HQC inputs | Verified | `evidence/standalone-integrity.txt`; runner exited 0 |
| Initialize and check in a fresh sparse worktree without private access | Verified | `evidence/standalone-fresh-worktree.txt`; runner exited 0, `paper/` remained uninitialized |
| Execute an upstream proof target through the runner | Verified | `mlkem-reduce` exited 0: two C-to-model proofs, two mathematical properties, two solver counterexamples for wrong-result controls |
| Artifact policy and workflow wiring | Verified locally | existing artifact checker accepted the full staged implementation diff; `wire_workflows.py --check` exited 0 |
| Shell and workflow syntax | Verified | ShellCheck 0.10.0 and actionlint 1.7.12 exited 0 |
| Full upstream `make verify`, Isabelle/AFP setup and auxiliary targets | Implemented, unverified here | standalone runner and manual `standalone_verify` macOS job; not executed locally |
| Release Rust library regression suite after import | Verified with network replay | full run: 4618 passed, nine loopback-permission failures, 119 pre-existing ignored; networking group replay: 44 passed, zero failed |

Commands from the repository root:

```sh
verification/pqc-assay/standalone.sh check
LD_LIBRARY_PATH=/workspace/toolchains/clang19/usr/lib/x86_64-linux-gnu \
PATH=/workspace/toolchains/saw-1.5.1/bin:$PATH \
verification/pqc-assay/standalone.sh make mlkem-reduce \
  SAW=/workspace/toolchains/saw-1.5.1/bin/saw \
  CLANG=/workspace/toolchains/clang19/usr/bin/clang-19
python3 tools/artifacts/wire_workflows.py --check
CARGO_TARGET_DIR=/workspace/toolchains/crypto-target \
  cargo test --release --lib --offline
# Repeat the unchanged networking group with loopback socket access enabled:
CARGO_TARGET_DIR=/workspace/toolchains/crypto-target \
  cargo test --release --lib --offline cryptanalysis::pollard_collab
```

The ML-KEM run used SAW 1.5.1 and Debian clang 19.1.7 on the same Linux host
as the original port. `evidence/standalone-mlkem-reduce.json` preserves the
complete proof output as a JSON string, including the prover's trailing
spaces, without changing the repository's whitespace checks. Decode the
exact bytes with `jq -jr .stdout`; that round trip was checked with `cmp`.
`standalone-tools.txt` records versions and input and bitcode hashes. The
upstream script's embedded timing descriptions are not new performance
measurements.

The fresh-worktree check used a temporary local snapshot of the staged
implementation, before adding these receipts. It cloned the public submodule
and fetched its exact pinned revision. The existing artifact checker accepted
the diff from the original base to that snapshot. Its sparse inputs now also
include `.gitmodules`; neither artifact policy nor checker code changed.
An earlier diagnostic `--shared` clone of the blobless superproject failed
while Git tried to transfer an unavailable promisor object, before any check
ran. `evidence/standalone-local-clone-attempt.txt` records that failure; it
does not establish source corruption. The sparse-worktree check then passed.

Conductor is still unavailable (`command not found`), the warning state
documented in `docs/CONDUCTOR.md`. No production Rust or upstream source was
changed. Upstream's existing Python generator is used for its input-integrity
check; no Python cryptographic implementation was added.

The library command used the same Rust 1.90.0 selectors as the original run.
It exited 101 with 4618 passed, nine failed and 119 pre-existing ignored.
All nine failures were `PermissionDenied: Operation not permitted` while
binding `127.0.0.1:0` in the existing `pollard_collab` networking tests.
With network access enabled in the execution sandbox, the unchanged
`cryptanalysis::pollard_collab` group passed all 44 tests, including all nine
failures. Thus all 4627 non-ignored library tests passed across the full run
and this replay; the full command itself did not exit 0. The failure summary
and complete replay output are in `evidence/standalone-library-*.txt`.
The 406864-byte full-run log remains in the execution workspace; its digest
and size are in `evidence/standalone-library-raw.sha256`. No durable raw-log
upload is claimed. No tests were changed, skipped or newly ignored.
