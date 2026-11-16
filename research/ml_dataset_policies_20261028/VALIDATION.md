# Policy and prime-root implementation validation

Runtime source `d71ec2ef5ab9b7f2a038f61c941af10297a775ad` passed the required
release library suite: **5,169 passed, zero failed, 123 ignored**, in 1,734.35
seconds. This is software validation, not an experimental measurement of
fall degree or a completed training corpus.

The same source passed 19 binary-field tests, eight prime-root tests,
153 dataset tests (one ignored), eight native CLI integration tests, strict
Rust 1.98 release Clippy for the library, `crypto` binary and CLI integration
target, scoped formatting and whitespace checks. The full suite used a fixed
copied executable with 16 Rayon threads and one test thread. Executable hashes
and detailed scope are recorded in [validation-d71ec2e.json](validation-d71ec2e.json).

The corrected binary-field counter charges every executed batched squaring.
Its regressions compare batching against repeated squaring and check inversion
identities and the expected `m-1` squarings. Arithmetic is unchanged. This
calling-thread tally includes the adapter and replay operations that invoke
the instrumented APIs; it is not a count of all optimized operations or machine
instructions. Historical measurements are not rewritten.

The source-bound producer binary generated 72 bounded software fixture cells.
The separate consumer audited all 72 and independently checked 22 positive
PDP certificates. Unsupported policies retain null features. Its static
policy view has a separate contract from producer training view v4. Consumer
root-set verification and equality of the two training views remain false.
The separately preserved fixture archive and source revision are named in the
validation record; these checks do not prove worker publication in EKS.

## Durable evidence

The final archive contains the terminal logs, source file hashes and patch,
source-bound CLI, executable checksum records and validation metadata:

```text
s3://setec-astronomy-artifacts/ml-dataset/20261028/software/policy-corrected/2d8d88c585e5da91e13464319c20351b23c7bed39c7b367117c8424debc25a16/validation.tar.gz
SHA-256: 2d8d88c585e5da91e13464319c20351b23c7bed39c7b367117c8424debc25a16
Bytes: 10414676
```

Conditional PUT, fresh ETag-bound GET, byte comparison, extraction and all
contained hashes passed at `2026-10-28T14:32:10Z`. The archive receipt and raw
PUT/GET metadata are adjacent to this file. The earlier source checkpoint,
original validation and failed preflight evidence remain separately preserved.
An initial packaging attempt found a tracked handoff absent from the sparse
checkout; packaging now hashes immutable Git objects at the source commit.
No missing tracked file was treated as absent evidence.

## Remaining gates and claims

This branch is stacked on persistence PR #2769 and needs reconciliation with
the producer lineage and current main before landing. Exact-head hosted checks
and review are separate from these local tests. The parent hosted validation
attempt failed during a checkout transport error before any test ran; only
that failed job was retried on 2026-10-28.

The independent durable worker quota mount and worker S3 grant remain
unresolved. No actual dataset worker, new scientific campaign, E12 retry, IAM
grant or deadline extension was launched by this validation. The USD 200 total
campaign ceiling is unchanged. The extensive solved corpus, complete execution
traces, total solver costs and general first/last-fall-degree questions remain
open. The implementation's literature and original design are in [DESIGN.md](DESIGN.md).
