# H016 native integration: development validation

This note records local development checks for PR #5153. All results and source
observations remain **[pcc:unverified]**. They are not an independently signed
execution receipt, a historical H016 replay, or a performance comparison.

## Source and environment

- Original evidence commit: `d1de3989b248a14a56c8d1fdf0556fa4d6de3d8e`.
- Current-main source merged into the task branch: `53df0c3ddc22de8774d83f6ec3a6c00fffcfd1ee`.
- Integration started from task-tracking commit: `b60f801660eb423901ddea0c03c629089a1b5bfc`.
- Compiler: `rustc 1.100.0 (0ef1bc386 2026-11-09)`; Cargo 1.100.0; Linux x86_64.
- Cargo uses the committed lockfile. An isolated official rustup installation
  supplied the otherwise missing toolchain. No dependencies or tracked Cargo
  profile settings were changed.

## Native execution attempts

| Attempt | Command (toolchain environment omitted) | Observed result |
| --- | --- | --- |
| Initial library compilation | `timeout 600s cargo test --release --locked --lib --no-run` | Exit 137 / SIGKILL. The log also had 54 missing-include errors covering 45 tracked files omitted by the sparse checkout. This attempt began while source integration was in progress; it is not a frozen baseline. |
| Restored-input library gate | `CARGO_BUILD_JOBS=1 timeout 600s cargo test --release --locked --lib -j 1` | Exit 101; `rustc` terminated with signal 9 / SIGKILL before any test executed. No missing-include errors repeated. |
| Default-release focused caller | `CARGO_BUILD_JOBS=1 timeout 600s cargo test --release --locked --example h016_factor_base_audit -j 1` | Exit 137 / SIGKILL while compiling the ordinary root library; no test executed. |
| Package-only optimization override | See command below. | Exit 0; 24 tests passed, none failed or ignored. The final rerun also compiled and linked both modified existing examples. |

All 45 omitted tracked compiler fixtures were restored exactly (2,160,077
bytes); their diff is empty. The retry failure is preserved in
[`validation/root-release-lib.log`](validation/root-release-lib.log). The
specific resource responsible for SIGKILL was not independently established;
this note does not infer an OOM diagnosis from the signal alone.

The separate focused qualification command is:

```sh
CARGO_BUILD_JOBS=1 timeout 600s cargo \
  --config 'profile.release.package.crypto.opt-level=0' \
  --config 'profile.release.package.crypto.debug=0' \
  test --release --locked --example h016_factor_base_audit -j 1
```

This override reduces optimization only for the root package. It keeps the
real root library and existing dependency settings. It is not a pass of the
unchanged default-release library gate. No test or source module was disabled.
The example also compiles the literal shared controls source through a
`#[path]` test-only module, whose arithmetic imports resolve to the production
`crypto_lib::cryptanalysis::prime_fast` module. Its `FastFactorBase` adapter
regression calls the actual production library methods.

## Focused result and public fixture

The focused command completed with **24 passed, 0 failed, 0 ignored**: 14 shared
module test groups and 10 CLI test groups. A final rerun added
`--example prime61_factor_base --example prime_ic_collection_frontier`; it also
exited 0. Those two existing examples each contain zero unit tests, so their
result is successful compilation/linking, not additional executed test cases.
The final combined output is
[`validation/native-tests-opt0.log`](validation/native-tests-opt0.log). Only its
final blank line was removed for repository whitespace rules; the original
stdout SHA-256 is `2dc83bf64d68d4c187063e6ce6e2493ad7e065125dfe70590ec8b0165c77af13`.
[pcc:unverified]

A separate cubic-loop oracle checks the rolling pair-table counter. It shares
the production group law but not the counting algorithm. The checks include
all 24 permutations of a support containing identity and an inverse pair,
identity targets without an inverse pair in support, repeated target weights,
coverage versus multiplicity, total tuple mass, mixed/closed support profiles,
51-point cardinality, identity/two-torsion fixed points, anchor signs, malformed
inputs and checked overflow. The actual `FastFactorBase` adapter is exercised.

The CLI was then built and executed with the same package-only optimization
overrides and the checked-in M31 fixture:

```sh
CARGO_BUILD_JOBS=1 timeout 180s cargo \
  --config 'profile.release.package.crypto.opt-level=0' \
  --config 'profile.release.package.crypto.debug=0' \
  run --release --locked --example h016_factor_base_audit -j 1 -- \
  --input research_notes/h016_negation_null_20261124/fixtures/m31_permutation.json \
  --output research_notes/h016_negation_null_20261124/validation/m31_permutation.audit.json
```

