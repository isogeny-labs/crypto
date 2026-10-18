# Primary K4096 orbit-domain SAT capacity and bounded search

Freeze one follow-on gate for the independently replayed
[primary K4096 S3 object](../k4096-primary-20261018/RESULT.md): `a=0`, public-X
hash, seed `2026100801`, 4,096 signed-Frobenius columns, 679,936 points,
compressed BLAKE3
`a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779`,
compressed SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`
and point-set BLAKE3
`671c2d4d3e2f13276b474ae5e011d89c15084521eaa38df86ae989369395e3ba`.
Fetch the exact `crypto-autoresearcher/factor-bases/icv1/etc/` key afresh,
require its byte and metadata identity, and rehydrate the committed manifest,
generic-replay and upload receipts into one scratch panel. Use the frozen
public fixture-zero corpus and validation sidecar from the earlier K2048
natural gate; hash both anew. No object is rewritten or uploaded here.

The reused Linux x86-64 static worker has SHA-256
`21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed`.
Its exporter, primary adapter, S3-chain and curve-source SHA-256 values must
match the already retained K2048 build attestation before launch. Use the
locally verified immutable arm64 image
`sha256:0dd364ba7e10242f07755449e3a3d0e35f9efd987952737b90def6709ab0c5ce`.
The worker runs through the available binary-format emulation, so these
stage walls do not compare native IC throughput.

1. Under a clean committed source checkout, run one five-summand,
   identity-mask-zero `frobenius_orbit_phase` model-construction call with
   `max_variables=150000`, `max_domain_clauses=2000000`, one CPU, 4 GiB hard
   memory, zero swap, no network, read-only inputs and a 900-second outer
   wall cap. Retain its exact model counters, cgroup peak, source snapshot,
   raw worker/outer receipts and cleanup. The source-derived preflight
   predicts 131,958 variables and 1,773,110 orbit-domain clauses; this is
   a prediction, not the measured construction result.
2. Only on `PASS_model_construction_only`, run one natural public fixture-zero
   `primary-orbit-chain-cold` search with the same base, encoding and resource
   guard, one trial, no direct relation, 32 models at most, 5,000 conflicts,
   and a 1,200-second outer and worker wall cap. Retain all model, relation,
   rank, group-check and solver counters. Any cap remains `UNKNOWN`.
3. Independently audit the S3 input, committed source blobs, exact binary,
   both Docker configurations and cleanup, worker/outer receipts, public
   query and any group relation, plus a forged-receipt negative control.
   Preserve first-attempt failures and all raw logs.

The conservative new-stage envelope is 900 seconds for capacity, 1,200 for
conditional search, and 300 for fresh input preparation, audit and cleanup:
2,400 seconds maximum for this gate. The earlier K4096 object construction
and all prior sweep work are separate charges. Do not select a base from one
capped call. Natural useful yield, independent rank growth, linear algebra,
individual-log extraction, verification and a matched same-point rho run are
still required for a complete-runtime comparison. Keep
`total_index_calculus_runtime_ms` and `selected_best_total_runtime` null.
