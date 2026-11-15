# r36 attempt-01 partial factor-base enumeration

Job `n83-fb-variants-r36-20261031-r1` (UID `e97b3bf9-a613-4645-a07e-4c9e19a1cba3`) ended Failed with Kubernetes condition `BackoffLimitExceeded` at 2026-11-01T07:10:45Z. The worker pod and its logs were no longer available to the EKS status/log endpoints; the underlying producer exit code and cause are therefore unknown. Do not treat this as a completed factor base.

The complete paginated S3 listing captured after failure contains 790 objects: six staged v9 source objects and 784 immutable objects under `attempt-01/snapshots/`. No final `s3-file-index` or `upload-history.jsonl` object was present, so the snapshot inventory cannot be mapped back to every logical local filename. Nothing under this failed prefix has been overwritten or deleted. The documented expected layout is included as `RUN_OUTPUT_FORMAT.json`; the raw object metadata is in `s3-object-listing.json`, with lossless transport chunks in `listing-chunks/`.

A retry must use a new frozen attempt binding and unique S3 prefix. Preserve this attempt and its partial data as-is.
