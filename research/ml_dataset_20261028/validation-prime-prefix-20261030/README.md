# Prime-chain causal replay software evidence

Initial validated prefix source `adde10cad21b8179ec358af81827eb76073abdaa` passed29 focused release
library tests (0failed,0ignored), including the existing exporter checks and new
certificate/root/replay tests. Strict maintained Clippy also passed on that source.
Source `06773dcf28f` passed the release CLI build and four actual export/retry
commands on the frozen retained curve/twist fixture. It exported four verified
branch prefixes for the curve and no branch prefixes for the inconsistent twist
root. Artifact readback, feature identities, restricted label joins, immutable
retries and unchanged input inventories passed. This remains partial software
validation.

Combined source `113526b577044a6283be81acb1a847c89c96aa4e` imports main
`bd4c2fe5e6fc5c12cbab17e3cebb8e4c2ba83991`. Its first validation failed because a
sparse local Cargo dependency was absent. Attempts two and three failed on 42
missing compile-time fixtures. The first scratch restoration helper omitted the
input redirection to its read loop and restored zero files; version two corrects
that bookkeeping error and restored 126 hash-checked blobs (34,801,774 bytes)
from the exact combined revision. These failures and both helper versions are
retained. Attempt four passed **236 dataset tests, 0 failed, 1 ignored**, including
both added regression tests, with exit code 0 and matching runtime-source
readback. The combined full release library suite subsequently passed
**5,441 tests, 0 failed, 123 ignored**, with exit0 and matching runtime-source
readback; see [full-library.receipt.json](full-library.receipt.json). Maintained
Clippy, release CLI build and all four combined-binary export/retry commands also
passed with exit 0 and matching runtime-source readback. The final
[pipeline receipt](pipeline.receipt.json) records completion of
[combined-validation-plan.json](combined-validation-plan.json). See
[clippy.receipt.json](clippy.receipt.json), [build.receipt.json](build.receipt.json)
and [cli.receipt.json](cli.receipt.json). Public feature identities, restricted
label joins, immutable retries, artifact readback and unchanged input inventories
passed on the retained curve/twist fixtures.
The ignored test subsequently passed all **360 endpoint combinations** when run
explicitly against the same combined-source binary (one test, zero failures,
exit0). Its binary hash matched before/after and runtime-source readback matched.
See [endpoint-validation-plan.json](endpoint-validation-plan.json) and
[endpoint-validation.receipt.json](endpoint-validation.receipt.json). It covers
n23/61, all three families, both twists, m2/4/15 and ten adapters with a four-point
factor base, work limit64, degree bound2 and private witness removed. It checks
structured outcomes and returned solution certificates; unsupported/capped cells
remain non-solutions. It does not exercise supervised wide-F4 admission or the
entire requested size/arity cross-product. Its separate
[endpoint artifact index](endpoint-store-index.json) avoids concurrent writes
with the full-suite archiver. All pipeline phases are now terminal and passed.

Initial source5d4336b45d2 failed compilation because of
a bounded-reader ownership error; its log and exit101 receipt are retained.

[store-index.json](store-index.json) names immutable native artifact-store objects
under `s3://setec-astronomy-artifacts/store`. [initial-readback.json](initial-readback.json)
records fresh download and byte comparison of the first five completed artifacts.
[pre-cli-readback.json](pre-cli-readback.json) extends this to nine artifacts and
10,455bytes, including lint evidence, the native-store archiver and the current
worker-role policy readback. [restoration-readback.json](restoration-readback.json)
extends this to 26 freshly downloaded artifacts and 43,204,772 bytes, including
the premerge CLI binary/output and the failed combined attempts. Later phases
add separate immutable receipts to the same index.
[dataset-pass-readback.json](dataset-pass-readback.json) records all
29 objects, including the combined dataset pass and finite validation driver.
The worker role's attached policy still names only
the legacy bucket; no worker S3 roundtrip or IAM modification is implied.
Use `CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store` and
`CRYPTO_STORE_INDEX=<absolute path to store-index.json>` with `crypto store pull`
to restore a named object. No Python code executes cryptographic replay or these
new CLI checks; `validate_prefix.sh` is thin shell process/log bookkeeping.

The source checkpoint readback is retained separately in
[../prefix-initial-checkpoint-readback.json](../prefix-initial-checkpoint-readback.json).
Its Git bundle contains source5d4336b45d2 and requires parent764410c3af5. Later
commits are included in [the combined-source checkpoint](../prefix-combined-checkpoint-readback.json):
head `37d6e8db1032`, fresh archive download and successful Git bundle verification,
requiring parent commits `764410c3af5` and `bd4c2fe5e6fc`. A checkpoint is not
corpus-generation evidence.

Scientific samples added:0. Verified cloud dataset cells:0. No trained model,
exact fall degree, native full-trace replay, whole-solver instruction total or
new ECDLP result is established. The original objective remains in
[remaining coverage](../../../docs/ml-dataset/REMAINING-COVERAGE.md).

Final pipeline archival is recorded in [final-readback.json](final-readback.json):
41 immutable objects, 89,046,821 bytes, each with a fresh S3 download and byte
comparison. This includes the combined binary, CLI output archive, all phase
logs/receipts and terminal pipeline log/receipt. The endpoint and portability
fixture indexes are separate and retain their own readback records.

A subsequent delivery review identified a row/term/degree-cap error-path mismatch in the
polynomial parser. The correction is governed by
[cap-fix-validation-plan.json](cap-fix-validation-plan.json). Earlier-source
passes above remain intact; they do not establish the follow-up's validation.

Follow-up runtime `d9a69910198d14a829b689253a62a7663b41d753` passed
**240 dataset tests, 0 failed, 1 ignored**, including all four new cap/error
regressions, then passed the explicit **360-case endpoint test**. Both phases
returned exit0 with matching runtime-source readback. See
[cap-fix-dataset.receipt.json](cap-fix-dataset.receipt.json),
[cap-fix-endpoint.receipt.json](cap-fix-endpoint.receipt.json), and
[cap-fix-scoped-readback.json](cap-fix-scoped-readback.json) for their four archived
logs/receipts and fresh S3 byte comparisons. The follow-up's strict Clippy,
release build and four-command CLI export/retry checks have also passed with
exit0 and matching runtime-source readback. See
[cap-fix-clippy.receipt.json](cap-fix-clippy.receipt.json),
[cap-fix-build.receipt.json](cap-fix-build.receipt.json), and
[cap-fix-cli.receipt.json](cap-fix-cli.receipt.json).
[cap-fix-cli-readback.json](cap-fix-cli-readback.json) retains fresh S3 byte
comparisons for these logs/receipts, the frozen binary and CLI output archive.
The full release library suite then passed **5,445 tests, 0 failed, 123 ignored**
with exit0 and matching runtime-source readback. The finite pipeline completed
successfully at2026-10-30T05:33:11Z; see
[cap-fix-full-library.receipt.json](cap-fix-full-library.receipt.json) and
[cap-fix-pipeline.receipt.json](cap-fix-pipeline.receipt.json).
[cap-fix-final-readback.json](cap-fix-final-readback.json) consolidates the
per-artifact fresh S3 download receipts and rehashes their retained downloaded
bytes against the exact artifact-index snapshot. No additional scientific
samples or cloud cells were produced.
