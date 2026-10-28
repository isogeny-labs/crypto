# Partial artifact software validation

## Controller contention fix — source 7d2aad8

Runtime source `7d2aad8baa5fa5851e3f2ac16b6024cd3999210a` passes **5,130
full-library tests, zero failures and 123 ignored**, all 25 focused budget
tests, strict Rust 1.98 release-library Clippy and scoped formatting. The
source-bound result is in `validation-7d2aad8.json`.

Distinct workers now wait up to 30 seconds for the controller lock, retrying
contention and interrupted calls while preserving other errors. A worker's
session lock remains immediately exclusive. Path identity is checked across
the wait; the controller is released before worker-journal replay. The limit
bounds retry time, not a blocked filesystem syscall. Regression tests exercise
disjoint allocations, reserved credits, interruption, timeout, missing and
replaced paths, and session exclusivity.

The full suite ran from a fixed test executable with SHA-256
`4fb48e0d7ba537067324e87cf8b74711c8deeea56cac4c9448a288ce9d5c1d78`
using `RAYON_NUM_THREADS=16` and `--test-threads=1`, finishing in 1,616.44
seconds. That is shared-host validation duration, not benchmark evidence.
The pre-migration run ended during host shutdown without a terminal result.
The first migrated run exposed missing C++ tooling; its interrupted log and
isolated diagnostic are retained. After installing gcc-c++ and
perl-Digest-SHA, all six device-emulation checks and the full suite passed.
No source test was weakened or skipped.

The 172,671-byte final archive was conditionally uploaded and freshly downloaded
with matching bytes at 2026-10-28T13:37:38Z:

```text
s3://setec-astronomy-artifacts/ml-dataset/20261028/software/live-lockfix/5c9b23c58de04f17878c20a964666ac42f51e23b795f516de2b61ab57c5ab2aa/validation.tar.gz
```

It preserves the successful and interrupted logs, diagnostics, source patch,
source/binary digests and terminal validation metadata. Its SHA-256 is
`5c9b23c58de04f17878c20a964666ac42f51e23b795f516de2b61ab57c5ab2aa`.
See `validation-7d2aad8-archive.json`. This transfer uses the controller's
existing credentials; actual dataset-worker S3 access remains unverified.
The future main/persistence merge must separately preserve main's exact-prefix
absence probing, cumulative admission and scoped-head integration fixtures,
then pass its own validation. This result does not validate that future source.

## Earlier partial-artifact implementation — source ae0683c

The tested runtime source is `ae0683c2edeee9066d696af9d0ca3e41ad9c4a24`.
Rust 1.98.0 release compilation, strict Clippy, scoped formatting and the
library/CLI tests pass. The final full library run passed **5,122 tests with
zero failures and 123 ignored**. Earlier failures remain recorded below.

| Check on ae0683c | Result |
|---|---|
| Release library, crypto binary and two integration-test builds | Pass |
| Full library on a fixed copy of the ae0683c test executable | 5,122 passed, 0 failed, 123 ignored |
| Dataset library | 116 passed, 0 failed, 1 ignored |
| Store library | 46 passed, 0 failed |
| Dataset CLI processes | 7 passed, 0 failed |
| Store CLI processes | 5 passed, 0 failed |
| Clippy, release lib/bin crypto/two integration tests, `-D warnings` | Pass |
| Scoped Rust formatting | Pass; installed stable formatter, pinned 1.98 compiler |
| Dataset and generate help | Pass |

The CLI fixtures use a directory remote and an executable sentinel that rejects
any accidental AWS invocation. They exercise initial and incremental snapshots,
fresh artifact-only restore after deleting the local source/cache, torn-tail
and orphan exclusion, stable retries, identity tampering, local and remote
corruption, and missing/incomplete S3 budget arguments. Their deliberately
invalid curve payloads ensure that artifact restore cannot depend on algebraic
replay. They are not native scientific datasets or an S3 deployment test.

## Broad library run and retained failures

