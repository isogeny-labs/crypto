# Root-phase worker build receipt

The accepted native worker was built from clean execution source commit
`43c9850a73f1d960f65ed470fdb8adac981a5ffe` in the offline arm64
`rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`
image. The source snapshot contains the six files listed in
[`source-files.sha256`](source-files.sha256); each compiled source byte
matched the checkout and read-only snapshot, as recorded by
[`source-check.log`](source-check.log) and
[`source-snapshot-check.log`](source-snapshot-check.log). The five retained
inputs match [`inputs-before-run.sha256`](inputs-before-run.sha256) and
[`input-check.log`](input-check.log). The byte-pinned signed-Frobenius
object and generic replay are inherited from the K=16,627 construction
study; this arm does not reconstruct it.

The final offline Docker build had exit code 0 and `OOMKilled=false` from
05:10:22 to 05:10:29 UTC. Its native release worker SHA-256 is retained in
[`worker-binary.sha256`](worker-binary.sha256); the exact container state
and compiler output are [`linux-build-state.json`](linux-build-state.json)
and [`linux-build.stderr.log`](linux-build.stderr.log). The 6.95-second
incremental build interval is a build receipt, not a cold performance
comparison.

Two implementation attempts failed before any degree-83 root-phase run:

| Source revision | Failure | Preserved evidence |
| --- | --- | --- |
| `b1a45be3c1797e121f50f7fa0ae72ec55e799888` | exit 101, no OOM: one oversized `json!` receipt exceeded the exporter macro recursion limit | [`compiler log`](linux-build-compile-failed.stderr.log), [`state`](linux-build-failed-state.json), [`source manifest`](pre-fix-source-files.sha256) |
| `6fd4808d82678f30117e3b41268699dc20666906` | exit 101, no OOM: a pre-existing capacity receipt binding was incorrectly made immutable during the fix | [`compiler log`](linux-build-mutability-failed.stderr.log), [`state`](linux-build-mutability-failed-state.json), [`source manifest`](pre-mutability-fix-source-files.sha256) |

The first sandboxed Docker attempt could not access the local socket;
[`linux-build-sandbox-denied.log`](linux-build-sandbox-denied.log) retains
the exact access error. None of these failures is a SAT result or a
factor-base verdict.
