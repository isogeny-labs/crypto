# r=27 attempt-04 scheduler recheck

Captured at 2026-10-31T07:45:00Z. This is a pre-start observation of the exact submitted Kubernetes Job. It records the scheduler response, result-prefix listing, bounded container log response, and a concise cluster-capacity context.

## Tree

- `status-tool.json`: exact status API response, including Job/Pod UIDs and unschedulable reason.
- `results-listing.json`: exact S3 object listing returned for the Job prefix.
- `container.log`: log payload returned by the bounded logs API (empty while container has not started).
- `scheduler-context.json`: pool/capacity summary and active or pending Job identities from cluster overview.
- `DATA_INDEX.json`: per-file SHA-256, byte length, and evidence role.
- `FORMAT.json`: package schema and interpretation limits.
- `manifest.sha256`: checksums for all evidence payload and description files above, excluding the index and manifest themselves.

This package documents a live pending observation only. It is not a terminal receipt and does not establish factor-base generation.