The first complete release library run, on implementation source
`c87174cddc02a47da0fdac7312e955db599bd1b2`, finished with **5,121 passed,
1 failed and 123 ignored**. The existing
`cryptanalysis::wdsat_oracle::tests::mock_child_exit_and_empty_output_are_not_refutations`
test exceeded its 2,000 ms child timeout on the shared host. An immediate
isolated rerun of the same binary passed in 0.06 s. No unrelated test was changed
or disabled. A full retry with `RAYON_NUM_THREADS=4` and `--test-threads=1`
finished with 5,119 passed, 3 failed and 123 ignored. The three failures were
ENOENT errors when existing tests spawned `std::env::current_exe()` after a
concurrent Cargo build replaced the running executable. The WDSAT test passed
in that retry.

The final ae0683c test executable was copied to a fixed path outside Cargo's
output directory, with SHA-256
`a8aa98c49fdac3d42a24e0a0ddfb38b413ababda23744897b6249289d34823b6`.
All three subprocess tests passed individually against that copy. Its full
run, with `RAYON_NUM_THREADS=16` and `--test-threads=1`, then passed all 5,122
non-ignored tests in 1,346.07 seconds. This is validation duration on a shared
host, not a performance measurement. No test was removed or weakened.

Since c87174c, runtime source changed only by expressing the snapshot-chain
predecessor check as a guarded match for Clippy. The additional changes are CLI
tests and documents. The focused checks above compile and exercise the final
ae0683c source. The earlier failed formatting-lint attempt and initial
disallowed-bucket error-precedence regression remain in the retained logs;
current-source checks pass after those corrections.

## Durable software archive

The archive contains the release CLI, scoped source, exact source/binary
digests, validation metadata, successful logs and the earlier failed logs.
The release CLI SHA-256 is
`f91def741dcddc6c5a39609c4b916d0210c8b30342c20e4552a9b145f474a82a`.

```text
s3://setec-astronomy-artifacts/ml-dataset/20261028/software/live-artifacts/c484c30d6dae5ce391321deaf29c77c8d3743a9779ebc1664a4063ed60d05b30/validation.tar.gz
```

The 10,443,794-byte archive passed a conditional PUT, fresh GET, SHA-256/length
check and byte comparison at **2026-10-28T10:47:37Z**. Its SHA-256 is
`c484c30d6dae5ce391321deaf29c77c8d3743a9779ebc1664a4063ed60d05b30`.
This software transfer used the controller's existing credentials; it does not
prove the dataset worker's S3 permission or cloud interruption recovery.

The separate final validation archive preserves all three broad-run logs,
the isolated reruns, final focused checks and source/binary hashes:

```text
s3://setec-astronomy-artifacts/ml-dataset/20261028/software/live-artifacts/93cc8928f1eece8106c8db993a56ca1580d446213b0ecc0a7b5667c32e29fc51/final-validation.tar.gz
```

Its SHA-256 is
`93cc8928f1eece8106c8db993a56ca1580d446213b0ecc0a7b5667c32e29fc51`.
The earlier software archive retains its original, accurately timestamped
pending-run status. See `validation-final-ae0683c.json` and
`validation-final-archive.json` for the terminal result and readback receipt.

## Integration and deployment limits

The review base is exact unlanded producer snapshot
`0f55f1c333449d4c85f04448da16fbfec15249cb`. At 11:20 UTC, PR #2756 had
advanced to `0cbf720bf39a85e624bdff9d1d572ea55edb10b7` by merging main into
its earlier `d0334808999e039d95bfea5137d0460db2868aea` head. It still does
not contain the inherited 0f55f1c producer-fix lineage. These are different
versions and require integration review.
A separate review-base ref preserves the narrow diff without changing the
producer owner's branch. Reconcile its inherited fixes and main's separate
store absence-probe change before retargeting and validating a main merge.
Do not merge the review PR into the temporary review-base branch.

The [deployment audit](DEPLOYMENT.md) still requires worker-role approval,
an independently durable mounted quota authority, actual-worker publication,
fresh restore and independent scientific replay. The **USD100–200 total
through tomorrow, maximum USD200**, budget remains unchanged. Byte/request
counters do not enforce account-wide dollar spend. No new cloud trial,
experiment retry, solver-memory checkpoint, completed corpus, fall-degree
bound or performance/security claim follows from these software checks.
