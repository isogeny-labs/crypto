# Native Boolean F4 suffix trace validation

The bounded native trace implementation and its local software fixtures passed the source-bound validation below. This is an optional single-call Boolean matrix trace API, not a dataset-adapter integration, a completed scientific corpus, a performance measurement, or a first/last fall degree result.

## Source and behavior

Base: `c14d4552e513f6e28bda0e0e045f5e64beb492db`, branch `codex/ml-binary-root-trace-20261028`, parent PR [2782](https://github.com/isogeny-labs/crypto/pull/2782).

Tested source: `29973950752cd0e2389a07608eee63781841afc2`. Runtime implementation first committed at `1e22241dab67a236eb954f93c75ab38d228a8101`; the final source changes only fixture-helper tests and schema clarification. Later validation/documentation commits change no runtime, build or test source.

Executing release library-test binary SHA-256: `e368482f8f8d56070d19e50c51aad1948d3238d1d579776f6c54e7f1243d4501`. Sources and executable are preserved in the accepted source archive below. Source/configuration/context/call identifiers in a stream are caller declarations, not execution attestation.

The profile admits canonical Boolean inputs with at most eight variables, degree bound at most four, at most 64 generators, and every input term within the declared degree bound. It rejects unsupported selected layouts before native work. The traced optional observer preserves the native Macaulay row-generation and suffix-elimination kernels and existing untraced wrappers. It emits bounded acknowledged JSONL prefixes with sticky sink failure, cancellation and cumulative event/byte/workspace/admission caps. The named admission units and executed suffix-word XOR count are not total instruction counts.

## Completed validation

| Gate | Exact observed result |
| --- | --- |
| Final source release library suite | 5,197 passed; 0 failed; 125 ignored; exit 0 |
| Final source focused trace tests | 14 passed; 0 failed; two explicit helper tests ignored by the ordinary filter |
| Strict release Clippy | Passed for library, tests and crypto binary with warnings denied |
| Prior native RREF regressions | 12 passed; 0 failed; four opt-in measurement tests ignored; exact runtime files unchanged at final source |
| Explicit source-bound native fixtures | Five complete streams and one row-cap-zero censored prefix |
| Independent checker | Five independent ranks 0, 1, 1, 2, 92; censored prefix accepted without a final rank |
| Actual suffix-boundary coverage | 296 rows, 92 columns; 68 row XORs with nonzero start-word offset |
| Rehashed corruption checks | Four multiword arithmetic mutations rejected |
| Checker CLI/library agreement | All six reports byte-identical |

The full-library elapsed 1,375.67 seconds is a test-run diagnostic only. No additional Cargo run, fixture generation or cloud experiment was performed while archiving or preparing delivery.

The separate checker is delivered in [PR 2800](https://github.com/isogeny-labs/crypto/pull/2800), runtime `2ca93f74a00a5ff133aeaf5cab772f616a110791`, CLI SHA-256 `3e294a77fcef48ad39462a90c67a7da5bc55996fd70282363823a70183603f3d`. It defines independent wire types and arithmetic, replays the full candidate/column/packing/elimination schedule for complete streams, and computes scalar GF(2) rank independently. A source review initially missed the native legacy cap+1 candidate. Its correction and actual censored native fixture are retained; the earlier nine-test checker archive is not current acceptance evidence. The final standalone checker suite passed ten tests, with its opt-in interoperability helper subsequently invoked explicitly.

Parent source review of both runtime files against the base found no actionable producer defect; the review is preserved separately from execution evidence. Producer stream flags for independently verified rank and complete execution remain false. Independent rank in a checker report does not rewrite producer flags.

## Durable evidence and recovery

All locations are in `setec-astronomy-artifacts`, region `us-east-2`. `VALIDATION.json` retains exact receipts, hashes, sizes and verification times.

- Accepted source, executable, focused tests, lint and six native fixtures: `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/boolean-f4-suffix-trace/53f2f9891e03acfb6eeaf37dac25ade84bd007fb1fbeb1266ded8e0b05e0ac5e/source.tar.gz` — 37,533,614 bytes, 62 files. Its record correctly says the full gate was pending at publication; it is not rewritten.
- Completed full-library log, exit status, source bundle, review and independent-replay references: `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/boolean-f4-suffix-full-gate/e5dbfcf4a2569ece526bf29d3f4cf28b8e3326837ebc13f06684c5e00cb4f8c2/validation.tar.gz` — 122,441 bytes, verified 2026-10-28 18:06:38 UTC.
- Independent checker source, binaries, native fixtures and replay reports: `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/boolean-matrix-checker-final/45f0743e02a7a01920e553cc389761685bd838a6d6b96f10ddc47bd8e7628c0e/validation.tar.gz` — 1,493,694 bytes.
- Parent source-review packet: `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/delivery-status/0826091163bf872f2bf146a3e51ca86b47cfe566aa159a1ded1a57892324827a/delivery.tar.gz` — 15,411 bytes.

Accepted archives used conditional PUT, fresh ETag-bound GET, exact hash/length/byte comparison and contained-file hash validation. The native source and full-gate packets also verified the downloaded incremental Git bundle. Bundle recovery requires the pinned base commit already present in the recovery repository. Verify the outer SHA-256, extract ordinary relative files, check SHA256SUMS and run `git bundle verify` before importing the bundle. No restore of opaque solver memory or measured machine execution is implied.

One preceding upload is explicitly quarantined and unaccepted: `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/boolean-f4-suffix-trace/c6fb086c64b1bb503e7305313af601f8e96d8e9c405122ded8c0ad714dead9a7/source.tar.gz`. Its overly broad file scan copied a then-live full-library log. The fresh-readback exclusion assertion failed; the object and failed assertion remain immutable. The accepted replacement used an explicit completed-file allowlist and excluded that live file. Only the later terminal full-gate packet above includes the completed full log.

## Claim boundaries and remaining work

The source trace does not certify PDP input encoding, complete solver execution, all operation cost, UNSAT, a complete Groebner basis, F5 criteria, first/last fall degree, a universal degree ceiling, or hardware performance. Prime F4, higher-m systems, unsupported layouts and adapter integration remain outside this increment. Static model features have not been extended with trace state or outcomes.

Hosted current-head CI/review and bottom-up integration are separate gates. This PR is stacked on the binary-root branch; auto-merge is unavailable for that feature base and should be considered only after the parent lands and exact-head checks are re-evaluated. No cloud worker, experiment retry, IAM mutation, budget increase or deadline extension was introduced. The original USD200 total cap remains unchanged.

The design, literature review, native-source comparison and exact bounds are in APPROVED-DESIGN.md, FROZEN-CONTRACT.md, SCHEMA.md and ADMISSION.md. No new mathematical method or novelty claim is made here.
