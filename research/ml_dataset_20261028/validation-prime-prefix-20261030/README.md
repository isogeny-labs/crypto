# Prime-chain causal replay software evidence

Current source `adde10cad21b8179ec358af81827eb76073abdaa` passed29 focused release
library tests (0failed,0ignored), including the existing exporter checks and new
certificate/root/replay tests. This is partial software validation. Strict
maintained Clippy is in flight; full-library, actual CLI and current-main combined
validation remain pending. Initial source5d4336b45d2 failed compilation because of
a bounded-reader ownership error; its log and exit101 receipt are retained.

[store-index.json](store-index.json) names immutable native artifact-store objects
under `s3://setec-astronomy-artifacts/store`. [initial-readback.json](initial-readback.json)
records fresh download and byte comparison of the first five completed artifacts.
Use `CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store` and
`CRYPTO_STORE_INDEX=<absolute path to store-index.json>` with `crypto store pull`
to restore a named object. No Python code executes cryptographic replay or these
new CLI checks; `validate_prefix.sh` is thin shell process/log bookkeeping.

The source checkpoint readback is retained separately in
[../prefix-initial-checkpoint-readback.json](../prefix-initial-checkpoint-readback.json).
Its Git bundle contains source5d4336b45d2 and requires parent764410c3af5. Later
commits need a later checkpoint. A checkpoint is not corpus-generation evidence.

Scientific samples added:0. Verified cloud dataset cells:0. No trained model,
exact fall degree, native full-trace replay, whole-solver instruction total or
new ECDLP result is established. The original objective remains in
[remaining coverage](../../../docs/ml-dataset/REMAINING-COVERAGE.md).
