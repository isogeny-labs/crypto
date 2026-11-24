# Production comparator integrated; full-base calls reach limits

**2026-11-24 · Local source and correctness qualification · [pcc:unverified]**

The Dickson harness now calls Isogeny Labs' existing compact quotient-product
extractor through an optional production adapter. The three predeclared panels
retained thirteen resource-limit results: none completed under the fixed caps.
A separately added planted four-leaf unit test returned the exact signed set
through the real production solver and adapter. It does not replace the failed
full-base panels. No cost ratio follows from this component.

![The production adapter preserves incomplete status and separately replays accepted rows.](production-adapter.pdf){width=85%}

## What was integrated

The adapter invokes `prime31_four_summand::extractor::extract_cofactor`, with
the existing native F4, root extraction and signed lifting. The caller passes
only public curve, base and target data. The original production closure copies
complete F4 files and selects other dependency intervals byte-for-byte; its
[build inventory](evidence/production_v1/production_source_manifest.json) is retained.
Three stale dependency pins were refreshed after reviewing every affected
selected interval. No solver algorithm was rewritten. The
[protocol](PRODUCTION_ADAPTER.md) records the exact changes and prior sources.

Accepted native rows must have four distinct valid indices with signs ±1 and
must replay to the target in the separate Dickson harness arithmetic. An
incomplete status never becomes an exact answer, even if partial rows exist.
The full production relation set and its regular-image projection are distinct
outputs. The candidate's narrower chart cannot claim the baseline's coverage.

\newpage

## Fixed full-base outcomes

All calls use the executable frozen from source `8be878c873`, the existing
public fixtures, and a three-second enclosing cooperative deadline. Native
limits are degree 10, 20,000 construction terms, 4,096 assignments, 20,000
pairs, 8,192 symbolic monomials and 2,000,000 matrix-workspace entries. Each
panel has one worker, a 2 GiB address-space ceiling, and a 60-second outer
timeout. There were no cap increases or panel retries.

| Fixture / selector | Base size | Variables / equations | Complete | Observed limit |
|---|---:|---:|---:|---|
| F127 / degree 16 | 8 | 7 / 8 | 0 / 4 | Symbolic terms |
| F8191 / degree 16 | 7 | 6 / 7 | 0 / 4 | Symbolic terms |
| F8191 / degree 32 | 14 | 13 / 14 | 0 / 5 | Matrix workspace |

Every input system has maximum total degree three. The first panel refused
symbolic admissions 8,239 / 8,256 / 8,252 / 8,252 against 8,192. The second
refused 8,200 / 8,200 / 8,198 / 8,199. Every depth-five query refused matrix
admission 2,000,502 against 2,000,000. The native diagnostic records zero
executed forbidden domain evaluations on all thirteen calls. All three panel
processes exited zero after recording these incomplete outcomes.

| Fixture / selector | Full direct-reference row counts | Refined regular-chart counts |
|---|---|---|
| F127 / degree 16 | 9, 10, 13, 13 | 1, 0, 3, 3 |
| F8191 / degree 16 | 0, 1, 0, 0 | 0, 0, 0, 0 |
| F8191 / degree 32 | 1, 5, 2, 3, 4 | 0, 0, 0, 0, 1 |

The left column is the test-only direct reference, not a native cofactor result.
The right column is retained from the earlier Refined panels. The difference
shows why completed-query counts alone cannot compare these two scopes.
No complete native regular projection was available from this new panel.

[F127 raw records](evidence/production_v1/f127_d4.jsonl) ·
[F8191 degree-16 records](evidence/production_v1/f8191_d4.jsonl) ·
[F8191 degree-32 records](evidence/production_v1/f8191_d5.jsonl) ·
[Pre-call identity](evidence/production_v1/pre_call.txt) ·
[Commands](evidence/production_v1/commands.txt)

## Validation and next step

Before the panels, nine adapter tests, all 32 production-closure library tests,
and eight public-example tests passed. The six original default-feature tests
also passed. After the panels, a separate test restricts the existing F127
positive fixture to its four expected leaves and calls the actual cofactor
solver under the same caps. Its complete signed set matches the independent
direct reference after conversion. The final feature-enabled suite has ten
passing tests. This is a planted integration control, not a natural full-base
query or a replacement of the failed calls.

[Adapter tests](evidence/production_v1/adapter_tests.txt) ·
[Production library tests](evidence/production_v1/original_library_tests.txt) ·
[Public-example tests](evidence/production_v1/public_example_tests.txt) ·
[Post-panel integration test](evidence/production_v1/post_panel_integration_tests.txt)

Claims `PCC-20261124-e7111b`, `PCC-20261124-4c3603`,
`PCC-20261124-a5c9c4`, `PCC-20261124-c7baf7` and
`PCC-20261124-f6947c` retain unsigned, unverified standing. The earlier
root-library compilation timeouts remain documented in
[DEVELOPMENT_CHECKS.md](DEVELOPMENT_CHECKS.md); no root-suite pass is claimed.

The production adapter is implemented, but its full-base complete-output gate
is still open. A comparable successful control must cover the same relation
scope, and the original oriented-pair S3 component still needs a complete
four-summand interface. Full cold setup, failed calls, collection, linear
algebra and scalar verification must then be charged on an admitted workload.
No EKS job was submitted. The [comparison problem](../../problems/refined-rr5-cold-comparison.md)
tracks those requirements. The Cloudflare hook has buffered the thirteen new
records locally; external delivery still needs the receiving URL.