It exited 0. Both support orders returned `[1,1,0,1]`; both coverage and mean
multiplicity were `0.75`. The retained
[`m31_permutation.audit.json`](validation/m31_permutation.audit.json) includes the
exact input and source digests and keeps unmeasured rank/recovery/cost fields
null. Its SHA-256 is
`5a61075cdee249498c856b3d13b78865ee9dcd809119ae3b1d16792785bb6e74`.
The original output path now exists, so use a fresh output path for a rerun.
This was a deterministic correctness execution, not an H016 historical cell or
a random-control experiment. [pcc:unverified]

All eight candidate source/input hashes below were checked again after native
execution and matched. `rustfmt --check` on touched Rust files and
`git diff --check` are part of the final source checks. No full library test
execution is inferred from the focused result. [pcc:unverified]

## Other checks and remaining gates

- Source review found no blocking defect in the counter, actual support/profile
  admission, adapters or strict CLI. This is review evidence, not execution.
- The original three H016 evidence files have zero byte differences from their
  original commit. The sibling [ERRATA.md](ERRATA.md) contains the correction.
- `python3 tools/benchrec.py gate` was rerun after restoring all 410 tracked
  benchmark metadata/progress files (8,322,667 bytes). It exited 1 with 5,610
  findings across 403 records. Progress was no longer reported stale. The
  restored files have zero diff from HEAD; no records or plots were regenerated.
- That benchmark-history gate reports 4,468 missing raw inputs, 874 missing
  report/graph/validation files, 192 missing entry directories, 53 open records,
  21 missing S3 URI fields, one invalid validation metadata object, and one raw
  digest mismatch. Most dependencies remain outside this sparse checkout.
  This is a failed gate with incomplete historical inputs, not a green check
  and not evidence that this integration introduced those record problems.
  The compact receipt is
  [`validation/benchrec-gate-summary.json`](validation/benchrec-gate-summary.json).
- The unchanged default-release library gate remains unsatisfied. Focused
  component qualification cannot substitute for repository-wide validation.
- No measured timing series, priced leaderboard entry, rank improvement or
  scalar recovery is claimed. Historical rescoring still requires actual
  original supports and ordered targets. [pcc:unverified]

## Candidate source SHA-256

These digests identify the source candidate, including the test-only wrapper.
The unchanged primality source is included because the native input validator
calls it. The fixture is a deterministic public correctness input.

| Path | SHA-256 |
| --- | --- |
| `src/cryptanalysis/prime_factor_base_controls.rs` | `b7ba98151e4f6a0021c06479e0ff4d9926be3416c90fed729c6d466c9ed43f62` |
| `src/cryptanalysis/prime_fast.rs` | `6a9dc015b1558045935912688beebf141a6b4161c045c821faac81fc3a430bc1` |
| `src/cryptanalysis/mod.rs` | `4a44596c57588fe95df17b713141743cb499282e33b509d315dffb64c42e8cdf` |
| `src/cryptanalysis/residual_walk.rs` | `4e54e030cebfc873f9288e25c83153841724d1cc0d8c79417288aa9e74f507c9` |
| `examples/h016_factor_base_audit.rs` | `736b94972e585c73b54749c565d75ddf9b04d4b5167c76cbc0eaf3547a000529` |
| `examples/prime61_factor_base.rs` | `af432127a0e2cbda0fe36781b63475ebf0fdc91c2241e467462d21f0ea7de56a` |
| `examples/prime_ic_collection_frontier.rs` | `38fc68165d2bc80faec3b458930b416d9fc0dd7bd4a89120d6c207d80b81c0f9` |
| `research_notes/h016_negation_null_20261124/fixtures/m31_permutation.json` | `e2fce6cdc36c06a7a7c81044445460e640e8e8db10408f5efe4b829c82f336a6` |

## Frozen historical SHA-256

| Path | SHA-256 |
| --- | --- |
| `scripts/l23_h016_negation_null.py` | `79464fdda8092ff12ad8352c7fa01c8059bddaba0482ebd36d31debcd50d2eb0` |
| `research_notes/h016_negation_null_20261124/README.md` | `70064da52a56ffa859229ee6fdd7df297d99cdfbc189869c943eb09fcd2093c2` |
| `research_notes/h016_negation_null_20261124/l23_rescore.json` | `da684e82b1deea6b5ec3893651b06fe8dfcb434a211945c0dae2e413eff07c1c` |
