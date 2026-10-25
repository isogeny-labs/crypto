# Primary N83 Linux XOR binary and input preflight

This workflow qualifies a Linux ARM CryptoMiniSat executable and the pinned
external-model verifier on the complete primary `a=0` 11-object S3 input
bundle. It performs only a two-variable XOR smoke, a worker build, an
independent S3 fetch/replay and the existing incomplete-model verifier gate.
It does not run a natural N83 relation query. Solver version and binary hash
are observations from this preflight; a later comparison must freeze their
exact values, full source and image identity, host envelope, arm order, caps,
and A/A controls in a separate committed protocol before timing.

The worker source is the immutable `acfcd8adfad8d45ddf9c5f799f9385120dce6cf4`
commit from PR #2551, which includes `verify-cross-xcnf`. The preflight
protocol comes from the dispatch commit on PR #2557. The Rust image is pinned
by digest, while the Ubuntu package is deliberately discovered and recorded.
Completed-stage binary bytes, build logs, smoke streams and hashes,
host/image/source IDs, S3 replay receipt and verifier gate are retained in
one Actions artifact. A failure before a stage begins leaves its outputs
absent and cannot be interpreted as a pass. The factor-base archives remain
in S3.
The [first attempt](RESULT.md) stopped at its first panel-input S3 GET after
the solver smoke and worker build passed. Its failure artifact is retained and
audited in both GitHub Actions and a write-once S3 output object; the next
push records the exact AWS caller and GET error before building again.

The first push of this workflow and smoke script to
`codex/n83-aws-handoff-20261025` triggers the qualification job. The push
path filter excludes later evidence-only commits, avoiding an accidental
second calibration. Inspect the run with
`gh run list --workflow n83-linux-xor-preflight.yml --branch codex/n83-aws-handoff-20261025`.

Inspect the exact run and artifact, verify all retained receipts and rerun
the S3 fetch audit before treating its Linux binaries as inputs to a paired
experiment. A package or flag mismatch is a failed qualification, not a
relation outcome. `selected_best_total_runtime` remains `null`.
