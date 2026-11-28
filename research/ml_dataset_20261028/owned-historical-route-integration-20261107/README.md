# Historical checkpoint replay and restricted export

The explicit `original_owned_search_historical_rows_v1` replay mode admits retained rows from one complete checkpoint of an interrupted, checkpoint-enabled original search. It preserves a distinct `HistoricalCheckedScope` verdict and `crypto.ml-dataset.owned-search-historical-rows/v1` projection. Generation and cell dispatch are refused under this replay policy.

Admission requires the original request/map/context and exact raw manifest/outcome, an allowed supervisor interruption envelope, and a fully authenticated capture. Every complete snapshot is identity-bound. Only a final truncated snapshot can pass strict prefix parsing and the same identity binding. The selector chooses once, then original descent and lineage/basis verification operate on the selected complete snapshot. A mathematical failure cannot trigger fallback to older rows. One retained replay budget and deadline cover all stages. Revalidation retains and checks every chunk including the tail; StorageFailure is not an allowed capture stop.

The projection separates selected checkpoint and terminal capture receipts, retains tail descriptors, and reports the producer attempt incomplete and the producer report join unverified. Original-descent and row-identity scope is explicit. Degree labels remain null. Historical target/record IDs, statuses and summary counts are separate from ordinary report-joined rows.

## CLI wiring

The existing global replay flags accept the new mode in their policy JSON:

```sh
crypto dataset --owned-replay-policy /absolute/policy.json --owned-replay-receipt /absolute/receipt.json export --out /absolute/run --destination /absolute/export
```

The policy retains schema `crypto.ml-dataset.owned-replay-policy/v1` and requires `mode`, the exact `run_sha256`, matching absolute `receipt_path`, ordered unique `cell_indices`, finite `seconds`, per-cell work/bytes/stream/payload allowances and aggregate work/byte ceilings. Receipts are write-once; existing start/terminal receipts require reconciliation rather than retry. This command wiring is source-reviewed, not a completed CLI run.

## Validation and remaining work

Twenty-one central controls passed (eight exact installed decode-method controls plus thirteen inherited selector tests), using actual wire/prefix/identity/selection modules with structural map/capsule/budget substitutes. Ten caller/export controls passed using extracted actual routing methods and documented custody/run-I/O substitutes. Source parsing and independent review passed. Full assembled typechecking, authentic-original-cell historical export, and executed selected-mathematical-failure integration remain unverified. No scientific/cloud jobs or deadline extension occurred.

The build preparation materialized missing tracked dependencies and froze 1,467 input hashes in ASSEMBLED_SOURCE_HASHES.json. The next full local build requires its own bounded authorization; no build was run here. Source review and component tests do not substitute for it.

S3_RECEIPT.json identifies the complete immutable evidence archive. Git log copies normalize terminal whitespace; packet manifests describe original archived bytes. Prior methods and claims are recorded in the chained prerequisite/descent/checkpoint packets and the included independent review.
