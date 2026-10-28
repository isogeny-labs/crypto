# Partial artifact software validation

The tested runtime source is `ae0683c2edeee9066d696af9d0ca3e41ad9c4a24`.
Rust 1.98.0 release compilation, strict Clippy, scoped formatting and the
affected library/CLI tests pass. The broad library run and its retry are
recorded separately below; this record does not claim a clean full-suite run.

| Check on ae0683c | Result |
|---|---|
| Release library, crypto binary and two integration-test builds | Pass |
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
or disabled. A full retry with `RAYON_NUM_THREADS=4` and `--test-threads=1` is
still running at this record's publication; its completion is not inferred
from the isolated pass.

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

## Integration and deployment limits

The review base is exact unlanded producer snapshot
`0f55f1c333449d4c85f04448da16fbfec15249cb`. PR #2756 currently publishes
`d0334808999e039d95bfea5137d0460db2868aea`; those are different versions.
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
