# r30 terminal point-base custody and index validation

Job n83-fb-variants-r30-20261031-r1 completed successfully with producer exit 0. The Pod ran on ip-10-84-111-242.us-east-2.compute.internal from 09:33:14Z to 09:56:23Z; Kubernetes marked the Job Succeeded at 09:56:26Z.

The candidate is spike30: 2^27 = 134,217,728 candidate x-coordinates on each of the three curves, represented by coordinate bit positions 0..25 and 30. Every family base is reused unchanged for m=2..9. All 384 shards (128 per family) are marked verified_all_candidates=true, and all 384 manifest records join the S3 file index by relative path, SHA-256 and compressed byte length. The final index contains 786 content-addressed snapshot entries totaling 4,511,793,211 listed bytes. The complete compressed point data remain at the source S3 prefix; this capture restores the index and structural manifests, not all 4.5 GB of shard payloads.

## File layout

- s3-file-index.json: complete 786-entry object index. Each entry binds a relative path, compressed size, SHA-256 and immutable S3 snapshot key.
- restored/data/index.json: dataset-level contract; candidate id, coordinate map, curve families, point counts, compressed byte totals and same-base m reuse.
- restored/data/{koblitz,binary,prime}/manifest.json: per-family curve/encoding/policy description and 128 shard records, including candidate coverage and content hashes.
- restored/FORMAT.json and restored/RUN_OUTPUT_FORMAT.json: binary shard and output schemas.
- restored/FACTOR_BASE_VARIANT_SWEEP_DESIGN.md: candidate mapping, exhaustive scope and point selection policy.
- restored/run-manifest.json and restored/manifest.sha256: producer completion and generated-file integrity record.
- validation-summary.json: exact join and scope checks recomputed from the restored JSON records.
- status-tool.json and logs-tool.json: terminal Kubernetes record and bounded producer logs.

The point payloads are immutable and remain under:
s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r30-20261031-r1/attempt-01/snapshots/

This is factor-base enumeration and custody evidence only. No candidate PDP FFD/LFD labels, certificates, or model results are included.
